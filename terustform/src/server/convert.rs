use crate::{
    framework::{
        datasource::{self, Mode},
        Diagnostics,
    },
    values::Type,
};

use super::grpc::tfplugin6;

impl datasource::Schema {
    pub(crate) fn to_tfplugin(self) -> tfplugin6::Schema {
        tfplugin6::Schema {
            version: 1,
            block: Some(tfplugin6::schema::Block {
                version: 0,
                attributes: self
                    .attributes
                    .into_iter()
                    .map(|(name, attr)| attr.to_tfplugin(name))
                    .collect(),
                block_types: vec![],
                description: self.description,
                description_kind: tfplugin6::StringKind::Markdown as _,
                deprecated: false,
            }),
        }
    }
}

impl datasource::Attribute {
    pub(crate) fn to_tfplugin(self, name: String) -> tfplugin6::schema::Attribute {
        let mut attr = tfplugin6::schema::Attribute {
            name,
            r#type: vec![],
            nested_type: None,
            description: "<placeholder, this is a bug in terustform>".to_owned(),
            required: false,
            optional: false,
            computed: true,
            sensitive: false,
            description_kind: tfplugin6::StringKind::Markdown as _,
            deprecated: false,
        };

        let set_modes = |attr: &mut tfplugin6::schema::Attribute, mode: Mode| {
            attr.required = mode.required();
            attr.optional = mode.optional();
            attr.computed = mode.computed();
        };

        match self {
            datasource::Attribute::String {
                description,
                mode,
                sensitive,
            } => {
                attr.r#type = Type::String.to_json().into_bytes();
                attr.description = description;
                set_modes(&mut attr, mode);
                attr.sensitive = sensitive;
            }
            datasource::Attribute::Int64 {
                description,
                mode,
                sensitive,
            } => {
                attr.r#type = Type::Number.to_json().into_bytes();
                attr.description = description;
                set_modes(&mut attr, mode);
                attr.sensitive = sensitive;
            }
        }

        attr
    }
}

impl Diagnostics {
    pub(crate) fn to_tfplugin_diags(self) -> Vec<tfplugin6::Diagnostic> {
        self.errors
            .into_iter()
            .map(|err| tfplugin6::Diagnostic {
                severity: tfplugin6::diagnostic::Severity::Error as _,
                summary: err,
                detail: "".to_owned(),
                attribute: None,
            })
            .collect()
    }
}
