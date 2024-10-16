use darling::{FromDeriveInput, ToTokens};
use proc_macro2::{Span, TokenStream};
use quote::{quote, TokenStreamExt};

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(attributes(es_event))]
pub struct EsEntity {
    ident: syn::Ident,
    #[darling(default, rename = "events")]
    events_ident: Option<syn::Ident>,
    #[darling(default)]
    events_field: Option<syn::Ident>,
}

pub fn derive(ast: syn::DeriveInput) -> proc_macro2::TokenStream {
    let entity = match EsEntity::from_derive_input(&ast) {
        Ok(val) => val,
        Err(err) => {
            return err.write_errors();
        }
    };

    quote!(#entity)
}

impl ToTokens for EsEntity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let events_field = self
            .events_field
            .clone()
            .unwrap_or_else(|| syn::Ident::new("events", Span::call_site()));
        let events = self.events_ident.clone().unwrap_or_else(|| {
            syn::Ident::new(
                &format!("{}Event", self.ident),
                proc_macro2::Span::call_site(),
            )
        });

        tokens.append_all(quote! {
            impl EsEntity<#events> for #ident {
                fn events_mut(&mut self) -> &mut EntityEvents<#events> {
                    &mut self.#events_field
                }
            }
        });
    }
}
