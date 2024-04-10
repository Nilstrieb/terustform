pub trait DataSource {
    fn schema(&self);
    fn read(&self) -> DResult<()>;
}

pub struct Diagnostics {
    
}

pub type DResult<T> = Result<T, Diagnostics>;

fn _data_source_obj_safe(_: &dyn DataSource) {}
