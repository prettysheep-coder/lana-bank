use darling::ToTokens;
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};

use super::options::*;

pub struct PersistEventsFn<'a> {
    id: &'a syn::Ident,
    event: &'a syn::Ident,
    error: &'a syn::Type,
    events_table_name: &'a str,
}

impl<'a> From<&'a RepositoryOptions> for PersistEventsFn<'a> {
    fn from(opts: &'a RepositoryOptions) -> Self {
        Self {
            id: opts.id(),
            event: opts.event(),
            error: opts.err(),
            events_table_name: opts.events_table_name(),
        }
    }
}

impl<'a> ToTokens for PersistEventsFn<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let query = format!(
            "INSERT INTO {} (id, recorded_at, sequence, event_type, event) SELECT unnested.id, $1, unnested.sequence, unnested.event_type, unnested.event FROM UNNEST($2::UUID[], $3::INT[], $4::TEXT[], $5::JSONB[]) AS unnested(id, sequence, event_type, event)",
            self.events_table_name,
        );
        let id_type = &self.id;
        let event_type = &self.event;
        let error = self.error;
        let id_tokens = quote! {
            &all_ids as &[#id_type]
        };

        tokens.append_all(quote! {
            fn extract_concurrent_modification<T>(res: Result<T, sqlx::Error>) -> Result<T, #error> {
                match res {
                    Ok(entity) => Ok(entity),
                    Err(sqlx::Error::Database(db_error)) if db_error.is_unique_violation() => {
                        Err(#error::from(es_entity::EsEntityError::ConcurrentModification))
                    }
                    Err(err) => Err(#error::from(err)),
                }
            }

            async fn persist_events(
                &self,
                op: &mut es_entity::DbOp<'_>,
                events_iter: impl Iterator<Item = &mut es_entity::EntityEvents<#event_type>>,
            ) -> Result<usize, #error> {
                let mut all_serialized = Vec::new();
                let mut all_types = Vec::new();
                let mut all_ids = Vec::new();
                let mut all_offsets = Vec::new();
                let now = op.now();

                let mut all_events = Vec::new();
                for events in events_iter {
                    let id = events.id();
                    let offset = events.len_persisted();
                    let serialized = events.serialize_new_events();
                    let types = serialized.iter()
                        .map(|e| e.get("type")
                            .and_then(es_entity::prelude::serde_json::Value::as_str)
                            .expect("Could not read event type")
                            .to_owned())
                        .collect::<Vec<_>>();

                    let n_events = serialized.len();
                    all_serialized.extend(serialized);
                    all_types.extend(types);
                    all_ids.extend(std::iter::repeat(id).take(n_events));
                    all_offsets.extend((offset..).take(n_events).map(|i| i as i32));
                    all_events.push(events);
                }

                let rows = Self::extract_concurrent_modification(
                    sqlx::query!(
                        #query,
                        now,
                        #id_tokens,
                        &all_offsets,
                        &all_types,
                        &all_serialized,
                    ).fetch_all(&mut **op.tx()).await)?;

                let total_events = all_serialized.len();
                for events in all_events {
                    events.mark_new_events_persisted_at(now);
                }

                Ok(total_events)
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persist_events_fn() {
        let id = syn::parse_str("EntityId").unwrap();
        let event = syn::Ident::new("EntityEvent", proc_macro2::Span::call_site());
        let error = syn::parse_str("es_entity::EsRepoError").unwrap();
        let persist_fn = PersistEventsFn {
            id: &id,
            event: &event,
            error: &error,
            events_table_name: "entity_events",
        };

        let mut tokens = TokenStream::new();
        persist_fn.to_tokens(&mut tokens);

        let expected = quote! {
            fn extract_concurrent_modification<T>(res: Result<T, sqlx::Error>) -> Result<T, es_entity::EsRepoError> {
                match res {
                    Ok(entity) => Ok(entity),
                    Err(sqlx::Error::Database(db_error)) if db_error.is_unique_violation() => {
                        Err(es_entity::EsRepoError::from(es_entity::EsEntityError::ConcurrentModification))
                    }
                    Err(err) => Err(es_entity::EsRepoError::from(err)),
                }
            }

            async fn persist_events(
                &self,
                op: &mut es_entity::DbOp<'_>,
                events_iter: impl Iterator<Item = &mut es_entity::EntityEvents<EntityEvent>>,
            ) -> Result<usize, es_entity::EsRepoError> {
                let mut all_serialized = Vec::new();
                let mut all_types = Vec::new();
                let mut all_ids = Vec::new();
                let mut all_offsets = Vec::new();
                let now = op.now();

                let mut all_events = Vec::new();
                for events in events_iter {
                    let id = events.id();
                    let offset = events.len_persisted();
                    let serialized = events.serialize_new_events();
                    let types = serialized.iter()
                        .map(|e| e.get("type")
                            .and_then(es_entity::prelude::serde_json::Value::as_str)
                            .expect("Could not read event type")
                            .to_owned())
                        .collect::<Vec<_>>();

                    let n_events = serialized.len();
                    all_serialized.extend(serialized);
                    all_types.extend(types);
                    all_ids.extend(std::iter::repeat(id).take(n_events));
                    all_offsets.extend((offset..).take(n_events).map(|i| i as i32));
                    all_events.push(events);
                }

                let rows = Self::extract_concurrent_modification(
                    sqlx::query!(
                        "INSERT INTO entity_events (id, recorded_at, sequence, event_type, event) SELECT unnested.id, $1, unnested.sequence, unnested.event_type, unnested.event FROM UNNEST($2::UUID[], $3::INT[], $4::TEXT[], $5::JSONB[]) AS unnested(id, sequence, event_type, event)",
                         now,
                         &all_ids as &[EntityId],
                         &all_offsets,
                         &all_types,
                         &all_serialized,
                    ).fetch_all(&mut **op.tx()).await
                )?;

                let total_events = all_serialized.len();
                for events in all_events {
                    events.mark_new_events_persisted_at(now);
                }

                Ok(total_events)
            }
        };

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}
