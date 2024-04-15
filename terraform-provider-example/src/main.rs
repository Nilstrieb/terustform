use std::collections::{BTreeMap, HashMap};

use terustform::{
    framework::{
        datasource::{self, DataSource},
        provider::Provider,
        AttrPath, DResult, Diagnostics, StringValue, ValueModel,
    },
    values::{Value, ValueKind},
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    terustform::serve(&ExampleProvider {}).await
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

    fn read(&self, config: Value) -> DResult<Value> {
        let model = ExampleDataSourceModel::from_value(config, &AttrPath::root())?;

        let StringValue::Known(name_str) = &model.name else {
            return Err(Diagnostics::error_string(
                "model name must be known".to_owned(),
            ));
        };
        let meow = format!("mrrrrr i am {name_str}");

        Ok(Value::Known(ValueKind::Object(BTreeMap::from([
            ("name".to_owned(), model.name.to_value()),
            (
                "meow".to_owned(),
                Value::Known(ValueKind::String(meow)),
            ),
            (
                "id".to_owned(),
                Value::Known(ValueKind::String("0".to_owned())),
            ),
        ]))))
    }
}

#[derive(terustform::DataSourceModel)]
struct ExampleDataSourceModel {
    name: StringValue,
    meow: StringValue,
    id: StringValue,
}
