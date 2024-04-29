use std::future::Future;

use crate::{
    provider::{BoxFut, MkResource, ProviderData},
    values::Value,
    Schema,
};

use super::DResult;

pub trait Resource: Sized + Send + Sync + 'static {
    type ProviderData: ProviderData;

    // todo: probably want some kind of Value+Schema thing like tfsdk? whatever.
    fn read(&self, config: Value) -> impl Future<Output = DResult<Value>> + Send + Sync;
    fn create(&self, config: Value) -> impl Future<Output = DResult<Value>> + Send + Sync;
    fn update(&self, config: Value) -> impl Future<Output = DResult<Value>> + Send + Sync;
    fn delete(&self, state: Value) -> impl Future<Output = DResult<Value>> + Send + Sync;

    fn name(provider_name: &str) -> String;
    fn schema() -> Schema;
    fn new(data: Self::ProviderData) -> DResult<Self>;

    fn erase() -> MkResource<Self::ProviderData> {
        MkResource::create::<Self>()
    }
}

pub(crate) trait DynResource: Send + Sync + 'static {
    fn read(&self, config: Value) -> BoxFut<'_, DResult<Value>>;
    fn create(&self, config: Value) -> BoxFut<'_, DResult<Value>>;
    fn update(&self, config: Value) -> BoxFut<'_, DResult<Value>>;
    fn delete(&self, config: Value) -> BoxFut<'_, DResult<Value>>;
}

impl<R: Resource> DynResource for R {
    fn read(&self, config: Value) -> BoxFut<'_, DResult<Value>> {
        Box::pin(Resource::read(self, config))
    }
    fn create(&self, config: Value) -> BoxFut<'_, DResult<Value>> {
        Box::pin(Resource::create(self, config))
    }
    fn update(&self, config: Value) -> BoxFut<'_, DResult<Value>> {
        Box::pin(Resource::update(self, config))
    }
    fn delete(&self, state: Value) -> BoxFut<'_, DResult<Value>> {
        Box::pin(Resource::create(self, state))
    }
}
