use proc_macro::{self, TokenStream};

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ItemIdentifier)]
pub fn item_identifier(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input as DeriveInput);

    let output = quote! {
        impl ItemIdentifier for #ident {
            const IDENTIFIER: &'static str = stringify!(#ident);
        }
    };

    output.into()
}
