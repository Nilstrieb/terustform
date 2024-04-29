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
    fn read(&self, current_state: Value) -> impl Future<Output = DResult<Value>> + Send + Sync;
    fn create(
        &self,
        config: Value,
        plan: Value,
    ) -> impl Future<Output = DResult<Value>> + Send + Sync;
    fn update(
        &self,
        config: Value,
        plan: Value,
        state: Value,
    ) -> impl Future<Output = DResult<Value>> + Send + Sync;
    fn delete(&self, state: Value) -> impl Future<Output = DResult<Value>> + Send + Sync;

    fn name(provider_name: &str) -> String;
    fn schema() -> Schema;
    fn new(data: Self::ProviderData) -> DResult<Self>;

    fn erase() -> MkResource<Self::ProviderData> {
        MkResource::create::<Self>()
    }
}

pub(crate) trait DynResource: Send + Sync + 'static {
    fn read(&self, current_state: Value) -> BoxFut<'_, DResult<Value>>;
    fn create(&self, config: Value, plan: Value) -> BoxFut<'_, DResult<Value>>;
    fn update(&self, config: Value, plan: Value, state: Value) -> BoxFut<'_, DResult<Value>>;
    fn delete(&self, state: Value) -> BoxFut<'_, DResult<Value>>;
}

impl<R: Resource> DynResource for R {
    fn read(&self, current_state: Value) -> BoxFut<'_, DResult<Value>> {
        Box::pin(Resource::read(self, current_state))
    }
    fn create(&self, config: Value, plan: Value) -> BoxFut<'_, DResult<Value>> {
        Box::pin(Resource::create(self, config, plan))
    }
    fn update(&self, config: Value, plan: Value, state: Value) -> BoxFut<'_, DResult<Value>> {
        Box::pin(Resource::update(self, config, plan, state))
    }
    fn delete(&self, state: Value) -> BoxFut<'_, DResult<Value>> {
        Box::pin(Resource::delete(self, state))
    }
}
