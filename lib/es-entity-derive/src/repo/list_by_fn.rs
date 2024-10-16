use convert_case::{Case, Casing};
use darling::ToTokens;
use proc_macro2::{Span, TokenStream};
use quote::{quote, TokenStreamExt};

use super::options::*;

pub struct CursorStruct<'a> {
    id: &'a syn::Ident,
    entity: &'a syn::Ident,
    column_name: syn::Ident,
    column_type: syn::Type,
}

impl<'a> ToTokens for CursorStruct<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let struct_ident = syn::Ident::new(
            &format!(
                "{}By{}Cursor",
                self.entity,
                self.column_name.to_string().to_case(Case::UpperCamel)
            ),
            Span::call_site(),
        );
        let id = &self.id;

        let field = if &self.column_name.to_string() != "id" {
            let column_name = &self.column_name;
            let column_type = &self.column_type;
            quote! {
                pub #column_name: #column_type,
            }
        } else {
            quote! {}
        };

        tokens.append_all(quote! {
            #[derive(serde::Serialize, serde::Deserialize)]
            pub struct #struct_ident {
                pub id: #id,
                #field
            }
        });
    }
}

pub struct ListByFn<'a> {
    id: &'a syn::Ident,
    entity: &'a syn::Ident,
    column_name: syn::Ident,
    column_type: &'a syn::Type,
    table_name: &'a str,
    events_table_name: &'a str,
    error: &'a syn::Ident,
}

impl<'a> ListByFn<'a> {
    pub fn new(
        column_name: syn::Ident,
        column_type: &'a syn::Type,
        opts: &'a RepositoryOptions,
    ) -> Self {
        Self {
            column_name,
            column_type,
            id: opts.id(),
            entity: opts.entity(),
            table_name: opts.table_name(),
            events_table_name: opts.events_table_name(),
            error: opts.err(),
        }
    }
}

impl<'a> ToTokens for ListByFn<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let entity = self.entity;
        let column_name = &self.column_name;
        let column_type = self.column_type;
        let error = self.error;

        let fn_name = syn::Ident::new(&format!("find_by_{}", column_name), Span::call_site());
        let fn_via = syn::Ident::new(&format!("find_by_{}_via", column_name), Span::call_site());
        let fn_in_tx =
            syn::Ident::new(&format!("find_by_{}_in_tx", column_name), Span::call_site());

        let query = format!(
            r#"SELECT i.id AS "id: {}", e.sequence, e.event, i.created_at AS entity_created_at, e.recorded_at AS event_recorded_at FROM {} i JOIN {} e ON i.id = e.id WHERE i.{} = $1 ORDER BY e.sequence"#,
            self.id, self.table_name, self.events_table_name, column_name
        );

        tokens.append_all(quote! {
            pub async fn #fn_name(
                &self,
                #column_name: #column_type
            ) -> Result<#entity, #error> {
                self.#fn_via(self.pool(), #column_name).await
            }

            pub async fn #fn_in_tx(
                &self,
                db: &mut sqlx::Transaction<'_, sqlx::Postgres>,
                #column_name: #column_type
            ) -> Result<#entity, #error> {
                self.#fn_via(&mut **db, #column_name).await
            }

            async fn #fn_via(
                &self,
                executor: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
                #column_name: #column_type
            ) -> Result<#entity, #error> {
                let rows = sqlx::query!(
                    #query,
                    #column_name as #column_type,
                )
                    .fetch_all(executor)
                    .await?;
                Ok(EntityEvents::load_first(rows.into_iter().map(|r|
                    GenericEvent {
                        id: r.id,
                        sequence: r.sequence,
                        event: r.event,
                        entity_created_at: r.entity_created_at,
                        event_recorded_at: r.event_recorded_at,
                }))?)
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
    fn cursor_struct_by_id() {
        let id_type = Ident::new("EntityId", Span::call_site());
        let entity = Ident::new("Entity", Span::call_site());

        let cursor = CursorStruct {
            column_name: Ident::new("id", Span::call_site()),
            column_type: syn::parse_str(&id_type.to_string()).unwrap(),
            id: &id_type,
            entity: &entity,
        };

        let mut tokens = TokenStream::new();
        cursor.to_tokens(&mut tokens);

        let expected = quote! {
            #[derive(serde::Serialize, serde::Deserialize)]
            pub struct EntityByIdCursor {
                pub id: EntityId,
            }
        };

        assert_eq!(tokens.to_string(), expected.to_string());
    }

    #[test]
    fn cursor_struct_by_created_at() {
        let id_type = Ident::new("EntityId", Span::call_site());
        let entity = Ident::new("Entity", Span::call_site());
        let column_type = syn::parse_str("DateTime<Utc>").unwrap();

        let cursor = CursorStruct {
            column_name: Ident::new("created_at", Span::call_site()),
            column_type,
            id: &id_type,
            entity: &entity,
        };

        let mut tokens = TokenStream::new();
        cursor.to_tokens(&mut tokens);

        let expected = quote! {
            #[derive(serde::Serialize, serde::Deserialize)]
            pub struct EntityByCreatedAtCursor {
                pub id: EntityId,
                pub created_at: DateTime<Utc>,
            }
        };

        assert_eq!(tokens.to_string(), expected.to_string());
    }

    fn test_list_by_fn() {
        let id_type = Ident::new("EntityId", Span::call_site());
        let column_type = syn::parse_str("EntityId").unwrap();
        let entity = Ident::new("Entity", Span::call_site());
        let error = Ident::new("EsRepoError", Span::call_site());

        let persist_fn = ListByFn {
            column_name: Ident::new("id", Span::call_site()),
            column_type: &column_type,
            id: &id_type,
            entity: &entity,
            table_name: "entities",
            events_table_name: "entity_events",
            error: &error,
        };

        let mut tokens = TokenStream::new();
        persist_fn.to_tokens(&mut tokens);

        let expected = quote! {
            // async fn list_by_id(
            //     &self,
            //     query: es_entity::query::PaginatedQueryArgs<EntityByIdCursor>,
            // ) -> Result<es_entity::query::PaginatedQueryRet<Entity, EntityByIdCursor>, EsRepoError> {
            //     let rows = sqlx::query!(
            //         "WITH entities AS (SELECT i.id AS \"id: EntityId\" FROM entities i WHERE id > $1 ORDER BY id LIMIT $2) SELECT i.id AS \"id: EntityId\", e.sequence, e.event, i.created_at AS entity_created_at, e.recorded_at AS event_recorded_at FROM entities i JOIN entity_events e ON i.id = e.id ORDER BY i.id, e.sequence",
            //       query.after.as_ref().map(|c| c.id) as Option<CustomerId>,
            //       query.after.map(|c| c.name),
            //       query.first as i64 + 1
            //     )
            //         .fetch_all(executor)
            //         .await?;
            //     Ok(EntityEvents::load_n(rows.into_iter().map(|r|
            //         GenericEvent {
            //             id: r.id,
            //             sequence: r.sequence,
            //             event: r.event,
            //             entity_created_at: r.entity_created_at,
            //             event_recorded_at: r.event_recorded_at,
            //     }))?)
            // }
        };

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}
