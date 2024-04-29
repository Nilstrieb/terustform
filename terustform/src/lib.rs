// Internal modules
mod server;

// Modules re-exported in the root
mod diag;
mod schema;
mod values;

// Public modules
pub mod datasource;
pub mod provider;
pub mod resource;

// Re-exports
pub use diag::*;
pub use schema::*;
pub use values::*;

pub use terustform_macros::Model;

pub use async_trait::async_trait;
pub use eyre;

// --------
// Rest of the file.

use provider::Provider;
use tracing::Level;

pub async fn start<P: Provider>(provider: P) -> eyre::Result<()> {
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
    pub use crate::{
        AttrPath, AttrPathSegment, BaseValue, DResult, Diagnostic, Diagnostics, Value, ValueKind,
        ValueModel,
    };
    pub use {Clone, Option::Some, Result::Err, ToOwned};

    pub fn new_object<const N: usize>(elems: [(&str, Value); N]) -> Value {
        Value::Known(ValueKind::Object(std::collections::BTreeMap::from_iter(
            elems.into_iter().map(|(k, v)| (k.to_owned(), v)),
        )))
    }
}
