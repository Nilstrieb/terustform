use std::collections::HashMap;

use tokio_util::sync::CancellationToken;

use crate::{datasource::DataSource, provider::Provider};

use super::{grpc::tfplugin6, Schemas};

pub struct ProviderHandler {
    pub(super) shutdown: CancellationToken,
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

    pub(super) fn get_schemas(&self) -> Schemas {
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

    pub(super) async fn do_read_data_source(
        &self,
        type_name: &str,
        config: &Option<tfplugin6::DynamicValue>,
    ) -> (Option<tfplugin6::DynamicValue>, Vec<tfplugin6::Diagnostic>) {
        let ds = self
            .state
            .as_ref()
            .unwrap()
            .data_sources
            .get(type_name)
            .unwrap();

        let typ = ds.schema().typ();
        let config = match config {
            None => crate::values::Value::Null,
            Some(v) => {
                let value = crate::values::Value::msg_unpack(&v.msgpack, &typ);
                match value {
                    Ok(value) => value,
                    Err(errs) => {
                        return (None, errs.to_tfplugin_diags());
                    }
                }
            }
        };

        let state = ds.read(config).await;
        let (state, diagnostics) = match state {
            Ok(s) => (
                Some(tfplugin6::DynamicValue {
                    msgpack: s.msg_pack(),
                    json: vec![],
                }),
                vec![],
            ),
            Err(errs) => (None, errs.to_tfplugin_diags()),
        };

        (state, diagnostics)
    }
}
