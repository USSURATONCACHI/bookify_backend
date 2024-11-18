mod derive_typed_field;
mod proc_rusmarc_docs;
mod util;

#[proc_macro_derive(TypedField)]
pub fn derive_typed_field(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_typed_field::derive_typed_field(input)
}

#[proc_macro]
pub fn rusmarc_docs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_rusmarc_docs::rusmarc_docs(input)
}
