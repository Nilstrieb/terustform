use quote::quote;
use syn::spanned::Spanned;

// This macro should only reference items in `terustform::__derive_private`.

#[proc_macro_derive(DataSourceModel)]
pub fn data_source_model(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    match data_source_model_inner(input) {
        Ok(ts) => ts.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

fn data_source_model_inner(
    input: syn::DeriveInput,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let struct_name = input.ident;

    let syn::Data::Struct(data) = input.data else {
        return Err(syn::Error::new(
            struct_name.span(),
            "models must be structs",
        ));
    };
    let syn::Fields::Named(fields) = data.fields else {
        return Err(syn::Error::new(
            struct_name.span(),
            "models must have named fields",
        ));
    };

    let tf = quote!(::terustform::__derive_private);

    let fields = fields
        .named
        .into_iter()
        .map(|field| {
            let Some(name) = field.ident else {
                return Err(syn::Error::new(field.span(), "field must be named"));
            };

            Ok((name, field.ty))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let field_extractions = fields.iter().map(|(name, ty)| {
        let name_str = proc_macro2::Literal::string(&name.to_string());
        quote! {
            let #tf::Some(#name) = obj.remove(#name_str) else {
                return #tf::Err(
                    #tf::Diagnostics::error_string(
                        format!("Expected property '{}', which was not present", #name_str),
                    ).with_path(path.clone())
                );
            };
            let #name = <#ty as #tf::ValueModel>::from_value(
                #name,
                &path.append_attribute_name(#tf::ToOwned::to_owned(#name_str))
            )?;
        }
    });
    let constructor_fields = fields.iter().map(|(name, _)| quote! { #name, });

    let to_value_fields = fields.iter().map(|(name, ty)| {
        let name_str = proc_macro2::Literal::string(&name.to_string());

        quote! { (#name_str, <#ty as #tf::ValueModel>::to_value(self.#name)), }
    });

    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics #tf::ValueModel
            for #struct_name #type_generics #where_clause
        {
            fn from_value(v: #tf::Value, path: &#tf::AttrPath) -> #tf::DResult<Self> {
                match v {
                    #tf::BaseValue::Unknown => {
                        return #tf::Err(#tf::Diagnostics::with_path(
                            #tf::Diagnostics::error_string(#tf::ToOwned::to_owned("Expected object, found unknown value")),
                            #tf::Clone::clone(&path),
                        ));
                    },
                    #tf::BaseValue::Null => {
                        return #tf::Err(#tf::Diagnostics::with_path(
                            #tf::Diagnostics::error_string(#tf::ToOwned::to_owned("Expected object, found null value")),
                            #tf::Clone::clone(&path),
                        ));
                    },
                    #tf::BaseValue::Known(#tf::ValueKind::Object(mut obj)) => {
                        #(#field_extractions)*

                        Ok(#struct_name {
                            #(#constructor_fields)*
                        })
                    },
                    #tf::BaseValue::Known(v) => {
                        return #tf::Err(#tf::Diagnostics::with_path(
                            #tf::Diagnostics::error_string(format!("Expected object, found {} value", v.diagnostic_type_str())),
                            #tf::Clone::clone(&path),
                        ));
                    },
                }
            }

            fn to_value(self) -> #tf::Value {
                #tf::new_object(
                    [
                        #(#to_value_fields)*
                    ]
                )
            }
        }
    })
}
