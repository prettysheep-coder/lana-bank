use darling::ToTokens;
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};

use super::options::*;

pub struct PersistFn<'a> {
    id: &'a syn::Ident,
    entity: &'a syn::Ident,
    table_name: &'a str,
    indexes: &'a Indexes,
    error: &'a syn::Ident,
}

impl<'a> From<&'a RepositoryOptions> for PersistFn<'a> {
    fn from(opts: &'a RepositoryOptions) -> Self {
        Self {
            id: opts.id(),
            entity: opts.entity(),
            error: opts.err(),
            indexes: &opts.indexes,
            table_name: opts.table_name(),
        }
    }
}

impl<'a> ToTokens for PersistFn<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let entity = self.entity;
        let error = self.error;

        let update_tokens = if !self.indexes.columns.is_empty() {
            let index_tokens = self.indexes.columns.iter().map(|column| {
                let ident = &column.name;
                quote! {
                    let #ident = &entity.#ident;
                }
            });
            let column_updates = self
                .indexes
                .columns
                .iter()
                .enumerate()
                .map(|(idx, column)| format!("{} = ${}", column.name.to_string(), idx + 2))
                .collect::<Vec<_>>()
                .join(", ");
            let query = format!(
                "UPDATE {} SET {} WHERE id = $1",
                self.table_name, column_updates,
            );
            let args = self.indexes.query_args();
            let id = &self.id;
            Some(quote! {
            let id = &entity.id;
            #(#index_tokens)*
            sqlx::query!(
                #query,
                id as &#id,
                #(#args),*
            )
                .execute(&mut **db)
                .await?;
            })
        } else {
            None
        };

        tokens.append_all(quote! {
            #[inline(always)]
            fn extract_events<T, E>(entity: &mut T) -> &mut es_entity::EntityEvents<E>
            where
                T: es_entity::EsEntity<E>,
                E: es_entity::EsEvent,
            {
                entity.events()
            }

            pub async fn persist_in_tx(
                &self,
                db: &mut sqlx::Transaction<'_, sqlx::Postgres>,
                entity: &mut #entity
            ) -> Result<(), #error> {
                #update_tokens
                self.persist_events(db, Self::extract_events(entity)).await?;
                Ok(())
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
    fn test_persist_fn() {
        let id = Ident::new("EntityId", Span::call_site());
        let entity = Ident::new("Entity", Span::call_site());
        let error = Ident::new("EsRepoError", Span::call_site());

        let indexes = Indexes {
            columns: vec![IndexColumn {
                name: Ident::new("name", Span::call_site()),
                ty: None,
            }],
        };

        let persist_fn = PersistFn {
            entity: &entity,
            table_name: "entities",
            id: &id,
            error: &error,
            indexes: &indexes,
        };

        let mut tokens = TokenStream::new();
        persist_fn.to_tokens(&mut tokens);

        let expected = quote! {
            #[inline(always)]
            fn extract_events<T, E>(entity: &mut T) -> &mut es_entity::EntityEvents<E>
            where
                T: es_entity::EsEntity<E>,
                E: es_entity::EsEvent,
            {
                entity.events()
            }

            pub async fn persist_in_tx(
                &self,
                db: &mut sqlx::Transaction<'_, sqlx::Postgres>,
                entity: &mut Entity
            ) -> Result<(), EsRepoError> {
                let id = &entity.id;
                let name = &entity.name;
                sqlx::query!(
                    "UPDATE entities SET name = $2 WHERE id = $1",
                    id as &EntityId,
                    name
                )
                    .execute(&mut **db)
                    .await?;
                self.persist_events(db, Self::extract_events(entity)).await?;
                Ok(())
            }
        };

        assert_eq!(tokens.to_string(), expected.to_string());
    }

    #[test]
    fn test_persist_fn_no_indexes() {
        let id = Ident::new("EntityId", Span::call_site());
        let entity = Ident::new("Entity", Span::call_site());
        let error = Ident::new("EsRepoError", Span::call_site());

        let indexes = Indexes { columns: vec![] };

        let persist_fn = PersistFn {
            entity: &entity,
            table_name: "entities",
            id: &id,
            error: &error,
            indexes: &indexes,
        };

        let mut tokens = TokenStream::new();
        persist_fn.to_tokens(&mut tokens);

        let expected = quote! {
            #[inline(always)]
            fn extract_events<T, E>(entity: &mut T) -> &mut es_entity::EntityEvents<E>
            where
                T: es_entity::EsEntity<E>,
                E: es_entity::EsEvent,
            {
                entity.events()
            }

            pub async fn persist_in_tx(
                &self,
                db: &mut sqlx::Transaction<'_, sqlx::Postgres>,
                entity: &mut Entity
            ) -> Result<(), EsRepoError> {
                self.persist_events(db, Self::extract_events(entity)).await?;
                Ok(())
            }
        };

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}
