use std::collections::HashMap;

use crate::{
    provider::{MkDataSource, ProviderData},
    values::{Type, Value},
};

use super::DResult;

#[crate::async_trait]
pub trait DataSource: Send + Sync + 'static {
    type ProviderData: ProviderData;

    // todo: probably want some kind of Value+Schema thing like tfsdk? whatever.
    async fn read(&self, config: Value) -> DResult<Value>;

    fn name(provider_name: &str) -> String
    where
        Self: Sized;
    fn schema() -> Schema
    where
        Self: Sized;
    fn new(data: Self::ProviderData) -> DResult<Self>
    where
        Self: Sized;

    fn erase() -> MkDataSource<Self::ProviderData>
    where
        Self: Sized,
    {
        MkDataSource::create::<Self>()
    }
}

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
