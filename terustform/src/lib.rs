mod base;
pub mod datasource;
pub mod provider;
mod server;
pub mod values;

pub use base::*;
pub use terustform_macros::Model;

use provider::Provider;

use tracing::Level;

pub async fn start(provider: &dyn Provider) -> eyre::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_writer(std::io::stderr)
        .without_time()
        .init();

    server::serve(provider).await
}

/// Private, only for use for with the derive macro.
#[doc(hidden)]
pub mod __derive_private {
    pub use crate::values::{Value, ValueKind};
    pub use crate::{
        AttrPath, AttrPathSegment, BaseValue, DResult, Diagnostic, Diagnostics, ValueModel,
    };
    pub use {Clone, Option::Some, Result::Err, ToOwned};

    pub fn new_object<const N: usize>(elems: [(&str, Value); N]) -> Value {
        Value::Known(ValueKind::Object(std::collections::BTreeMap::from_iter(
            elems.into_iter().map(|(k, v)| (k.to_owned(), v)),
        )))
    }
}
