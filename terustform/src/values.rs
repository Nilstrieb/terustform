// check terraform-plugin-go tfprotov6/internal/toproto for convesions
//                           tftypes                    for types and values

use std::{
    collections::{BTreeMap, HashMap},
    io::{self, Read},
};

use crate::{BaseValue, DResult, Diagnostic};

#[derive(Debug)]
pub enum Type {
    Bool,
    Number,
    String,
    Dynamic,
    /// A list of elements of the same type.
    List {
        elem: Box<Type>,
    },
    /// A bunch of unordered string-value pair of the same type.
    Map {
        elem: Box<Type>,
    },
    /// A set of unique values of the same type.
    Set {
        elem: Box<Type>,
    },
    /// A bunch of unordered string-value pairs of different types.
    /// The attributes are statically known.
    Object {
        attrs: HashMap<String, Type>,
        /// The attributes in `attrs` that are optional.
        /// Always empty for now because of JSON reasons.
        optionals: Vec<String>,
    },
    /// An ordered list of values of different types.
    Tuple {
        elems: Vec<Type>,
    },
}

impl Type {
    pub fn to_json(&self) -> String {
        let value = self.to_json_inner();
        serde_json::to_string(&value).unwrap()
    }
    pub fn to_json_inner(&self) -> serde_json::Value {
        use serde_json::Value;

        let compound =
            |tag: &str, inner: Value| Value::Array(vec![Value::String(tag.to_owned()), inner]);

        match self {
            Self::Bool => Value::String("bool".to_owned()),
            Self::String => Value::String("string".to_owned()),
            Self::Number => Value::String("number".to_owned()),
            Self::Dynamic => Value::String("dynamic".to_owned()),
            Self::List { elem } => compound("list", elem.to_json_inner()),
            Self::Map { elem } => compound("map", elem.to_json_inner()),
            Self::Set { elem } => compound("set", elem.to_json_inner()),
            Self::Object {
                attrs,
                optionals: _,
            } => compound(
                "object",
                Value::Object(
                    attrs
                        .iter()
                        .map(|(k, v)| (k.clone(), v.to_json_inner()))
                        .collect(),
                ),
            ),
            Self::Tuple { elems } => compound(
                "tuple",
                elems.iter().map(|elem| elem.to_json_inner()).collect(),
            ),
        }
    }
}

pub type Value = BaseValue<ValueKind>;

#[derive(Debug)]
pub enum ValueKind {
    String(String),
    Number(f64),
    Bool(bool),
    List(Vec<Value>),
    Set(Vec<Value>),
    Map(BTreeMap<String, Value>),
    Tuple(Vec<Value>),
    Object(BTreeMap<String, Value>),
}

impl ValueKind {
    pub fn diagnostic_type_str(&self) -> &'static str {
        match self {
            ValueKind::String(_) => "string",
            ValueKind::Number(_) => "number",
            ValueKind::Bool(_) => "bool",
            ValueKind::List(_) => "list",
            ValueKind::Set(_) => "set",
            ValueKind::Map(_) => "map",
            ValueKind::Tuple(_) => "tuple",
            ValueKind::Object(_) => "object",
        }
    }
}

// marshal msg pack
// tftypes/value.go:MarshalMsgPack

impl Value {
    pub fn msg_pack(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.msg_pack_inner(&mut buf)
            .expect("writing to Vec<u8> cannot fail");
        buf
    }

    pub fn msg_pack_inner(&self, wr: &mut Vec<u8>) -> std::io::Result<()> {
        use rmp::encode as mp;

        let known = match self {
            Value::Unknown => {
                wr.extend_from_slice(&[0xd4, 0, 0]);
                return Ok(());
            }
            Value::Null => {
                mp::write_nil(wr)?;
                return Ok(());
            }
            Value::Known(known) => known,
        };

        match known {
            ValueKind::String(s) => {
                mp::write_str(wr, s)?;
            }
            &ValueKind::Number(n) => {
                if n.is_infinite() {
                    if n.signum() == -1.0 {
                        mp::write_f64(wr, f64::NEG_INFINITY)?;
                    } else {
                        mp::write_f64(wr, f64::INFINITY)?;
                    }
                } else if (n as i64 as f64) == n {
                    // is int
                    mp::write_i64(wr, n as i64)?;
                } else {
                    mp::write_f64(wr, n)?;
                }
                // Terraform handles bigfloats but we do emphatically not care
            }
            ValueKind::Bool(b) => {
                mp::write_bool(wr, *b)?;
            }
            ValueKind::List(elems) | ValueKind::Set(elems) | ValueKind::Tuple(elems) => {
                mp::write_array_len(wr, elems.len().try_into().unwrap())?;
                for elem in elems {
                    elem.msg_pack_inner(wr)?;
                }
            }
            ValueKind::Map(o) | ValueKind::Object(o) => {
                mp::write_map_len(wr, o.len().try_into().unwrap())?;
                for (key, val) in o {
                    mp::write_str(wr, key)?;
                    val.msg_pack_inner(wr)?;
                }
            }
        }

        Ok(())
    }

    pub fn msg_unpack(data: &[u8], typ: &Type) -> DResult<Self> {
        tracing::debug!(?typ, ?data, "Unpacking message");
        let mut read = io::Cursor::new(data);
        Self::msg_unpack_inner(&mut read, typ)
    }

    fn msg_unpack_inner(rd: &mut io::Cursor<&[u8]>, typ: &Type) -> DResult<Self> {
        use rmp::decode as mp;

        if let Ok(()) = mp::read_nil(rd) {
            return Ok(Value::Null);
        }
        rd.set_position(rd.position() - 1); // revert past the nil

        let read_string = |rd: &mut io::Cursor<&[u8]>| -> DResult<String> {
            let len = std::cmp::min(mp::read_str_len(rd)?, 1024 * 1024); // you're not gonna get more than a 1MB string...
            let mut buf = vec![0; len as usize];
            rd.read_exact(&mut buf)?;
            Ok(String::from_utf8(buf)?)
        };

        let value = match typ {
            Type::Bool => {
                let b = mp::read_bool(rd)?;
                ValueKind::Bool(b)
            }
            Type::Number => {
                let prev = rd.position();
                if let Ok(int) = mp::read_int::<i64, _>(rd) {
                    ValueKind::Number(int as f64)
                } else {
                    rd.set_position(prev);
                    if let Ok(f32) = mp::read_f32(rd) {
                        ValueKind::Number(f32 as f64)
                    } else {
                        rd.set_position(prev);
                        let f64 = mp::read_f64(rd)?;
                        ValueKind::Number(f64)
                    }
                }
            }
            Type::String => ValueKind::String(read_string(rd)?),
            Type::Dynamic => todo!("dynamic"),
            Type::List { elem } => {
                let len = mp::read_array_len(rd)?;

                let elems = (0..len)
                    .map(|_| Value::msg_unpack_inner(rd, &elem))
                    .collect::<Result<Vec<_>, _>>()?;
                ValueKind::List(elems)
            }
            Type::Map { elem } => {
                let len = mp::read_map_len(rd)?;

                let elems = (0..len)
                    .map(|_| -> DResult<_> {
                        let key = read_string(rd)?;
                        let value = Value::msg_unpack_inner(rd, &elem)?;
                        Ok((key, value))
                    })
                    .collect::<DResult<BTreeMap<_, _>>>()?;
                ValueKind::Map(elems)
            }
            Type::Set { elem } => {
                let len = mp::read_array_len(rd)?;

                let elems = (0..len)
                    .map(|_| Value::msg_unpack_inner(rd, &elem))
                    .collect::<Result<Vec<_>, _>>()?;
                ValueKind::Set(elems)
            }
            Type::Object { attrs, optionals } => {
                assert!(optionals.is_empty());
                let len = mp::read_map_len(rd)?;

                if attrs.len() != (len as usize) {
                    return Err(Diagnostic::error_string(format!(
                        "expected {} attrs, found {len} attrs in object",
                        attrs.len()
                    ))
                    .into());
                }
                let elems = (0..len)
                    .map(|_| -> DResult<_> {
                        let key = read_string(rd)?;
                        let typ = attrs.get(&key).ok_or_else(|| {
                            Diagnostic::error_string(format!("unexpected attribute: '{key}'"))
                        })?;
                        let value = Value::msg_unpack_inner(rd, &typ)?;
                        Ok((key, value))
                    })
                    .collect::<DResult<BTreeMap<_, _>>>()?;
                ValueKind::Object(elems)
            }
            Type::Tuple { elems } => {
                let len = mp::read_array_len(rd)?;
                if elems.len() != (len as usize) {
                    return Err(Diagnostic::error_string(format!(
                        "expected {} elems, found {len} elems in tuple",
                        elems.len()
                    ))
                    .into());
                }

                let elems = elems
                    .iter()
                    .map(|typ| Value::msg_unpack_inner(rd, &typ))
                    .collect::<Result<Vec<_>, _>>()?;
                ValueKind::Tuple(elems)
            }
        };

        Ok(Value::Known(value))
    }
}
