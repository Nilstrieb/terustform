use std::collections::HashMap;

use terustform::{
    datasource::DataSource, AttrPath, Attribute, DResult, Mode, Schema, StringValue, Value,
    ValueModel,
};

use crate::client::CorsClient;

pub struct ExampleDataSource {}

#[derive(terustform::Model)]
struct ExampleDataSourceModel {
    name: StringValue,
    meow: StringValue,
    id: StringValue,
}

impl DataSource for ExampleDataSource {
    type ProviderData = CorsClient;

    fn name(provider_name: &str) -> String {
        format!("{provider_name}_kitty")
    }

    fn schema() -> Schema {
        Schema {
            description: "an example".to_owned(),
            attributes: HashMap::from([
                (
                    "name".to_owned(),
                    Attribute::String {
                        description: "a cool name".to_owned(),
                        mode: Mode::Required,
                        sensitive: false,
                    },
                ),
                (
                    "meow".to_owned(),
                    Attribute::String {
                        description: "the meow of the cat".to_owned(),
                        mode: Mode::Computed,
                        sensitive: false,
                    },
                ),
                (
                    "id".to_owned(),
                    Attribute::String {
                        description: "the ID of the meowy cat".to_owned(),
                        mode: Mode::Computed,
                        sensitive: false,
                    },
                ),
            ]),
        }
    }

    fn new(_data: Self::ProviderData) -> DResult<Self> {
        Ok(ExampleDataSource {})
    }

    async fn read(&self, config: Value) -> DResult<Value> {
        let mut model = ExampleDataSourceModel::from_value(config, &AttrPath::root())?;

        let name_str = model.name.expect_known(AttrPath::attr("name"))?;

        let meow = format!("mrrrrr i am {name_str}");

        model.meow = StringValue::Known(meow);
        model.id = StringValue::Known("0".to_owned());

        Ok(model.to_value())
    }
}
