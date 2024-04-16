mod client;
mod resources;

use std::collections::HashMap;

use eyre::Context;
use terustform::{
    datasource::DataSource,
    provider::{MkDataSource, Provider},
    DResult, EyreExt, Schema, Value,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    terustform::start(ExampleProvider {}).await
}

pub struct ExampleProvider {}

impl Provider for ExampleProvider {
    type Data = client::CorsClient;
    fn name(&self) -> String {
        "corsschool".to_owned()
    }

    fn schema(&self) -> Schema {
        Schema {
            description: "uwu".to_owned(),
            attributes: HashMap::new(),
        }
    }

    async fn configure(&self, _config: Value) -> DResult<Self::Data> {
        let username = std::env::var("CORSSCHOOL_USERNAME")
            .wrap_err("CORSSCHOOL_USERNAME environment variable not set")
            .eyre_to_tf()?;
        let password = std::env::var("CORSSCHOOL_PASSWORD")
            .wrap_err("CORSSCHOOL_PASSWORD environment variable not set")
            .eyre_to_tf()?;
        let client = client::CorsClient::new(username, password)
            .await
            .wrap_err("failed to create client")
            .eyre_to_tf()?;
        Ok(client)
    }

    fn data_sources(&self) -> Vec<MkDataSource<Self::Data>> {
        vec![
            resources::kitty::ExampleDataSource::erase(),
            resources::hugo::HugoDataSource::erase(),
        ]
    }
}
