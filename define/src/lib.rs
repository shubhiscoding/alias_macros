use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Token, Ident};
use proc_macro2::TokenStream as TokenStream2;

struct DefineInput {
    alias: Ident,
    replacement: TokenStream2,
}

impl syn::parse::Parse for DefineInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let alias: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let replacement: TokenStream2 = input.parse()?;
        Ok(DefineInput {alias, replacement})
    }
}

#[proc_macro]
pub fn define(input: TokenStream) -> TokenStream {
    let DefineInput { alias, replacement} =
        parse_macro_input!(input as DefineInput);
    let expand = quote! {
        macro_rules! #alias {
            () => { #replacement };
            ($($args:tt)*) => { #replacement!($($args)*) };
        }
    };
    expand.into()
}