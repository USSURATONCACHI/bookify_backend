use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn derive_parse_typed(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match derive_parse_typed_inner(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error().into(),
    }
}

fn derive_parse_typed_inner(input: DeriveInput) -> Result<proc_macro::TokenStream, syn::Error> {
    let name = input.ident;

    let number_token = crate::derive_typed_field::get_struct_field_number(&name)?;

    let expanded = quote! {
        impl ParseTypedField for #name {
            fn parse(data: FieldData) -> Result<Box<dyn AnyTypedField>, ParseTypedFieldError> {
                match data {
                    FieldData::FullLine { text } => Ok(Box::new(#name::new(text))),
                    FieldData::Subfields { .. } => Err(format!("Field {} does not have subfields", #number_token)),
                }
            }
        }
    };

    Ok(proc_macro::TokenStream::from(expanded))
}
