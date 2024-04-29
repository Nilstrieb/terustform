use std::future::Future;

use crate::{
    provider::{BoxFut, MkDataSource, ProviderData},
    values::Value,
    Schema,
};

use super::DResult;

pub trait DataSource: Sized + Send + Sync + 'static {
    type ProviderData: ProviderData;

    // todo: probably want some kind of Value+Schema thing like tfsdk? whatever.
    fn read(&self, config: Value) -> impl Future<Output = DResult<Value>> + Send + Sync;

    fn name(provider_name: &str) -> String;
    fn schema() -> Schema;
    fn new(data: Self::ProviderData) -> DResult<Self>;

    fn erase() -> MkDataSource<Self::ProviderData> {
        MkDataSource::create::<Self>()
    }
}

pub(crate) trait DynDataSource: Send + Sync + 'static {
    fn read(&self, config: Value) -> BoxFut<'_, DResult<Value>>;
}

impl<Ds: DataSource> DynDataSource for Ds {
    fn read(&self, config: Value) -> BoxFut<'_, DResult<Value>> {
        Box::pin(DataSource::read(self, config))
    }
}
