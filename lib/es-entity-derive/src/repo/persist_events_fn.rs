use darling::ToTokens;
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};

use super::options::*;

pub struct PersistEventsFn {
    id_type: syn::Ident,
    event_type: syn::Ident,
}

impl<'a> From<&'a RepositoryOptions> for PersistEventsFn {
    fn from(opts: &'a RepositoryOptions) -> Self {
        Self {
            id_type: syn::Ident::new("UserId", proc_macro2::Span::call_site()),
            event_type: syn::Ident::new("UserEvent", proc_macro2::Span::call_site()),
        }
    }
}

impl ToTokens for PersistEventsFn {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let table_name = "user_events";

        let query = format!(
            "INSERT INTO {} (id, sequence, event_type, event) SELECT $1, ROW_NUMBER() OVER () + $2, unnested.event_type, unnested.event FROM UNNEST($3::text[], $4::jsonb[]) AS unnested(event_type, event) RETURNING recorded_at",
            table_name,
        );
        let id_type = &self.id_type;
        let event_type = &self.event_type;
        let id_tokens = quote! {
            id as &#id_type
        };

        tokens.append_all(quote! {
            async fn persist_events(
                &self,
                db: &mut sqlx::Transaction<'_, sqlx::Postgres>,
                events: &mut EntityEvents<#event_type>
            ) -> Result<usize, sqlx::Error> {
                if !events.any_new() {
                    return Ok(0);
                }

                let id = events.id();
                let offset = events.n_persisted();
                let serialized_events = events.serialize_new_events();
                let events_types = serialized_events.iter().map(|e| e.get("type").and_then(serde_json::Value::as_str).expect("Could not read event type").to_owned()).collect::<Vec<_>>();

                let rows = sqlx::query!(
                    #query,
                    #id_tokens,
                    offset as i32,
                    &events_types,
                    &serialized_events
                ).fetch_all(&mut **db).await?;

                Ok(0)
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Span;
    use syn::Ident;

    #[test]
    fn test_create_fn() {
        let id_type = syn::Ident::new("UserId", proc_macro2::Span::call_site());
        let event_type = syn::Ident::new("UserEvent", proc_macro2::Span::call_site());
        let persist_fn = PersistEventsFn {
            id_type,
            event_type,
        };

        let mut tokens = TokenStream::new();
        persist_fn.to_tokens(&mut tokens);

        let expected = quote! {
            async fn persist_events(
                &self,
                db: &mut sqlx::Transaction<'_, sqlx::Postgres>,
                events: &mut EntityEvents<UserEvent>
            ) -> Result<usize, sqlx::Error> {
                if !events.any_new() {
                    return Ok(0);
                }

                let id = events.id();
                let offset = events.n_persisted();
                let serialized_events = events.serialize_new_events();
                let events_types = serialized_events.iter().map(|e| e.get("type").and_then(serde_json::Value::as_str).expect("Could not read event type").to_owned()).collect::<Vec<_>>();

                let rows = sqlx::query!(
                    "INSERT INTO user_events (id, sequence, event_type, event) SELECT $1, ROW_NUMBER() OVER () + $2, unnested.event_type, unnested.event FROM UNNEST($3::text[], $4::jsonb[]) AS unnested(event_type, event) RETURNING recorded_at",
                    id as &UserId,
                    offset as i32,
                    &events_types,
                    &serialized_events
                ).fetch_all(&mut **db).await?;

                Ok(0)
            }
        };

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}
