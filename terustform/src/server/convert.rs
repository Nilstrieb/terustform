use crate::{AttrPath, AttrPathSegment, Attribute, Diagnostics, Mode, Schema, Value};

use super::grpc::tfplugin6;

impl Schema {
    pub(crate) fn into_tfplugin(self) -> tfplugin6::Schema {
        tfplugin6::Schema {
            version: 1,
            block: Some(tfplugin6::schema::Block {
                version: 0,
                attributes: self
                    .attributes
                    .into_iter()
                    .map(|(name, attr)| attr.into_tfplugin(name))
                    .collect(),
                block_types: vec![],
                description: self.description,
                description_kind: tfplugin6::StringKind::Markdown as _,
                deprecated: false,
            }),
        }
    }
}

impl Attribute {
    pub(crate) fn into_tfplugin(self, name: String) -> tfplugin6::schema::Attribute {
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

        attr.r#type = self.typ().to_json().into_bytes();

        match self {
            Attribute::String {
                description,
                mode,
                sensitive,
            } => {
                attr.description = description;
                set_modes(&mut attr, mode);
                attr.sensitive = sensitive;
            }
            Attribute::Int64 {
                description,
                mode,
                sensitive,
            } => {
                attr.description = description;
                set_modes(&mut attr, mode);
                attr.sensitive = sensitive;
            }
            Attribute::Object {
                description,
                mode,
                sensitive,
                attrs: _,
            } => {
                attr.description = description;
                set_modes(&mut attr, mode);
                attr.sensitive = sensitive;
            }
        }

        attr
    }
}

impl Diagnostics {
    pub(crate) fn into_tfplugin_diags(self) -> Vec<tfplugin6::Diagnostic> {
        self.diags
            .into_iter()
            .map(|err| tfplugin6::Diagnostic {
                severity: tfplugin6::diagnostic::Severity::Error as _,
                summary: err.msg,
                detail: "".to_owned(),
                attribute: err.attr.map(|path| path.into_tfplugin()),
            })
            .collect()
    }
}

impl AttrPath {
    pub(crate) fn into_tfplugin(self) -> tfplugin6::AttributePath {
        tfplugin6::AttributePath {
            steps: self
                .0
                .into_iter()
                .map(|segment| {
                    use tfplugin6::attribute_path::step::Selector;

                    tfplugin6::attribute_path::Step {
                        selector: Some(match segment {
                            AttrPathSegment::AttributeName(name) => Selector::AttributeName(name),
                            AttrPathSegment::ElementKeyString(key) => {
                                Selector::ElementKeyString(key)
                            }
                            AttrPathSegment::ElementKeyInt(key) => Selector::ElementKeyInt(key),
                        }),
                    }
                })
                .collect(),
        }
    }
}

impl Value {
    pub(crate) fn into_tfplugin(self) -> Option<tfplugin6::DynamicValue> {
        Some(tfplugin6::DynamicValue {
            msgpack: self.msg_pack(),
            json: vec![],
        })
    }
}
