use crate::{
    provider::{MkResource, ProviderData},
    values::Value,
    Schema,
};

use super::DResult;

#[crate::async_trait]
pub trait Resource: Send + Sync + 'static {
    type ProviderData: ProviderData;

    // todo: probably want some kind of Value+Schema thing like tfsdk? whatever.
    async fn read(&self, config: Value) -> DResult<Value>;
    async fn create(&self, config: Value) -> DResult<Value>;
    async fn update(&self, config: Value) -> DResult<Value>;
    async fn delete(&self, config: Value) -> DResult<Value>;

    fn name(provider_name: &str) -> String
    where
        Self: Sized;
    fn schema() -> Schema
    where
        Self: Sized;
    fn new(data: Self::ProviderData) -> DResult<Self>
    where
        Self: Sized;

    fn erase() -> MkResource<Self::ProviderData>
    where
        Self: Sized,
    {
        MkResource::create::<Self>()
    }
}
