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

    let terustform = quote!(::terustform::__derive_private);

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
            let #terustform::Some(#name) = obj.remove(#name_str) else {
                return #terustform::Err(
                    #terustform::Diagnostics::error_string(
                        format!("Expected property '{}', which was not present", #name_str),
                    ).with_path(path.clone())
                );
            };
            let #name = <#ty as #terustform::ValueModel>::from_value(
                #name,
                &path.append_attribute_name(#terustform::ToOwned::to_owned(#name_str))
            )?;
        }
    });
    let constructor_fields = fields.iter().map(|(name, _)| quote! { #name, });

    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics #terustform::ValueModel
            for #struct_name #type_generics #where_clause
        {
            fn from_value(v: #terustform::Value, path: &#terustform::AttrPath) -> #terustform::DResult<Self> {
                match v {
                    #terustform::BaseValue::Unknown => {
                        return #terustform::Err(#terustform::Diagnostics::with_path(
                            #terustform::Diagnostics::error_string(#terustform::ToOwned::to_owned("Expected object, found unknown value")),
                            #terustform::Clone::clone(&path),
                        ));
                    },
                    #terustform::BaseValue::Null => {
                        return #terustform::Err(#terustform::Diagnostics::with_path(
                            #terustform::Diagnostics::error_string(#terustform::ToOwned::to_owned("Expected object, found null value")),
                            #terustform::Clone::clone(&path),
                        ));
                    },
                    #terustform::BaseValue::Known(#terustform::ValueKind::Object(mut obj)) => {
                        #(#field_extractions)*

                        Ok(#struct_name {
                            #(#constructor_fields)*
                        })
                    },
                    #terustform::BaseValue::Known(v) => {
                        return #terustform::Err(#terustform::Diagnostics::with_path(
                            #terustform::Diagnostics::error_string(format!("Expected object, found {} value", v.diagnostic_type_str())),
                            #terustform::Clone::clone(&path),
                        ));
                    },
                }
            }
        }
    })
}
