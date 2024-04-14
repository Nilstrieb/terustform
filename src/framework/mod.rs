#![allow(dead_code)]

pub mod datasource;
pub mod provider;

use self::datasource::DataSource;

pub struct Diagnostics {
    pub(crate) errors: Vec<String>,
}

pub type DResult<T> = Result<T, Diagnostics>;

impl Diagnostics {
    pub fn error_string(msg: String) -> Self {
        Self {
            errors: vec![msg],
        }
    }
}

impl<E: std::error::Error + std::fmt::Debug> From<E> for Diagnostics {
    fn from(value: E) -> Self {
        Self::error_string(format!("{:?}", value))
    }
}
