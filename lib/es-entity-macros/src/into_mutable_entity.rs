use darling::{FromDeriveInput, ToTokens};
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(attributes(es_event))]
pub struct IntoMutableEntity {
    ident: syn::Ident,
    #[darling(default, rename = "events")]
    events_ident: Option<syn::Ident>,
}

pub fn derive(ast: syn::DeriveInput) -> darling::Result<proc_macro2::TokenStream> {
    let entity = IntoMutableEntity::from_derive_input(&ast)?;
    Ok(quote!(#entity))
}

impl ToTokens for IntoMutableEntity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let events = self.events_ident.clone().unwrap_or_else(|| {
            syn::Ident::new(
                &format!("{}Event", self.ident),
                proc_macro2::Span::call_site(),
            )
        });

        tokens.append_all(quote! {
            impl es_entity::IntoMutableEntity for #ident {
                type Entity = #ident;

                fn to_mutable(self) -> Self::Entity {
                    self
                }
            }

            impl #ident {
                fn clone_events<T, E>(entity: &T) -> es_entity::EntityEvents<E>
                    where T: EsEntity<E>, E: EsEvent + Clone {
                    entity.events().clone()
                }
            }

            impl es_entity::IntoMutableEntity for &#ident {
                type Entity = #ident;

                fn to_mutable(self) -> Self::Entity {
                    <#ident as es_entity::TryFromEvents<#events>>::try_from_events(
                        #ident::clone_events(self)
                    ).expect("Issue making entity mutable")
                }
            }
        });
    }
}
