extern crate proc_macro2;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;
use quote::{quote, ToTokens};
use syn::Ident;
use syn::LitInt;

pub fn rusmarc_docs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match rustmarc_docs_impl(input.into()) {
        Ok(ok) => ok.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn rustmarc_docs_impl(
    input: proc_macro2::TokenStream,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut input = input.into_iter();
    let mut next_tt = || input.next().expect("Unexpected end of input");

    // 1. Parse digits
    let field_number = extract_int_literal(next_tt())?;
    let field_number_value = field_number.base10_parse::<i128>()?;
    let field_name = extract_ident(next_tt())?;
    let struct_name = Ident::new(
        &format!("Field{}{}", field_number_value, field_name),
        field_name.span(),
    );
    let field_description = extract_tokens_line(&mut next_tt)?;
    let field_description = field_description.to_string();

    // 2. Collect field name
    drop(next_tt);
    skip_till_field_start(&mut input);
    let mut fields = Vec::new();
    while let Some(x) = parse_field(&mut input)? {
        fields.push(x);
    }
    let fields = TokenStream::from_iter(fields);
    // let comment_tok;

    let output = quote! {

        #[doc = #field_description]
        pub struct #struct_name {
            #fields
        }
        impl #struct_name {
            fn test() {
                println!("Hello, world!");
            }
        }
    };
    Ok(output.into())
}

fn _get_line(text: &str) -> &str {
    match text.find("\n") {
        Some(offset) => &text[..offset],
        None => text,
    }
}

fn extract_int_literal(tt: TokenTree) -> Result<LitInt, syn::Error> {
    match tt {
        TokenTree::Literal(lit) => {
            // Attempt to parse the literal as a syn::LitInt (integer literal)
            syn::parse::<LitInt>(lit.to_token_stream().into())
        }
        _ => {
            // If the TokenTree isn't a literal (e.g., identifier, punctuation), return an error
            Err(syn::Error::new(tt.span(), "Expected an integer literal"))
        }
    }
}

fn extract_ident(tt: TokenTree) -> Result<syn::Ident, syn::Error> {
    let span = tt.span();
    match tt {
        TokenTree::Ident(ident) => Ok(ident),
        whatever => Err(syn::Error::new(
            span,
            &format!("Expected an identifier, but found: {:?}", whatever),
        )),
    }
}

fn extract_tokens_line(
    token_getter: &mut dyn FnMut() -> TokenTree,
) -> Result<TokenStream, syn::Error> {
    let mut tokens = Vec::new();

    loop {
        let next_token = token_getter();
        tokens.push(next_token);

        if let TokenTree::Punct(p) = tokens.last().unwrap() {
            if p.as_char() == ';' {
                break;
            }
        }
    }

    Ok(TokenStream::from_iter(tokens.into_iter()))
}

fn skip_till_field_start(iter: &mut dyn Iterator<Item = TokenTree>) {
    for tok in iter {
        if let TokenTree::Punct(p) = tok {
            if p.as_char() == '$' {
                break;
            }
        }
    }
}

fn parse_field(
    iter: &mut dyn Iterator<Item = TokenTree>,
) -> Result<Option<TokenStream>, syn::Error> {
    // Parse field name
    let name_tt = iter.next();
    let mut next_tt = || iter.next().expect("Unexpected end of input");

    let name_tt = match name_tt {
        Some(s) => s,
        None => return Ok(None),
    };

    let name = extract_ident(name_tt)?;
    let mut range: Option<(LitInt, LitInt)> = None;

    let mut after_name = next_tt();

    // Check if range is specified
    if let TokenTree::Punct(p) = after_name.clone() {
        match p.as_char() {
            '/' => {
                let range_start = extract_int_literal(next_tt())?;
                let mut range_end = range_start.clone();
                after_name = next_tt();

                if let TokenTree::Punct(arp) = after_name.clone() {
                    if arp.as_char() == '-' {
                        // Expect another int literal
                        range_end = extract_int_literal(next_tt())?;
                        after_name = next_tt();
                    }
                }

                range = Some((range_start, range_end));
            }
            _ => {}
        }
    }

    // Collect field comment
    let _field_comment = collect_till_line_end(after_name, &mut next_tt)?;
    let _ = range; // TODO: Use ranges

    Ok(Some(quote! {
        pub #name: Option<String>,
    }))

    // todo!()
}

fn collect_till_line_end(
    first_tt: TokenTree,
    rest: &mut dyn FnMut() -> TokenTree,
) -> Result<Vec<TokenTree>, syn::Error> {
    let mut tokens = Vec::new();

    if let TokenTree::Punct(p) = first_tt.clone() {
        if p.as_char() == ';' {
            return Ok(tokens);
        }
    }
    tokens.push(first_tt);

    loop {
        let next_token = rest();
        tokens.push(next_token);

        if let TokenTree::Punct(p) = tokens.last().unwrap() {
            if p.as_char() == ';' {
                break;
            }
        }
    }

    Ok(tokens)
}
