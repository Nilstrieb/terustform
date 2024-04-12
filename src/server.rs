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

#[derive(Debug)]
pub struct MyProvider {
    pub shutdown: CancellationToken,
}

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
impl Provider for MyProvider {
    /// GetMetadata returns upfront information about server capabilities and
    /// supported resource types without requiring the server to instantiate all
    /// schema information, which may be memory intensive. This RPC is optional,
    /// where clients may receive an unimplemented RPC error. Clients should
    /// ignore the error and call the GetProviderSchema RPC as a fallback.
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
    async fn get_provider_schema(
        &self,
        request: Request<tfplugin6::get_provider_schema::Request>,
    ) -> Result<Response<tfplugin6::get_provider_schema::Response>, Status> {
        info!("Received get_provider_schema");
        let reply = tfplugin6::get_provider_schema::Response {
            provider: Some(empty_schema()),
            provider_meta: Some(empty_schema()),
            server_capabilities: Some(tfplugin6::ServerCapabilities {
                plan_destroy: true,
                get_provider_schema_optional: true,
                move_resource_state: false,
            }),
            data_source_schemas: HashMap::from([(
                "terustform_kitty".to_owned(),
                tfplugin6::Schema {
                    version: 1,
                    block: Some(tfplugin6::schema::Block {
                        version: 0,
                        attributes: vec![tfplugin6::schema::Attribute {
                            name: "kitten".to_owned(),
                            r#type: Type::String.to_json().into_bytes(),
                            nested_type: None,
                            description: "what sound does the kitten make?".to_owned(),
                            required: false,
                            optional: false,
                            computed: true,
                            sensitive: false,
                            description_kind: 0,
                            deprecated: false,
                        }],
                        block_types: vec![],
                        description: "something or nothing?".to_owned(),
                        description_kind: 0,
                        deprecated: false,
                    }),
                },
            )]),
            resource_schemas: HashMap::from([("terustform_hello".to_owned(), empty_schema())]),
            functions: HashMap::default(),
            diagnostics: vec![],
        };

        Ok(Response::new(reply))
    }
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
    async fn validate_resource_config(
        &self,
        request: Request<tfplugin6::validate_resource_config::Request>,
    ) -> Result<Response<tfplugin6::validate_resource_config::Response>, Status> {
        tracing::info!("validate_resource_config");

        let reply = tfplugin6::validate_resource_config::Response {
            diagnostics: vec![],
        };

        Ok(Response::new(reply))
    }
    async fn validate_data_resource_config(
        &self,
        request: Request<tfplugin6::validate_data_resource_config::Request>,
    ) -> Result<Response<tfplugin6::validate_data_resource_config::Response>, Status> {
        tracing::info!("validate_data_resource_config");

        let reply = tfplugin6::validate_data_resource_config::Response {
            diagnostics: vec![],
        };

        Ok(Response::new(reply))
    }
    async fn upgrade_resource_state(
        &self,
        request: Request<tfplugin6::upgrade_resource_state::Request>,
    ) -> Result<Response<tfplugin6::upgrade_resource_state::Response>, Status> {
        tracing::error!("upgrade_resource_state");
        todo!("upgrade_resource_state")
    }
    /// ////// One-time initialization, called before other functions below
    async fn configure_provider(
        &self,
        request: Request<tfplugin6::configure_provider::Request>,
    ) -> Result<Response<tfplugin6::configure_provider::Response>, Status> {
        tracing::info!("configure_provider");
        let reply = tfplugin6::configure_provider::Response {
            diagnostics: vec![],
        };
        Ok(Response::new(reply))
    }
    /// ////// Managed Resource Lifecycle
    async fn read_resource(
        &self,
        request: Request<tfplugin6::read_resource::Request>,
    ) -> Result<Response<tfplugin6::read_resource::Response>, Status> {
        tracing::error!("read_resource");
        todo!("read_resource")
    }
    async fn plan_resource_change(
        &self,
        request: Request<tfplugin6::plan_resource_change::Request>,
    ) -> Result<Response<tfplugin6::plan_resource_change::Response>, Status> {
        tracing::info!("plan_resource_change");

        let reply = tfplugin6::plan_resource_change::Response {
            planned_state: request.into_inner().proposed_new_state,
            requires_replace: vec![],
            planned_private: vec![],
            diagnostics: vec![],
            legacy_type_system: false,
            deferred: None,
        };

        Ok(Response::new(reply))
    }
    async fn apply_resource_change(
        &self,
        request: Request<tfplugin6::apply_resource_change::Request>,
    ) -> Result<Response<tfplugin6::apply_resource_change::Response>, Status> {
        tracing::info!("apply_resource_change");

        let reply = tfplugin6::apply_resource_change::Response {
            new_state: request.into_inner().planned_state,
            private: vec![],
            diagnostics: vec![],
            legacy_type_system: false,
        };

        Ok(Response::new(reply))
    }
    async fn import_resource_state(
        &self,
        request: Request<tfplugin6::import_resource_state::Request>,
    ) -> Result<Response<tfplugin6::import_resource_state::Response>, Status> {
        tracing::error!("import_resource_state");

        todo!("import_resource_state")
    }
    async fn move_resource_state(
        &self,
        request: Request<tfplugin6::move_resource_state::Request>,
    ) -> Result<Response<tfplugin6::move_resource_state::Response>, Status> {
        tracing::error!("move_resource_state");

        todo!("move_resource_state")
    }
    async fn read_data_source(
        &self,
        request: Request<tfplugin6::read_data_source::Request>,
    ) -> Result<Response<tfplugin6::read_data_source::Response>, Status> {
        tracing::info!("read_data_source");

        let reply = tfplugin6::read_data_source::Response {
            state: Some(tfplugin6::DynamicValue {
                msgpack: crate::values::Value::Object(BTreeMap::from([(
                    "kitten".to_owned(),
                    Box::new(crate::values::Value::String("meow".to_owned())),
                )]))
                .msg_pack(),
                json: vec![],
            }),
            deferred: None,
            diagnostics: vec![],
        };

        dbg!(request);

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

pub struct MyController {
    pub shutdown: CancellationToken,
}

async fn shutdown(token: &CancellationToken) -> ! {
    token.cancel();
    std::future::poll_fn::<(), _>(|_| std::task::Poll::Pending).await;
    unreachable!("we've should have gone to sleep")
}

#[tonic::async_trait]
impl plugin::grpc_controller_server::GrpcController for MyController {
    async fn shutdown(&self, request: Request<plugin::Empty>) -> Result<Response<plugin::Empty>> {
        shutdown(&self.shutdown).await
    }
}
