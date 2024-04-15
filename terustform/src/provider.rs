use std::{future::Future, sync::Arc};

use crate::{
    datasource::{self, DataSource, Schema},
    DResult, Value,
};

pub trait ProviderData: Clone + Send + Sync + 'static {}
impl<D: Clone + Send + Sync + 'static> ProviderData for D {}

pub struct MkDataSource<D: ProviderData> {
    pub(crate) name: fn(&str) -> String,
    pub(crate) schema: datasource::Schema,
    pub(crate) mk: fn(D) -> DResult<StoredDataSource<D>>,
}

pub(crate) struct StoredDataSource<D: ProviderData> {
    pub(crate) ds: Arc<dyn DataSource<ProviderData = D>>,
    pub(crate) schema: datasource::Schema,
}

impl<D: ProviderData> Clone for StoredDataSource<D> {
    fn clone(&self) -> Self {
        Self {
            ds: self.ds.clone(),
            schema: self.schema.clone(),
        }
    }
}

impl<D: ProviderData> MkDataSource<D> {
    pub fn create<Ds: DataSource<ProviderData = D>>() -> Self {
        Self {
            name: Ds::name,
            schema: Ds::schema(),
            mk: |data| {
                Ok(StoredDataSource {
                    ds: Arc::new(Ds::new(data)?),
                    schema: Ds::schema(),
                })
            },
        }
    }
}

pub trait Provider: Send + Sync + Sized + 'static {
    type Data: ProviderData;
    fn name(&self) -> String;
    fn schema(&self) -> Schema;
    fn configure(&self, config: Value) -> impl Future<Output = DResult<Self::Data>> + Send;
    fn data_sources(&self) -> Vec<MkDataSource<Self::Data>>;
}
