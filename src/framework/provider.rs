use super::DataSource;

pub trait Provider: Send + Sync {
    fn name(&self) -> String;
    fn data_sources(&self) -> Vec<Box<dyn DataSource>>;
}
