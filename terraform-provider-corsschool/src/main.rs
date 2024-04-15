use std::collections::HashMap;

use terustform::{
    datasource::{self, DataSource},
    provider::Provider,
    AttrPath, DResult, StringValue, Value, ValueModel,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    terustform::start(&ExampleProvider {}).await
}

pub struct ExampleProvider {}

impl Provider for ExampleProvider {
    fn name(&self) -> String {
        "example".to_owned()
    }

    fn data_sources(&self) -> Vec<Box<dyn DataSource>> {
        vec![ExampleDataSource {}.erase()]
    }
}

struct ExampleDataSource {}

#[derive(terustform::Model)]
struct ExampleDataSourceModel {
    name: StringValue,
    meow: StringValue,
    id: StringValue,
}

#[terustform::async_trait]
impl DataSource for ExampleDataSource {
    fn name(&self, provider_name: &str) -> String {
        format!("{provider_name}_kitty")
    }

    fn schema(&self) -> datasource::Schema {
        datasource::Schema {
            description: "an example".to_owned(),
            attributes: HashMap::from([
                (
                    "name".to_owned(),
                    datasource::Attribute::String {
                        description: "a cool name".to_owned(),
                        mode: datasource::Mode::Required,
                        sensitive: false,
                    },
                ),
                (
                    "meow".to_owned(),
                    datasource::Attribute::String {
                        description: "the meow of the cat".to_owned(),
                        mode: datasource::Mode::Computed,
                        sensitive: false,
                    },
                ),
                (
                    "id".to_owned(),
                    datasource::Attribute::String {
                        description: "the ID of the meowy cat".to_owned(),
                        mode: datasource::Mode::Computed,
                        sensitive: false,
                    },
                ),
            ]),
        }
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
