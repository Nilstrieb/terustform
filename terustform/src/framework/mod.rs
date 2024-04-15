#![allow(dead_code)]

pub mod datasource;
pub mod provider;

use crate::values::{Value, ValueKind};

use self::datasource::DataSource;

#[derive(Debug, Default)]
pub struct Diagnostics {
    pub(crate) errors: Vec<String>,
    pub(crate) attr: Option<AttrPath>,
    // note: lol this cannot contain warnings that would be fucked oops
}

pub type DResult<T> = Result<T, Diagnostics>;

impl Diagnostics {
    pub fn error_string(msg: String) -> Self {
        Self {
            errors: vec![msg],
            attr: None,
        }
    }

    pub fn with_path(mut self, path: AttrPath) -> Self {
        self.attr = Some(path);
        self
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

impl<E: std::error::Error + std::fmt::Debug> From<E> for Diagnostics {
    fn from(value: E) -> Self {
        Self::error_string(format!("{:?}", value))
    }
}

// TODO: this could probably be a clever 0-alloc &-based linked list!

#[derive(Debug, Clone, Default)]
pub struct AttrPath(Vec<AttrPathSegment>);

#[derive(Debug, Clone)]
pub enum AttrPathSegment {
    AttributeName(String),
    ElementKeyString(String),
    ElementKeyInt(i64),
}

impl AttrPath {
    pub fn root() -> Self {
        Self::default()
    }
    pub fn append_attribute_name(&self, name: String) -> Self {
        let mut p = self.clone();
        p.0.push(AttrPathSegment::AttributeName(name));
        p
    }
}

pub type StringValue = BaseValue<String>;
pub type I64Value = BaseValue<i64>;

#[derive(Debug)]
pub enum BaseValue<T> {
    Unknown,
    Null,
    Known(T),
}

impl<T> BaseValue<T> {
    fn map<U>(self, f: impl FnOnce(T) -> U) -> BaseValue<U> {
        self.try_map(|v| Ok(f(v))).unwrap()
    }

    fn try_map<U>(self, f: impl FnOnce(T) -> DResult<U>) -> DResult<BaseValue<U>> {
        Ok(match self {
            Self::Unknown => BaseValue::Unknown,
            Self::Null => BaseValue::Null,
            Self::Known(v) => BaseValue::Known(f(v)?),
        })
    }
}

pub trait ValueModel: Sized {
    fn from_value(v: Value, path: &AttrPath) -> DResult<Self>;

    fn to_value(self) -> Value {
        todo!()
    }
}

impl ValueModel for StringValue {
    fn from_value(v: Value, path: &AttrPath) -> DResult<Self> {
        v.try_map(|v| match v {
            ValueKind::String(s) => Ok(s),
            _ => Err(Diagnostics::error_string(format!(
                "expected string, found {}",
                v.diagnostic_type_str()
            ))
            .with_path(path.clone())),
        })
    }

    fn to_value(self) -> Value {
        self.map(ValueKind::String)
    }
}
