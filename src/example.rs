use std::collections::{BTreeMap, HashMap};

use crate::{
    framework::{
        datasource::{self, DataSource},
        provider::Provider,
        DResult,
    },
    values::{Value, ValueKind},
};

pub struct ExampleProvider {}

impl Provider for ExampleProvider {
    fn name(&self) -> String {
        "terustform".to_owned()
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
        Ok(Value::Known(ValueKind::Object(BTreeMap::from([
            (
                "name".to_owned(),
                match config {
                    Value::Known(ValueKind::Object(mut obj)) => obj.remove("name").unwrap(),
                    _ => unreachable!(),
                },
            ),
            (
                "meow".to_owned(),
                Value::Known(ValueKind::String("mrrrrr".to_owned())),
            ),
            (
                "id".to_owned(),
                Value::Known(ValueKind::String("0".to_owned())),
            ),
        ]))))
    }
}
