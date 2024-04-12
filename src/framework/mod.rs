#![allow(dead_code)]

pub mod datasource;
pub mod provider;

use self::datasource::DataSource;

pub struct Diagnostics {
    pub(crate) errors: Vec<String>,
}

pub type DResult<T> = Result<T, Diagnostics>;
