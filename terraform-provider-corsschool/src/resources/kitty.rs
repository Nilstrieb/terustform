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
    paws: ExampleDataSourceModelPaws,
}

#[derive(terustform::Model)]
struct ExampleDataSourceModelPaws {
    left: StringValue,
    right: StringValue,
}

impl DataSource for ExampleDataSource {
    type ProviderData = CorsClient;

    fn name(provider_name: &str) -> String {
        format!("{provider_name}_kitty")
    }

    fn schema() -> Schema {
        Schema {
            description: "an example".to_owned(),
            attributes: terustform::attrs! {
                "name" => Attribute::String {
                    description: "a cool name".to_owned(),
                    mode: Mode::Required,
                    sensitive: false,
                },
                "meow" => Attribute::String {
                    description: "the meow of the cat".to_owned(),
                    mode: Mode::Computed,
                    sensitive: false,
                },
                "paws" => Attribute::Object {
                    description: "the ID of the meowy cat".to_owned(),
                    mode: Mode::Required,
                    sensitive: false,
                    attrs: terustform::attrs! {
                        "left" => Attribute::String {
                            description: "meow".to_owned(),
                            mode: Mode::Required,
                            sensitive: false,
                        },
                        "right" => Attribute::String {
                            description: "meow".to_owned(),
                            mode: Mode::Required,
                            sensitive: false,
                        },
                    },
                },
            },
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
        model.paws.right = StringValue::Known("O".to_owned());

        Ok(model.to_value())
    }
}
