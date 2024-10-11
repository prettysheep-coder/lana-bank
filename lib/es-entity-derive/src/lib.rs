#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![cfg_attr(feature = "fail-on-warnings", deny(clippy::all))]
mod repo;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_derive(EsEntityRepository, attributes(es_repo))]
pub fn es_entity_repository_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    repo::derive(ast).into()
}
