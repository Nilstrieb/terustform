#![allow(dead_code)]

pub mod datasource;
pub mod provider;

use crate::values::{Value, ValueKind};

use self::datasource::DataSource;

#[derive(Debug, Default)]
pub struct Diagnostics {
    pub(crate) diags: Vec<Diagnostic>,
    // note: lol this cannot contain warnings that would be fucked oops
}

#[derive(Debug)]
pub struct Diagnostic {
    pub(crate) msg: String,
    pub(crate) attr: Option<AttrPath>,
}

pub type DResult<T> = Result<T, Diagnostics>;

impl Diagnostic {
    pub fn error_string(msg: impl Into<String>) -> Self {
        Diagnostic {
            msg: msg.into(),
            attr: None,
        }
    }
    pub fn with_path(mut self, path: AttrPath) -> Self {
        self.attr = Some(path);
        self
    }
}

impl Diagnostics {
    pub fn has_errors(&self) -> bool {
        !self.diags.is_empty()
    }
}

impl<E: std::error::Error + std::fmt::Debug> From<E> for Diagnostic {
    fn from(value: E) -> Self {
        Self::error_string(format!("{:?}", value))
    }
}
impl<E: std::error::Error + std::fmt::Debug> From<E> for Diagnostics {
    fn from(value: E) -> Self {
        Diagnostic::from(value).into()
    }
}

impl From<Diagnostic> for Diagnostics {
    fn from(value: Diagnostic) -> Self {
        Self { diags: vec![value] }
    }
}

// TODO: this could probably be a clever 0-alloc &-based linked list!

#[derive(Debug, Clone, Default)]
pub struct AttrPath(pub(crate) Vec<AttrPathSegment>);

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
    pub fn attr(name: impl Into<String>) -> Self {
        Self(vec![AttrPathSegment::AttributeName(name.into())])
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

    pub fn expect_known(&self, path: AttrPath) -> DResult<&T> {
        match self {
            BaseValue::Null => Err(Diagnostic::error_string("expected value, found null value")
                .with_path(path)
                .into()),
            BaseValue::Unknown => Err(Diagnostic::error_string(
                "expected known value, found unknown value",
            )
            .with_path(path)
            .into()),
            BaseValue::Known(v) => Ok(v),
        }
    }
}

pub trait ValueModel: Sized {
    fn from_value(v: Value, path: &AttrPath) -> DResult<Self>;

    fn to_value(self) -> Value;
}

impl ValueModel for StringValue {
    fn from_value(v: Value, path: &AttrPath) -> DResult<Self> {
        v.try_map(|v| -> DResult<String> {
            match v {
                ValueKind::String(s) => Ok(s),
                _ => Err(Diagnostic::error_string(format!(
                    "expected string, found {}",
                    v.diagnostic_type_str()
                ))
                .with_path(path.clone())
                .into()),
            }
        })
    }

    fn to_value(self) -> Value {
        self.map(ValueKind::String)
    }
}
