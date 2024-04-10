// check terraform-plugin-go tfprotov6/internal/toproto for convesions
//                           tftypes                    for types and values

use std::collections::{BTreeMap, HashMap};

pub enum Type {
    Number,
    String,
}

impl Type {
    pub fn to_json(&self) -> String {
        match *self {
            Self::String => "\"string\"".to_owned(),
            Self::Number => "\"number\"".to_owned(),
        }
    }
}

// this is very dumb and wrong
pub enum Value {
    String(String),
    Object(BTreeMap<String, Box<Value>>)
}

impl Value {
    pub fn ty(&self) -> Type {
        todo!()
    } 
}

// marshal msg pack
// tftypes/value.go:MarshalMsgPack

impl Value {
    pub fn msg_pack(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.msg_pack_inner(&mut buf);
        buf
    }

    pub fn msg_pack_inner(&self, v: &mut Vec<u8>) {
        match self {
            Value::String(s) => {
                rmp::encode::write_str(v, s).unwrap();
            }
            Value::Object(o) => {
                rmp::encode::write_map_len(v, o.len().try_into().unwrap()).unwrap();
                for (key, val) in o {
                    rmp::encode::write_str(v, key).unwrap();
                    val.msg_pack_inner(v);
                }
            }
        }
    }
}