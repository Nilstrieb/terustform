use std::collections::HashMap;

use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

use crate::{
    provider::{MkDataSource, Provider, StoredDataSource},
    DResult, Diagnostic, Diagnostics, Type, Value,
};

use super::{grpc::tfplugin6, Schemas};

pub struct ProviderHandler<P: Provider> {
    pub(super) shutdown: CancellationToken,
    /// Delayed diagnostics reporting in `GetProviderSchema` for better UX.
    state: Mutex<ProviderState<P>>,
}

enum ProviderState<P: Provider> {
    Setup {
        provider: P,
        mk_ds: HashMap<String, MkDataSource<P::Data>>,
    },
    Failed {
        diags: Diagnostics,
    },
    Configured {
        data_sources: HashMap<String, StoredDataSource<P::Data>>,
    },
}

impl<P: Provider> ProviderHandler<P> {
    /// Creates a new `ProviderHandler`.
    /// This function is infallible, as it is not called during a time where reporting errors nicely is possible.
    /// If there's an error, we just taint our internal state and report errors in `GetProviderSchema`.
    pub fn new(shutdown: CancellationToken, provider: P) -> Self {
        let mut mk_ds = HashMap::new();

        let mut errors = Diagnostics::default();
        let name = provider.name();

        for ds in provider.data_sources() {
            let ds_name = (ds.name)(&name);
            let entry = mk_ds.insert(ds_name.clone(), ds);
            if entry.is_some() {
                errors.push(Diagnostic::error_string(format!(
                    "data source {ds_name} exists more than once"
                )));
            }
        }

        let state = if errors.has_errors() {
            ProviderState::Failed { diags: errors }
        } else {
            ProviderState::Setup { provider, mk_ds }
        };
        Self {
            shutdown,
            state: Mutex::new(state),
        }
    }

    pub(super) async fn do_configure_provider(
        &self,
        config: &Option<tfplugin6::DynamicValue>,
    ) -> Vec<tfplugin6::Diagnostic> {
        let mut state = self.state.lock().await;
        let (provider, mk_ds) = match &*state {
            ProviderState::Setup { provider, mk_ds } => (provider, mk_ds),
            ProviderState::Failed { diags } => return diags.clone().to_tfplugin_diags(),
            ProviderState::Configured { .. } => unreachable!("called configure twice"),
        };
        let config = match parse_dynamic_value(config, &provider.schema().typ()) {
            Ok(config) => config,
            Err(errs) => return errs.to_tfplugin_diags(),
        };

        let data = match provider.configure(config).await {
            Ok(data) => data,
            Err(errs) => return errs.to_tfplugin_diags(),
        };

        let mut data_sources = HashMap::new();
        let mut diags = vec![];

        for (ds_name, ds) in mk_ds {
            let ds = (ds.mk)(data.clone());

            match ds {
                Ok(ds) => {
                    data_sources.insert(ds_name.clone(), ds);
                }
                Err(errs) => diags.extend(errs.to_tfplugin_diags()),
            }
        }

        *state = ProviderState::Configured { data_sources };

        diags
    }

    pub(super) async fn get_schemas(&self) -> Schemas {
        let state = self.state.lock().await;
        let resources = HashMap::new();
        let mk_ds = match &*state {
            ProviderState::Setup { mk_ds, provider: _ } => mk_ds,
            ProviderState::Failed { diags } => {
                return Schemas {
                    resources: HashMap::new(),
                    data_sources: HashMap::new(),
                    diagnostics: diags.clone().to_tfplugin_diags(),
                }
            }
            ProviderState::Configured { .. } => {
                unreachable!("called get_schemas after configuration")
            }
        };
        let data_sources = mk_ds
            .iter()
            .map(|(name, ds)| {
                tracing::debug!(?name, "Initializing data source");
                (name.to_owned(), ds.schema.clone().to_tfplugin())
            })
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
        let ds: StoredDataSource<P::Data> = {
            let state = self.state.lock().await;
            match &*state {
                ProviderState::Setup { .. } => {
                    unreachable!("must be set up before calling data sources")
                }
                ProviderState::Failed { diags } => {
                    return (None, diags.clone().to_tfplugin_diags())
                }
                ProviderState::Configured { data_sources } => {
                    data_sources.get(type_name).unwrap().clone()
                }
            }
        };

        let typ = ds.schema.typ();

        let config = match parse_dynamic_value(config, &typ) {
            Ok(value) => value,
            Err(errs) => {
                return (None, errs.to_tfplugin_diags());
            }
        };

        let state = ds.ds.read(config).await;
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

fn parse_dynamic_value(value: &Option<tfplugin6::DynamicValue>, typ: &Type) -> DResult<Value> {
    match value {
        None => Ok(Value::Null),
        Some(v) => Value::msg_unpack(&v.msgpack, typ),
    }
}
