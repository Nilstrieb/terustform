use std::collections::HashMap;

use terustform::{
    resource::Resource, AttrPath, Attribute, DResult, EyreExt, Mode, Schema, Value, ValueModel,
};

use crate::client::CorsClient;

use super::class_data_source::ClassModel;

pub struct ClassResource {
    client: CorsClient,
}

impl Resource for ClassResource {
    type ProviderData = CorsClient;

    async fn read(&self, current_state: Value) -> DResult<Value> {
        let model = ClassModel::from_value(current_state, &AttrPath::root())?;

        let class = self
            .client
            .get_class(model.id.expect_known(AttrPath::attr("id"))?)
            .await
            .eyre_to_tf()?;

        Ok(ClassModel {
            id: model.id,
            name: class.name.into(),
            description: class.description.into(),
            discord_id: class.discord_id.into(),
        }
        .to_value())
    }

    async fn create(&self, _config: Value, plan: Value) -> DResult<Value> {
        let model = ClassModel::from_root_value(plan)?;

        let class = self
            .client
            .post_class(&dto::Class {
                id: Default::default(),
                members: vec![],
                name: model.name.expect_known(AttrPath::attr("name"))?.clone(),
                description: model
                    .description
                    .expect_known(AttrPath::attr("description"))?
                    .clone(),
                discord_id: model
                    .discord_id
                    .expect_known_or_null(AttrPath::attr("discord_id"))?
                    .cloned(),
            })
            .await
            .eyre_to_tf()?;

        Ok(ClassModel {
            id: class.id.to_string().into(),
            name: class.name.into(),
            description: class.description.into(),
            discord_id: class.discord_id.into(),
        }
        .to_value())
    }

    async fn update(&self, _config: Value, _plan: Value, _state: Value) -> DResult<Value> {
        todo!()
    }

    async fn delete(&self, _state: Value) -> DResult<Value> {
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
                        mode: Mode::Required,
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
