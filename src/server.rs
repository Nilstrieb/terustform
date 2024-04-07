pub mod tfplugin6 {
    tonic::include_proto!("tfplugin6");
}

use tfplugin6::provider_server::{Provider, ProviderServer};
use tonic::{transport::Server, Request, Response, Result, Status};

#[derive(Debug, Default)]
pub struct MyProvider;

#[tonic::async_trait]
impl Provider for MyProvider {
    /// GetMetadata returns upfront information about server capabilities and
    /// supported resource types without requiring the server to instantiate all
    /// schema information, which may be memory intensive. This RPC is optional,
    /// where clients may receive an unimplemented RPC error. Clients should
    /// ignore the error and call the GetProviderSchema RPC as a fallback.
    async fn get_metadata(
        &self,
        request: tonic::Request<tfplugin6::get_metadata::Request>,
    ) -> std::result::Result<tonic::Response<tfplugin6::get_metadata::Response>, tonic::Status> {
        todo!()
    }
    /// GetSchema returns schema information for the provider, data resources,
    /// and managed resources.
    async fn get_provider_schema(
        &self,
        request: tonic::Request<tfplugin6::get_provider_schema::Request>,
    ) -> std::result::Result<tonic::Response<tfplugin6::get_provider_schema::Response>, tonic::Status>
    {
        todo!()
    }
    async fn validate_provider_config(
        &self,
        request: tonic::Request<tfplugin6::validate_provider_config::Request>,
    ) -> std::result::Result<
        tonic::Response<tfplugin6::validate_provider_config::Response>,
        tonic::Status,
    > {
        todo!()
    }
    async fn validate_resource_config(
        &self,
        request: tonic::Request<tfplugin6::validate_resource_config::Request>,
    ) -> std::result::Result<
        tonic::Response<tfplugin6::validate_resource_config::Response>,
        tonic::Status,
    > {
        todo!()
    }
    async fn validate_data_resource_config(
        &self,
        request: tonic::Request<tfplugin6::validate_data_resource_config::Request>,
    ) -> std::result::Result<
        tonic::Response<tfplugin6::validate_data_resource_config::Response>,
        tonic::Status,
    > {
        todo!()
    }
    async fn upgrade_resource_state(
        &self,
        request: tonic::Request<tfplugin6::upgrade_resource_state::Request>,
    ) -> std::result::Result<tonic::Response<tfplugin6::upgrade_resource_state::Response>, tonic::Status>
    {
        todo!()
    }
    /// ////// One-time initialization, called before other functions below
    async fn configure_provider(
        &self,
        request: tonic::Request<tfplugin6::configure_provider::Request>,
    ) -> std::result::Result<tonic::Response<tfplugin6::configure_provider::Response>, tonic::Status>
    {
        todo!()
    }
    /// ////// Managed Resource Lifecycle
    async fn read_resource(
        &self,
        request: tonic::Request<tfplugin6::read_resource::Request>,
    ) -> std::result::Result<tonic::Response<tfplugin6::read_resource::Response>, tonic::Status> {
        todo!()
    }
    async fn plan_resource_change(
        &self,
        request: tonic::Request<tfplugin6::plan_resource_change::Request>,
    ) -> std::result::Result<tonic::Response<tfplugin6::plan_resource_change::Response>, tonic::Status>
    {
        todo!()
    }
    async fn apply_resource_change(
        &self,
        request: tonic::Request<tfplugin6::apply_resource_change::Request>,
    ) -> std::result::Result<tonic::Response<tfplugin6::apply_resource_change::Response>, tonic::Status>
    {
        todo!()
    }
    async fn import_resource_state(
        &self,
        request: tonic::Request<tfplugin6::import_resource_state::Request>,
    ) -> std::result::Result<tonic::Response<tfplugin6::import_resource_state::Response>, tonic::Status>
    {
        todo!()
    }
    async fn move_resource_state(
        &self,
        request: tonic::Request<tfplugin6::move_resource_state::Request>,
    ) -> std::result::Result<tonic::Response<tfplugin6::move_resource_state::Response>, tonic::Status>
    {
        todo!()
    }
    async fn read_data_source(
        &self,
        request: tonic::Request<tfplugin6::read_data_source::Request>,
    ) -> std::result::Result<tonic::Response<tfplugin6::read_data_source::Response>, tonic::Status>
    {
        todo!()
    }
    /// GetFunctions returns the definitions of all functions.
    async fn get_functions(
        &self,
        request: tonic::Request<tfplugin6::get_functions::Request>,
    ) -> std::result::Result<tonic::Response<tfplugin6::get_functions::Response>, tonic::Status> {
        todo!()
    }
    /// ////// Provider-contributed Functions
    async fn call_function(
        &self,
        request: tonic::Request<tfplugin6::call_function::Request>,
    ) -> std::result::Result<tonic::Response<tfplugin6::call_function::Response>, tonic::Status> {
        todo!()
    }
    /// ////// Graceful Shutdown
    async fn stop_provider(
        &self,
        request: tonic::Request<tfplugin6::stop_provider::Request>,
    ) -> std::result::Result<tonic::Response<tfplugin6::stop_provider::Response>, tonic::Status> {
        todo!()
    }
}
