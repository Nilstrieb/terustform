use std::{future::Future, sync::Arc};

use crate::{datasource::DataSource, resource::Resource, DResult, Schema, Value};

// This setup is a bit complicated.
// In this explanation, substitute "`Resource`" for "`Resource` or `DataSource`".
// Semantically, we want to store a `HashMap<String, Box<dyn Resource>>`.
// But this doesn't quite work.
// The reason for this is that we want our `dyn Resource`s to be able to store `ProviderData` directly.
// But `ProviderData` is only available after configuration, and we need to know the schema _before_ configuration.
// So we turn the `dyn Resource` into a _statically known_ `MkResource` that contains the constructor and the schema.
// Then after configuration, we invoke the constructor and get our `dyn Resource`.

pub trait ProviderData: Clone + Send + Sync + 'static {}
impl<D: Clone + Send + Sync + 'static> ProviderData for D {}

pub struct MkDataSource<D: ProviderData> {
    pub(crate) name: fn(&str) -> String,
    pub(crate) schema: Schema,
    pub(crate) mk: fn(D) -> DResult<StoredDataSource<D>>,
}

pub(crate) struct StoredDataSource<D: ProviderData> {
    pub(crate) ds: Arc<dyn DataSource<ProviderData = D>>,
    pub(crate) schema: Schema,
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


pub struct MkResource<D: ProviderData> {
    pub(crate) name: fn(&str) -> String,
    pub(crate) schema: Schema,
    pub(crate) mk: fn(D) -> DResult<StoredResource<D>>,
}

pub(crate) struct StoredResource<D: ProviderData> {
    pub(crate) ds: Arc<dyn Resource<ProviderData = D>>,
    pub(crate) schema: Schema,
}

impl<D: ProviderData> Clone for StoredResource<D> {
    fn clone(&self) -> Self {
        Self {
            ds: self.ds.clone(),
            schema: self.schema.clone(),
        }
    }
}

impl<D: ProviderData> MkResource<D> {
    pub fn create<Rs: Resource<ProviderData = D>>() -> Self {
        Self {
            name: Rs::name,
            schema: Rs::schema(),
            mk: |data| {
                Ok(StoredResource {
                    ds: Arc::new(Rs::new(data)?),
                    schema: Rs::schema(),
                })
            },
        }
    }
}

pub type DataSources<P> = Vec<MkDataSource<<P as Provider>::Data>>;
pub type Resources<P> = Vec<MkResource<<P as Provider>::Data>>;

pub trait Provider: Send + Sync + Sized + 'static {
    type Data: ProviderData;
    fn name(&self) -> String;
    fn schema(&self) -> Schema;
    fn configure(&self, config: Value) -> impl Future<Output = DResult<Self::Data>> + Send;
    fn data_sources(&self) -> DataSources<Self>;
    fn resources(&self) -> Resources<Self>;
}
