use darling::{FromDeriveInput, ToTokens};
use proc_macro2::{Span, TokenStream};
use quote::{quote, TokenStreamExt};

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(attributes(es_event))]
pub struct EsEntity {
    ident: syn::Ident,
    #[darling(default, rename = "new")]
    new_entity_ident: Option<syn::Ident>,
    #[darling(default, rename = "event")]
    event_ident: Option<syn::Ident>,
    #[darling(default)]
    events_field: Option<syn::Ident>,
}

pub fn derive(ast: syn::DeriveInput) -> darling::Result<proc_macro2::TokenStream> {
    let entity = EsEntity::from_derive_input(&ast)?;
    Ok(quote!(#entity))
}

impl ToTokens for EsEntity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let events_field = self
            .events_field
            .clone()
            .unwrap_or_else(|| syn::Ident::new("events", Span::call_site()));
        let event = self.event_ident.clone().unwrap_or_else(|| {
            syn::Ident::new(
                &format!("{}Event", self.ident),
                proc_macro2::Span::call_site(),
            )
        });
        let new = self.new_entity_ident.clone().unwrap_or_else(|| {
            syn::Ident::new(
                &format!("New{}", self.ident),
                proc_macro2::Span::call_site(),
            )
        });

        tokens.append_all(quote! {
            impl es_entity::EsEntity for #ident {
                type Event = #event;
                type New = #new;

                fn events_mut(&mut self) -> &mut es_entity::EntityEvents<#event> {
                    &mut self.#events_field
                }
                fn events(&self) -> &es_entity::EntityEvents<#event> {
                    &self.#events_field
                }
            }
        });
    }
}
