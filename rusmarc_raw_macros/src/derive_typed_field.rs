use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn derive_typed_field(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let name_text = name.to_string();

    // Check that word starts correctly
    let must_start_with = "Field";
    if !name_text.starts_with(must_start_with) {
        return syn::Error::new(
            name.span(),
            &format!("Type name must start with `{must_start_with}`"),
        )
        .to_compile_error()
        .into();
    }
    let text = &name_text[must_start_with.len()..];

    // Get a number
    let number = crate::util::get_leading_digits(text);
    if number.len() == 0 {
        return syn::Error::new(
            name.span(),
            &format!("Type name must contain a number after `Field` (e.g. `Field123`)"),
        )
        .to_compile_error()
        .into();
    }
    let number_token = syn::LitInt::new(number, name.span());

    let expanded = quote! {
        impl TypedField for #name {
            fn field_number(&self) -> i128 {
                #number_token
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
