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
use tracing_subscriber::EnvFilter;

// --------
// Rest of the file.

use provider::Provider;

pub async fn start<P: Provider>(provider: P) -> eyre::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder().parse_lossy(
                std::env::var("RUST_LOG")
                    .unwrap_or_else(|_| "h2=info,rustls=info,hyper_util=info,debug".into()),
            ),
        )
        .with_writer(std::io::stderr)
        .without_time()
        .init();

    server::serve(provider).await
}

/// ```rust
/// # use std::collections::HashMap;
/// let x: HashMap<String, u8> = terustform::attrs! {
///     "hello" => 0,
/// };
/// ```
#[macro_export]
macro_rules! attrs {
    (
        $( $name:literal => $rhs:expr ,)*
    ) => {
        <$crate::__derive_private::HashMap<_, _> as $crate::__derive_private::FromIterator<(_, _)>>::from_iter([
            $(
                (
                    $name.into(),
                    $rhs,
                ),
            )*
        ])
    };
}

/// Private, only for use for with the derive macro.
#[doc(hidden)]
pub mod __derive_private {
    pub use crate::{
        AttrPath, AttrPathSegment, BaseValue, DResult, Diagnostic, Diagnostics, Value, ValueKind,
        ValueModel,
    };
    pub use {std::collections::HashMap, Clone, FromIterator, Option::Some, Result::Err, ToOwned};

    pub fn new_object<const N: usize>(elems: [(&str, Value); N]) -> Value {
        Value::Known(ValueKind::Object(std::collections::BTreeMap::from_iter(
            elems.into_iter().map(|(k, v)| (k.to_owned(), v)),
        )))
    }
}
