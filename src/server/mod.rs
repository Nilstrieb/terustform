mod convert;
mod grpc;

use std::collections::HashMap;

use tokio_util::sync::CancellationToken;

use crate::framework::datasource::{self, DataSource};
use crate::framework::provider::Provider;
use crate::framework::DResult;

pub use grpc::plugin::grpc_controller_server::GrpcControllerServer;
pub use grpc::tfplugin6::provider_server::ProviderServer;
pub use grpc::Controller;

use self::grpc::tfplugin6;

pub struct ProviderHandler {
    shutdown: CancellationToken,
    /// Delayed diagnostics reporting in `GetProviderSchema` for better UX.
    state: Result<ProviderState, Vec<tfplugin6::Diagnostic>>,
}

struct ProviderState {
    data_sources: HashMap<String, Box<dyn DataSource>>,
}

impl ProviderHandler {
    /// Creates a new `ProviderHandler`.
    /// This function is infallible, as it is not called during a time where reporting errors nicely is possible.
    /// If there's an error, we just taint our internal state and report errors in `GetProviderSchema`.
    pub fn new(shutdown: CancellationToken, provider: &dyn Provider) -> Self {
        let name = provider.name();
        let mut data_sources = HashMap::new();
        let mut errors = vec![];

        for ds in provider.data_sources() {
            let ds_name = ds.name(&name);
            let entry = data_sources.insert(ds_name.clone(), ds);
            if entry.is_some() {
                errors.push(tfplugin6::Diagnostic {
                    severity: tfplugin6::diagnostic::Severity::Error as _,
                    summary: format!("data source {ds_name} exists more than once"),
                    detail: "".to_owned(),
                    attribute: None,
                });
            }
        }

        let state = if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(ProviderState { data_sources })
        };

        Self { shutdown, state }
    }

    fn get_schemas(&self) -> Schemas {
        let resources = HashMap::new();
        let state = match &self.state {
            Ok(state) => state,
            Err(errors) => {
                return Schemas {
                    resources: HashMap::new(),
                    data_sources: HashMap::new(),
                    diagnostics: errors.clone(),
                }
            }
        };
        let data_sources = state
            .data_sources
            .iter()
            .map(|(name, ds)| (name.to_owned(), ds.schema().to_tfplugin()))
            .collect::<HashMap<String, tfplugin6::Schema>>();

        Schemas {
            resources,
            data_sources,
            diagnostics: vec![],
        }
    }
}

struct Schemas {
    resources: HashMap<String, tfplugin6::Schema>,
    data_sources: HashMap<String, tfplugin6::Schema>,
    diagnostics: Vec<tfplugin6::Diagnostic>,
}
