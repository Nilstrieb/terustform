#[derive(Debug, Default, Clone)]
pub struct Diagnostics {
    pub(crate) diags: Vec<Diagnostic>,
    // note: lol this cannot contain warnings that would be fucked oops
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub(crate) msg: String,
    pub(crate) attr: Option<AttrPath>,
}

pub type DResult<T> = Result<T, Diagnostics>;

// TODO: this could probably be a clever 0-alloc &-based linked list!

#[derive(Debug, Clone, Default)]
pub struct AttrPath(pub(crate) Vec<AttrPathSegment>);

#[derive(Debug, Clone)]
pub enum AttrPathSegment {
    AttributeName(String),
    ElementKeyString(String),
    ElementKeyInt(i64),
}

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
    pub fn push(&mut self, d: Diagnostic) {
        self.diags.push(d);
    }
    pub fn has_errors(&self) -> bool {
        !self.diags.is_empty()
    }
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

pub trait EyreExt<T> {
    fn eyre_to_tf(self) -> DResult<T>;
}

impl<T> EyreExt<T> for Result<T, eyre::Report> {
    fn eyre_to_tf(self) -> DResult<T> {
        self.map_err(|e| Diagnostic::error_string(format!("{:?}", e)).into())
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
