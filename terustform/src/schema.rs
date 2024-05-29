use std::collections::HashMap;

use crate::Type;

#[derive(Clone)]
pub struct Schema {
    pub description: String,
    pub attributes: HashMap<String, Attribute>,
}

#[derive(Clone)]
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
    Object {
        description: String,
        mode: Mode,
        sensitive: bool,
        attrs: HashMap<String, Attribute>,
    },
}

#[derive(Clone, Copy)]
pub enum Mode {
    Required,
    Optional,
    OptionalComputed,
    Computed,
}

impl Attribute {
    pub fn mode(&self) -> Mode {
        match *self {
            Self::Int64 { mode, .. } => mode,
            Self::String { mode, .. } => mode,
            Self::Object { mode, .. } => mode,
        }
    }
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
        attrs_typ(&self.attributes)
    }
}

impl Attribute {
    pub fn typ(&self) -> Type {
        match self {
            Attribute::Int64 { .. } => Type::Number,
            Attribute::String { .. } => Type::String,
            Attribute::Object { attrs, .. } => attrs_typ(attrs),
        }
    }
}

fn attrs_typ(attrs: &HashMap<String, Attribute>) -> Type {
    let attr_tys = attrs
        .iter()
        .map(|(name, attr)| (name.clone(), attr.typ()))
        .collect();

    let optionals = attrs
        .iter()
        .filter_map(|(name, attr)| {
            if attr.mode().optional() {
                Some(name.clone())
            } else {
                None
            }
        })
        .collect();

    Type::Object {
        attrs: attr_tys,
        optionals,
    }
}
