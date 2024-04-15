mod client;

use std::collections::HashMap;

use terustform::{
    datasource::{self, DataSource},
    provider::{MkDataSource, Provider},
    AttrPath, DResult, StringValue, Value, ValueModel,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    terustform::start(ExampleProvider {}).await
}

pub struct ExampleProvider {}

impl Provider for ExampleProvider {
    type Data = ();
    fn name(&self) -> String {
        "corsschool".to_owned()
    }

    fn schema(&self) -> datasource::Schema {
        datasource::Schema {
            description: "uwu".to_owned(),
            attributes: HashMap::new(),
        }
    }

    async fn configure(&self, _config: Value) -> DResult<Self::Data> {
        Ok(())
    }

    fn data_sources(&self) -> Vec<MkDataSource<Self::Data>> {
        vec![ExampleDataSource::erase()]
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
    type ProviderData = ();

    fn name(provider_name: &str) -> String {
        format!("{provider_name}_kitty")
    }

    fn schema() -> datasource::Schema {
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
