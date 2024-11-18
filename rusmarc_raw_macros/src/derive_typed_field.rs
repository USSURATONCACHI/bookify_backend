use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, LitInt};

pub fn derive_typed_field(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match derive_typed_field_inner(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error().into(),
    }
}

fn derive_typed_field_inner(input: DeriveInput) -> Result<proc_macro::TokenStream, syn::Error> {
    let name = input.ident;

    let number_token = get_struct_field_number(&name)?;

    let expanded = quote! {
        impl TypedField for #name {
            fn field_number(&self) -> u128 {
                #number_token
            }
        }
    };

    Ok(proc_macro::TokenStream::from(expanded))
}

pub fn get_struct_field_number(name: &Ident) -> Result<LitInt, syn::Error> {
    let name_text = name.to_string();

    // Check that word starts correctly
    let must_start_with = "Field";
    if !name_text.starts_with(must_start_with) {
        return Err(syn::Error::new(
            name.span(),
            &format!("Type name must start with `{must_start_with}`"),
        ));
    }
    let text = &name_text[must_start_with.len()..];

    // Get a number
    let number = crate::util::get_leading_digits(text);
    if number.len() == 0 {
        return Err(syn::Error::new(
            name.span(),
            &format!("Type name must contain a number after `Field` (e.g. `Field123`)"),
        ));
    }
    let number_token = syn::LitInt::new(number, name.span());

    Ok(number_token)
}
