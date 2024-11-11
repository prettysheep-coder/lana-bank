use darling::{FromDeriveInput, FromField, ToTokens};
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};

#[derive(Debug, FromField)]
#[darling(forward_attrs)]
struct EventsField {
    attrs: Vec<syn::Attribute>,
    ident: Option<syn::Ident>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(supports(struct_named), attributes(es_event))]
pub struct EsEntity {
    ident: syn::Ident,
    #[darling(default, rename = "new")]
    new_entity_ident: Option<syn::Ident>,
    #[darling(default, rename = "event")]
    event_ident: Option<syn::Ident>,
    data: darling::ast::Data<(), EventsField>,
}

impl EsEntity {
    fn find_events_field(&self) -> Option<&EventsField> {
        match &self.data {
            darling::ast::Data::Struct(fields) => fields.iter().find(|field| {
                field
                    .attrs
                    .iter()
                    .any(|attr| attr.path().is_ident("events"))
                    || field.ident.as_ref().map_or(false, |i| i == "events")
            }),
            _ => None,
        }
    }
}

pub fn derive(ast: syn::DeriveInput) -> darling::Result<proc_macro2::TokenStream> {
    let entity = EsEntity::from_derive_input(&ast)?;
    Ok(quote!(#entity))
}

impl ToTokens for EsEntity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let events_field = self
            .find_events_field()
            .expect("Struct must have a field marked with #[events]")
            .ident
            .as_ref()
            .expect("Not ident on #[events]");

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

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn test_derive_es_entity() {
        let input: syn::DeriveInput = parse_quote! {
            #[derive(EsEntity)]
            pub struct User {
                pub id: UserId,
                pub email: String,
                #[events]
                the_events: EntityEvents<UserEvent>
            }
        };

        let output = derive(input).unwrap();
        let expected = quote! {
            impl es_entity::EsEntity for User {
                type Event = UserEvent;
                type New = NewUser;
                fn events_mut(&mut self) -> &mut es_entity::EntityEvents<UserEvent> {
                    &mut self.the_events
                }
                fn events(&self) -> &es_entity::EntityEvents<UserEvent> {
                    &self.the_events
                }
            }
        };

        assert_eq!(output.to_string(), expected.to_string());
    }
    #[test]
    fn test_derive_without_events_attr() {
        let input: syn::DeriveInput = parse_quote! {
            #[derive(EsEntity)]
            pub struct User {
                pub id: UserId,
                events: EntityEvents<UserEvent>
            }
        };

        let output = derive(input).unwrap();
        let expected = quote! {
            impl es_entity::EsEntity for User {
                type Event = UserEvent;
                type New = NewUser;
                fn events_mut(&mut self) -> &mut es_entity::EntityEvents<UserEvent> {
                    &mut self.events
                }
                fn events(&self) -> &es_entity::EntityEvents<UserEvent> {
                    &self.events
                }
            }
        };

        assert_eq!(output.to_string(), expected.to_string());
    }
}
