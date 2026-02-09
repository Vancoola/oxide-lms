use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Load)]
pub fn derive_load_fn(_item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(_item as DeriveInput);

    let name = input.ident;

    todo!()
}