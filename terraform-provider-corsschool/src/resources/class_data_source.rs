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
struct ClassDataSourceModel {
    id: StringValue,
    name: StringValue,
    description: StringValue,
    discord_id: StringValue,
}

#[terustform::async_trait]
impl DataSource for ClassDataSource {
    type ProviderData = CorsClient;

    async fn read(&self, config: Value) -> DResult<Value> {
        let mut model = ClassDataSourceModel::from_value(config, &AttrPath::root())?;

        let class = self
            .client
            .get_class(&model.id.expect_known(AttrPath::attr("id"))?)
            .await
            .wrap_err("failed to get class")
            .eyre_to_tf()?;

        model.name = StringValue::Known(class.name);
        model.description = StringValue::Known(class.description);
        model.discord_id = StringValue::from(class.discord_id);

        Ok(model.to_value())
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
