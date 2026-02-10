use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};


#[proc_macro_attribute]
pub fn secure(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {


        #[cfg(feature = "zeroize")]
        #[derive(crate::zeroize::Zeroize, crate::zeroize::ZeroizeOnDrop)]
        #input

        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Secure {} (***)", stringify!(#name))
            }
        }
    };

    TokenStream::from(expanded)
}