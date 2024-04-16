use crate::{
    provider::{MkDataSource, ProviderData},
    values::Value,
    Schema,
};

use super::DResult;

#[crate::async_trait]
pub trait DataSource: Send + Sync + 'static {
    type ProviderData: ProviderData;

    // todo: probably want some kind of Value+Schema thing like tfsdk? whatever.
    async fn read(&self, config: Value) -> DResult<Value>;

    fn name(provider_name: &str) -> String
    where
        Self: Sized;
    fn schema() -> Schema
    where
        Self: Sized;
    fn new(data: Self::ProviderData) -> DResult<Self>
    where
        Self: Sized;

    fn erase() -> MkDataSource<Self::ProviderData>
    where
        Self: Sized,
    {
        MkDataSource::create::<Self>()
    }
}
