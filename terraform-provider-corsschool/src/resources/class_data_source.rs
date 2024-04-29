use std::collections::HashMap;

use eyre::Context;
use terustform::{
    datasource::DataSource, AttrPath, Attribute, DResult, EyreExt, Mode, Schema, StringValue,
    Value, ValueModel,
};

use crate::client::CorsClient;

pub struct ClassDataSource {
    client: CorsClient,
}

#[derive(terustform::Model)]
pub(super) struct ClassModel {
    pub(super) id: StringValue,
    pub(super) name: StringValue,
    pub(super) description: StringValue,
    pub(super) discord_id: StringValue,
}

impl DataSource for ClassDataSource {
    type ProviderData = CorsClient;

    async fn read(&self, config: Value) -> DResult<Value> {
        let model = ClassModel::from_value(config, &AttrPath::root())?;

        let class = self
            .client
            .get_class(model.id.expect_known(AttrPath::attr("id"))?)
            .await
            .wrap_err("failed to get class")
            .eyre_to_tf()?;

        Ok(ClassModel {
            id: model.id,
            name: class.name.into(),
            description: class.description.into(),
            discord_id: class.discord_id.into(),
        }
        .to_value())
    }

    fn name(provider_name: &str) -> String {
        format!("{provider_name}_class")
    }

    fn schema() -> Schema {
        Schema {
            description: "Get a class by name".to_owned(),
            attributes: HashMap::from([
                (
                    "id".to_owned(),
                    // TODO: UUID validation :3
                    Attribute::String {
                        description: "The UUID".to_owned(),
                        mode: Mode::Required,
                        sensitive: false,
                    },
                ),
                (
                    "name".to_owned(),
                    Attribute::String {
                        description: "The description".to_owned(),
                        mode: Mode::Computed,
                        sensitive: false,
                    },
                ),
                (
                    "description".to_owned(),
                    Attribute::String {
                        description: "The description".to_owned(),
                        mode: Mode::Computed,
                        sensitive: false,
                    },
                ),
                (
                    "discord_id".to_owned(),
                    Attribute::String {
                        description: "The discord ID of the class".to_owned(),
                        mode: Mode::Computed,
                        sensitive: false,
                    },
                ),
            ]),
        }
    }

    fn new(data: Self::ProviderData) -> DResult<Self> {
        Ok(Self { client: data })
    }
}
