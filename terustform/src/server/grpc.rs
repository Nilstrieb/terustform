#![allow(unused_variables, unused_imports)]


pub mod tfplugin6 {
    tonic::include_proto!("tfplugin6");
}

pub mod plugin {
    tonic::include_proto!("plugin");
}

use std::{
    collections::{BTreeMap, HashMap},
    sync::Mutex,
    vec,
};

use tfplugin6::provider_server::{Provider, ProviderServer};
use tokio_util::sync::CancellationToken;
use tonic::{transport::Server, Request, Response, Result, Status};
use tracing::info;

use crate::values::Type;

fn empty_schema() -> tfplugin6::Schema {
    tfplugin6::Schema {
        version: 1,
        block: Some(tfplugin6::schema::Block {
            version: 0,
            attributes: vec![],
            block_types: vec![],
            description: "hello world".to_owned(),
            description_kind: 0,
            deprecated: false,
        }),
    }
}

#[tonic::async_trait]
impl<P: crate::provider::Provider> Provider for super::ProviderHandler<P> {
    /// GetMetadata returns upfront information about server capabilities and
    /// supported resource types without requiring the server to instantiate all
    /// schema information, which may be memory intensive. This RPC is optional,
    /// where clients may receive an unimplemented RPC error. Clients should
    /// ignore the error and call the GetProviderSchema RPC as a fallback.
    /// Returns data source, managed resource, and function metadata, such as names.
    #[tracing::instrument(skip(self, request))]
    async fn get_metadata(
        &self,
        request: Request<tfplugin6::get_metadata::Request>,
    ) -> Result<Response<tfplugin6::get_metadata::Response>, Status> {
        info!("get_metadata");
        Err(Status::unimplemented(
            "GetMetadata: Not implemeneted".to_owned(),
        ))
    }
    /// GetSchema returns schema information for the provider, data resources,
    /// and managed resources.
    /// Returns provider schema, provider metaschema, all resource schemas and all data source schemas.
    #[tracing::instrument(skip(self, request))]
    async fn get_provider_schema(
        &self,
        request: Request<tfplugin6::get_provider_schema::Request>,
    ) -> Result<Response<tfplugin6::get_provider_schema::Response>, Status> {
        info!("get_provider_schema");

        let schemas = self.do_get_provider_schema().await;

        let reply = tfplugin6::get_provider_schema::Response {
            provider: Some(empty_schema()),
            provider_meta: Some(empty_schema()),
            server_capabilities: Some(tfplugin6::ServerCapabilities {
                plan_destroy: true,
                get_provider_schema_optional: true,
                move_resource_state: false,
            }),
            data_source_schemas: schemas.data_sources,
            resource_schemas: schemas.resources,
            functions: HashMap::default(),
            diagnostics: schemas.diagnostics,
        };

        Ok(Response::new(reply))
    }

    /// Validates the practitioner supplied provider configuration by verifying types conform to the schema and supports value validation diagnostics.
    #[tracing::instrument(skip(self, request))]
    async fn validate_provider_config(
        &self,
        request: Request<tfplugin6::validate_provider_config::Request>,
    ) -> Result<Response<tfplugin6::validate_provider_config::Response>, Status> {
        tracing::info!("validate_provider_config");

        let reply = tfplugin6::validate_provider_config::Response {
            diagnostics: vec![],
        };

        Ok(Response::new(reply))
    }

    /// Validates the practitioner supplied resource configuration by verifying types conform to the schema and supports value validation diagnostics.
    #[tracing::instrument(skip(self, request), fields(name = request.get_ref().type_name))]
    async fn validate_resource_config(
        &self,
        request: Request<tfplugin6::validate_resource_config::Request>,
    ) -> Result<Response<tfplugin6::validate_resource_config::Response>, Status> {
        tracing::info!(name=?request.get_ref().type_name, "validate_resource_config");

        // No validators for now.

        let reply = tfplugin6::validate_resource_config::Response {
            diagnostics: vec![],
        };

        Ok(Response::new(reply))
    }

    /// Validates the practitioner supplied data source configuration by verifying types conform to the schema and supports value validation diagnostics.
    #[tracing::instrument(skip(self, request), fields(name = request.get_ref().type_name))]
    async fn validate_data_resource_config(
        &self,
        request: Request<tfplugin6::validate_data_resource_config::Request>,
    ) -> Result<Response<tfplugin6::validate_data_resource_config::Response>, Status> {
        tracing::info!(name=?request.get_ref().type_name, "validate_data_resource_config");

        // No validators for now.

        let reply = tfplugin6::validate_data_resource_config::Response {
            diagnostics: vec![],
        };

        Ok(Response::new(reply))
    }

    /// Called when a resource has existing state. Primarily useful for when the schema version does not match the current version.
    /// The provider is expected to modify the state to upgrade it to the latest schema.
    #[tracing::instrument(skip(self, request), fields(name = request.get_ref().type_name))]
    async fn upgrade_resource_state(
        &self,
        request: Request<tfplugin6::upgrade_resource_state::Request>,
    ) -> Result<Response<tfplugin6::upgrade_resource_state::Response>, Status> {
        tracing::info!(name=?request.get_ref().type_name, "upgrade_resource_state");
        // We don't do anything interesting, it's fine.
        let reply = tfplugin6::upgrade_resource_state::Response {
            upgraded_state: None,
            diagnostics: vec![],
        };

        Ok(Response::new(reply))
    }
    /// ////// One-time initialization, called before other functions below
    /// Passes the practitioner supplied provider configuration to the provider.
    #[tracing::instrument(skip(self, request))]
    async fn configure_provider(
        &self,
        request: Request<tfplugin6::configure_provider::Request>,
    ) -> Result<Response<tfplugin6::configure_provider::Response>, Status> {
        tracing::info!("configure_provider");
        let (_, diagnostics) = self.do_configure_provider(&request.get_ref().config).await;
        let reply = tfplugin6::configure_provider::Response { diagnostics };
        Ok(Response::new(reply))
    }
    /// ////// Managed Resource Lifecycle
    /// Called when refreshing a resource's state.
    #[tracing::instrument(skip(self, request), fields(name = request.get_ref().type_name))]
    async fn read_resource(
        &self,
        request: Request<tfplugin6::read_resource::Request>,
    ) -> Result<Response<tfplugin6::read_resource::Response>, Status> {
        let req = request.get_ref();

        let (new_state, diagnostics) = self
            .do_read_resource(&req.type_name, &req.current_state)
            .await;

        let reply = tfplugin6::read_resource::Response {
            deferred: None,
            diagnostics: vec![],
            new_state,
            private: vec![],
        };

        Ok(Response::new(reply))
    }

    /// Calculates a plan for a resource. A proposed new state is generated, which the provider can modify.
    #[tracing::instrument(skip(self, request), fields(name = request.get_ref().type_name))]
    async fn plan_resource_change(
        &self,
        request: Request<tfplugin6::plan_resource_change::Request>,
    ) -> Result<Response<tfplugin6::plan_resource_change::Response>, Status> {
        tracing::info!(name=?request.get_ref().type_name, "plan_resource_change");
        let req = request.get_ref();

        // We don't do anything interesting like requires_replace for now.
        // We're supposed to handle defaults here.

        let (planned_state, diagnostics) = self
            .do_plan_resource_change(
                &req.type_name,
                &req.prior_state,
                &req.proposed_new_state,
                &req.config,
            )
            .await;
        let reply = tfplugin6::plan_resource_change::Response {
            planned_state,
            requires_replace: vec![],
            planned_private: vec![],
            diagnostics,
            legacy_type_system: false,
            deferred: None,
        };

        Ok(Response::new(reply))
    }

    /// Called when a practitioner has approved a planned change.
    /// The provider is to apply the changes contained in the plan, and return a resulting state matching the given plan.
    #[tracing::instrument(skip(self, request), fields(name = request.get_ref().type_name))]
    async fn apply_resource_change(
        &self,
        request: Request<tfplugin6::apply_resource_change::Request>,
    ) -> Result<Response<tfplugin6::apply_resource_change::Response>, Status> {
        tracing::info!(name=?request.get_ref().type_name, "apply_resource_change");
        let req = request.get_ref();

        let (new_state, diagnostics) = self
            .do_apply_resource_change(
                &req.type_name,
                &req.prior_state,
                &req.planned_state,
                &req.config,
            )
            .await;
        tracing::debug!(?new_state, ?diagnostics, "post apply_resource_change");

        let reply = tfplugin6::apply_resource_change::Response {
            new_state,
            private: vec![],
            diagnostics,
            legacy_type_system: false,
        };

        Ok(Response::new(reply))
    }

    /// Called when importing a resource into state so that the resource becomes managed.
    #[tracing::instrument(skip(self, request), fields(name = request.get_ref().type_name))]
    async fn import_resource_state(
        &self,
        request: Request<tfplugin6::import_resource_state::Request>,
    ) -> Result<Response<tfplugin6::import_resource_state::Response>, Status> {
        tracing::error!(name=?request.get_ref().type_name, "import_resource_state");

        Err(Status::unimplemented("import_resource_state"))
    }

    #[tracing::instrument(skip(self, request), fields(source_name = request.get_ref().source_type_name))]
    async fn move_resource_state(
        &self,
        request: Request<tfplugin6::move_resource_state::Request>,
    ) -> Result<Response<tfplugin6::move_resource_state::Response>, Status> {
        tracing::error!(source_name=?request.get_ref().source_type_name, "move_resource_state");

        Err(Status::unimplemented("move_resource_state"))
    }

    /// Called when refreshing a data source's state.
    #[tracing::instrument(skip(self, request), fields(name = request.get_ref().type_name))]
    async fn read_data_source(
        &self,
        request: Request<tfplugin6::read_data_source::Request>,
    ) -> Result<Response<tfplugin6::read_data_source::Response>, Status> {
        tracing::info!(name=?request.get_ref().type_name, "read_data_source");
        let req = request.get_ref();

        let (state, diagnostics) = self.do_read_data_source(&req.type_name, &req.config).await;

        let reply = tfplugin6::read_data_source::Response {
            state,
            deferred: None,
            diagnostics,
        };

        Ok(Response::new(reply))
    }
    /// GetFunctions returns the definitions of all functions.
    async fn get_functions(
        &self,
        request: Request<tfplugin6::get_functions::Request>,
    ) -> Result<Response<tfplugin6::get_functions::Response>, Status> {
        tracing::error!("get_functions");

        todo!("get_functions")
    }
    /// ////// Provider-contributed Functions
    async fn call_function(
        &self,
        request: Request<tfplugin6::call_function::Request>,
    ) -> Result<Response<tfplugin6::call_function::Response>, Status> {
        tracing::error!("call_function");

        todo!("call_function")
    }
    /// ////// Graceful Shutdown
    async fn stop_provider(
        &self,
        request: Request<tfplugin6::stop_provider::Request>,
    ) -> Result<Response<tfplugin6::stop_provider::Response>, Status> {
        tracing::info!("stop_provider");

        shutdown(&self.shutdown).await
    }
}

pub struct Controller {
    pub shutdown: CancellationToken,
}

async fn shutdown(token: &CancellationToken) -> ! {
    token.cancel();
    std::future::poll_fn::<(), _>(|_| std::task::Poll::Pending).await;
    unreachable!("we've should have gone to sleep")
}

#[tonic::async_trait]
impl plugin::grpc_controller_server::GrpcController for Controller {
    async fn shutdown(&self, request: Request<plugin::Empty>) -> Result<Response<plugin::Empty>> {
        shutdown(&self.shutdown).await
    }
}
