use std::collections::HashMap;

use eyre::Context;
use terustform::{
    datasource::DataSource, Attribute, DResult, EyreExt, Mode, Schema, StringValue, Value,
    ValueModel,
};

use crate::client::CorsClient;

pub struct HugoDataSource {
    client: CorsClient,
}

#[derive(terustform::Model)]
struct HugoDataSourceModel {
    hugo: StringValue,
}

impl DataSource for HugoDataSource {
    type ProviderData = CorsClient;

    async fn read(&self, _config: Value) -> DResult<Value> {
        let hugo = self
            .client
            .get_hugo()
            .await
            .wrap_err("failed to get hugo")
            .eyre_to_tf()?;

        Ok(HugoDataSourceModel {
            hugo: StringValue::Known(hugo),
        }
        .to_value())
    }

    fn name(provider_name: &str) -> String {
        format!("{provider_name}_hugo")
    }

    fn schema() -> Schema {
        Schema {
            description: "Get Hugo Boss".to_owned(),
            attributes: HashMap::from([(
                "hugo".to_owned(),
                Attribute::String {
                    description: "Hugo Boss".to_owned(),
                    mode: Mode::Computed,
                    sensitive: false,
                },
            )]),
        }
    }

    fn new(data: Self::ProviderData) -> DResult<Self> {
        Ok(Self { client: data })
    }
}
