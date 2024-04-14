use std::collections::HashMap;

use crate::values::{Type, Value};

use super::DResult;

pub trait DataSource: Send + Sync {
    fn name(&self, provider_name: &str) -> String;
    fn schema(&self) -> Schema;
    // todo: probably want some kind of Value+Schema thing like tfsdk? whatever.
    fn read(&self, config: Value) -> DResult<Value>;

    fn erase(self) -> Box<dyn DataSource>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

pub struct Schema {
    pub description: String,
    pub attributes: HashMap<String, Attribute>,
}

pub enum Attribute {
    String {
        description: String,
        mode: Mode,
        sensitive: bool,
    },
    Int64 {
        description: String,
        mode: Mode,
        sensitive: bool,
    },
}

#[derive(Clone, Copy)]
pub enum Mode {
    Required,
    Optional,
    OptionalComputed,
    Computed,
}

impl Mode {
    pub fn required(&self) -> bool {
        matches!(self, Self::Required)
    }

    pub fn optional(&self) -> bool {
        matches!(self, Self::Optional | Self::OptionalComputed)
    }

    pub fn computed(&self) -> bool {
        matches!(self, Self::OptionalComputed | Self::Computed)
    }
}

impl Schema {
    pub fn typ(&self) -> Type {
        let attrs = self
            .attributes
            .iter()
            .map(|(name, attr)| {
                let attr_type = match attr {
                    Attribute::Int64 { .. } => Type::Number,
                    Attribute::String { .. } => Type::String,
                };

                (name.clone(), attr_type)
            })
            .collect();

        Type::Object {
            attrs,
            optionals: vec![],
        }
    }
}
