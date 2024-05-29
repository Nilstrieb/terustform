// check terraform-plugin-go tfprotov6/internal/toproto for convesions
//                           tftypes                    for types and values

use std::{
    collections::BTreeMap,
    io::{self, Read},
};

use crate::{AttrPath, DResult, Diagnostic};

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
        attrs: BTreeMap<String, Type>,
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
    // tftypes/type.go
    // https://github.com/hashicorp/terraform-plugin-go/blob/05dc75aefa5b71406022d0ac08eca99f44fbf378/tftypes/type.go#L95
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
            Self::Object { attrs, optionals } => {
                let mut parts = vec![
                    Value::String("object".to_owned()),
                    Value::Object(
                        attrs
                            .iter()
                            .map(|(k, v)| (k.clone(), v.to_json_inner()))
                            .collect(),
                    ),
                ];

                if !optionals.is_empty() {
                    parts.push(Value::Array(
                        optionals.iter().map(|v| Value::String(v.clone())).collect(),
                    ));
                }

                Value::Array(parts)
            }
            Self::Tuple { elems } => compound(
                "tuple",
                elems.iter().map(|elem| elem.to_json_inner()).collect(),
            ),
        }
    }
}

pub type Value = BaseValue<ValueKind>;

#[derive(PartialEq, Debug)]
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

pub type StringValue = BaseValue<String>;
pub type I64Value = BaseValue<i64>;

#[derive(PartialEq, Eq, Debug)]
pub enum BaseValue<T> {
    Unknown,
    Null,
    Known(T),
}

impl<T> BaseValue<T> {
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    fn map<U>(self, f: impl FnOnce(T) -> U) -> BaseValue<U> {
        self.try_map(|v| Ok(f(v))).unwrap()
    }

    fn try_map<U>(self, f: impl FnOnce(T) -> DResult<U>) -> DResult<BaseValue<U>> {
        Ok(match self {
            Self::Unknown => BaseValue::Unknown,
            Self::Null => BaseValue::Null,
            Self::Known(v) => BaseValue::Known(f(v)?),
        })
    }

    pub fn expect_known(&self, path: AttrPath) -> DResult<&T> {
        match self {
            BaseValue::Null => Err(Diagnostic::error_string("expected value, found null value")
                .with_path(path)
                .into()),
            BaseValue::Unknown => Err(Diagnostic::error_string(
                "expected known value, found unknown value",
            )
            .with_path(path)
            .into()),
            BaseValue::Known(v) => Ok(v),
        }
    }

    pub fn expect_known_or_null(&self, path: AttrPath) -> DResult<Option<&T>> {
        match self {
            BaseValue::Null => Ok(None),
            BaseValue::Unknown => Err(Diagnostic::error_string(
                "expected known value, found unknown value",
            )
            .with_path(path)
            .into()),
            BaseValue::Known(v) => Ok(Some(v)),
        }
    }
}

impl<T> From<T> for BaseValue<T> {
    fn from(value: T) -> Self {
        Self::Known(value)
    }
}

impl<T> From<Option<T>> for BaseValue<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => Self::Known(value),
            None => Self::Null,
        }
    }
}

pub trait ValueModel: Sized {
    fn from_value(v: Value, path: &AttrPath) -> DResult<Self>;

    fn to_value(self) -> Value;

    fn from_root_value(v: Value) -> DResult<Self> {
        Self::from_value(v, &AttrPath::root())
    }
}

impl ValueModel for StringValue {
    fn from_value(v: Value, path: &AttrPath) -> DResult<Self> {
        v.try_map(|v| -> DResult<String> {
            match v {
                ValueKind::String(s) => Ok(s),
                _ => Err(Diagnostic::error_string(format!(
                    "expected string, found {}",
                    v.diagnostic_type_str()
                ))
                .with_path(path.clone())
                .into()),
            }
        })
    }

    fn to_value(self) -> Value {
        self.map(ValueKind::String)
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
        Self::msg_unpack_inner(&mut read, typ).map_err(|mut diag| {
            diag.diags[0].msg = format!("msgpack decoding error: {}", diag.diags[0].msg);
            diag
        })
    }

    fn msg_unpack_inner(rd: &mut io::Cursor<&[u8]>, typ: &Type) -> DResult<Self> {
        use rmp::decode as mp;

        let start = rd.position();

        if let Ok(()) = mp::read_nil(rd) {
            return Ok(Value::Null);
        }
        rd.set_position(start);
        // TODO: Handle unknown values better
        // https://github.com/hashicorp/terraform/blob/main/docs/plugin-protocol/object-wire-format.md#schemaattribute-mapping-rules-for-messagepack
        if mp::read_fixext1(rd).is_ok() {
            return Ok(Value::Unknown);
        }
        rd.set_position(start);

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
                    .map(|_| Value::msg_unpack_inner(rd, elem))
                    .collect::<Result<Vec<_>, _>>()?;
                ValueKind::List(elems)
            }
            Type::Map { elem } => {
                let len = mp::read_map_len(rd)?;

                let elems = (0..len)
                    .map(|_| -> DResult<_> {
                        let key = read_string(rd)?;
                        let value = Value::msg_unpack_inner(rd, elem)?;
                        Ok((key, value))
                    })
                    .collect::<DResult<BTreeMap<_, _>>>()?;
                ValueKind::Map(elems)
            }
            Type::Set { elem } => {
                let len = mp::read_array_len(rd)?;

                let elems = (0..len)
                    .map(|_| Value::msg_unpack_inner(rd, elem))
                    .collect::<Result<Vec<_>, _>>()?;
                ValueKind::Set(elems)
            }
            Type::Object { attrs, optionals } => {
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
                        let value = Value::msg_unpack_inner(rd, typ)?;
                        Ok((key, value))
                    })
                    .collect::<DResult<BTreeMap<_, _>>>()?;

                for expected_attr in attrs.keys() {
                    let is_ok = elems.contains_key(expected_attr);
                    if !is_ok && !optionals.contains(expected_attr) {
                        return Err(Diagnostic::error_string(format!(
                            "expected attribute '{expected_attr}', but it was not present"
                        ))
                        .into());
                    }
                }

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
                    .map(|typ| Value::msg_unpack_inner(rd, typ))
                    .collect::<Result<Vec<_>, _>>()?;
                ValueKind::Tuple(elems)
            }
        };

        Ok(Value::Known(value))
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use crate::{Type, Value, ValueKind};

    #[test]
    fn type_json() {
        let typs = [
            (Type::Bool, "\"bool\""),
            (Type::Number, "\"number\""),
            (Type::String, "\"string\""),
            (Type::Dynamic, "\"dynamic\""),
            (
                Type::List {
                    elem: Box::new(Type::String),
                },
                r#"["list","string"]"#,
            ),
            (
                Type::Map {
                    elem: Box::new(Type::String),
                },
                r#"["map","string"]"#,
            ),
            (
                Type::Set {
                    elem: Box::new(Type::String),
                },
                r#"["set","string"]"#,
            ),
            (
                Type::Object {
                    attrs: crate::attrs! {
                        "meow" => Type::String,
                        "mrooow" => Type::String,
                        "uwu" => Type::String,
                    },
                    optionals: vec![],
                },
                r#"["object",{"meow":"string","mrooow":"string","uwu":"string"}]"#,
            ),
            (
                Type::Object {
                    attrs: crate::attrs! {
                        "meow" => Type::String,
                        "mrooow" => Type::String,
                        "uwu" => Type::String,
                    },
                    optionals: vec!["uwu".to_owned()],
                },
                r#"["object",{"meow":"string","mrooow":"string","uwu":"string"},["uwu"]]"#,
            ),
        ];

        for (typ, expected) in typs {
            let actual_str = typ.to_json();
            assert_eq!(actual_str, expected);
        }
    }

    #[test]
    fn decode_object() {
        let typ = Type::Object {
            attrs: BTreeMap::from([
                ("id".into(), Type::String),
                ("discord_id".into(), Type::String),
                ("name".into(), Type::String),
                ("description".into(), Type::String),
            ]),
            optionals: vec![],
        };
        let data = [
            132, 171, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 163, 63, 63, 63, 170,
            100, 105, 115, 99, 111, 114, 100, 95, 105, 100, 192, 162, 105, 100, 212, 0, 0, 164,
            110, 97, 109, 101, 164, 109, 101, 111, 119,
        ];

        let value = Value::msg_unpack(&data, &typ);

        assert_eq!(
            value.unwrap(),
            Value::Known(ValueKind::Object(BTreeMap::from([
                (
                    "description".into(),
                    Value::Known(ValueKind::String("???".into()))
                ),
                ("discord_id".into(), Value::Null),
                ("id".into(), Value::Unknown),
                (
                    "name".into(),
                    Value::Known(ValueKind::String("meow".into()))
                ),
            ])))
        );
    }
}
