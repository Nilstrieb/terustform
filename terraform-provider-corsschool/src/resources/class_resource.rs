use std::collections::HashMap;

use terustform::{resource::Resource, Attribute, DResult, Mode, Schema, Value};

use crate::client::CorsClient;

pub struct ClassResource {
    client: CorsClient,
}

impl Resource for ClassResource {
    type ProviderData = CorsClient;

    async fn read(&self, config: Value) -> DResult<Value> {
        todo!()
    }

    async fn create(&self, config: Value) -> DResult<Value> {
        todo!()
    }

    async fn update(&self, config: Value) -> DResult<Value> {
        todo!()
    }

    async fn delete(&self, state: Value) -> DResult<Value> {
        todo!()
    }

    fn name(provider_name: &str) -> String {
        format!("{provider_name}_class")
    }

    fn schema() -> terustform::Schema {
        Schema {
            description: "A class".into(),
            attributes: HashMap::from([
                (
                    "id".to_owned(),
                    // TODO: UUID validation :3
                    Attribute::String {
                        description: "The UUID".to_owned(),
                        mode: Mode::Computed,
                        sensitive: false,
                    },
                ),
                (
                    "name".to_owned(),
                    Attribute::String {
                        description: "The description".to_owned(),
                        mode: Mode::Required,
                        sensitive: false,
                    },
                ),
                (
                    "description".to_owned(),
                    Attribute::String {
                        description: "The description".to_owned(),
                        mode: Mode::Optional,
                        sensitive: false,
                    },
                ),
                (
                    "discord_id".to_owned(),
                    Attribute::String {
                        description: "The discord ID of the class".to_owned(),
                        mode: Mode::Optional,
                        sensitive: false,
                    },
                ),
            ]),
        }
    }

    fn new(client: Self::ProviderData) -> DResult<Self> {
        Ok(Self { client })
    }
}
