use terustform::{datasource::DataSource, DResult, Value};

use crate::client::CorsClient;

pub struct ClassResource {
    client: CorsClient,
}
