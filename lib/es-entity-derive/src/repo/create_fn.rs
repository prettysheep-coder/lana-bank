use darling::ToTokens;
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};

use super::options::*;

pub struct CreateFn<'a> {
    new_entity: &'a syn::Ident,
    id: &'a syn::Ident,
    entity: &'a syn::Ident,
    table_name: &'a str,
    indexes: &'a Indexes,
    error: &'a syn::Ident,
}

impl<'a> From<&'a RepositoryOptions> for CreateFn<'a> {
    fn from(opts: &'a RepositoryOptions) -> Self {
        Self {
            new_entity: opts.new_entity(),
            table_name: opts.table_name(),
            id: opts.id(),
            entity: opts.entity(),
            error: opts.err(),
            indexes: &opts.indexes,
        }
    }
}

impl<'a> ToTokens for CreateFn<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let new_entity = self.new_entity;
        let id = self.id;
        let entity = self.entity;
        let error = self.error;

        let index_tokens = self.indexes.columns.iter().map(|column| {
            let ident = &column.name;
            quote! {
                let #ident = &new_entity.#ident;
            }
        });

        let table_name = self.table_name;

        let columns_names: Vec<_> = self
            .indexes
            .columns
            .iter()
            .map(|c| c.name.to_string())
            .collect();
        let placeholders = (1..=self.indexes.columns.len() + 1)
            .map(|i| format!("${}", i))
            .collect::<Vec<_>>()
            .join(", ");
        let args = self.indexes.query_args();

        let query = format!(
            "INSERT INTO {} (id, {}) VALUES ({})",
            table_name,
            columns_names.join(", "),
            placeholders,
        );

        tokens.append_all(quote! {
            #[inline(always)]
            fn convert_new<T, E>(item: T) -> es_entity::EntityEvents<E>
            where
                T: es_entity::IntoEvents<E>,
                E: es_entity::EsEvent,
            {
                item.into_events()
            }

            #[inline(always)]
            fn hydrate_entity<T, E>(events: es_entity::EntityEvents<E>) -> Result<T, #error>
            where
                T: es_entity::TryFromEvents<E>,
                #error: From<es_entity::EsEntityError>,
                E: es_entity::EsEvent,
            {
                Ok(T::try_from_events(events)?)
            }

            pub async fn create(
                &self,
                db: &mut sqlx::Transaction<'_, sqlx::Postgres>,
                new_entity: #new_entity
            ) -> Result<#entity, #error> {
                let id = &new_entity.id;
                #(#index_tokens)*

                 sqlx::query!(
                     #query,
                     id as &#id,
                     #(#args),*
                )
                .execute(&mut **db)
                .await?;

                let mut events = Self::convert_new(new_entity);
                let n_events = self.persist_events(&mut **db, &mut events).await?;

                Self::hydrate_entity(events)
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
        let new_entity = Ident::new("NewEntity", Span::call_site());
        let entity = Ident::new("Entity", Span::call_site());
        let error = Ident::new("EsRepoError", Span::call_site());
        let id = Ident::new("EntityId", Span::call_site());

        let indexes = Indexes {
            columns: vec![IndexColumn {
                name: Ident::new("name", Span::call_site()),
                ty: Ident::new("String", Span::call_site()),
            }],
        };

        let create_fn = CreateFn {
            new_entity: &new_entity,
            table_name: "entities",
            id: &id,
            entity: &entity,
            error: &error,
            indexes: &indexes,
        };

        let mut tokens = TokenStream::new();
        create_fn.to_tokens(&mut tokens);

        let expected = quote! {
            #[inline(always)]
            fn convert_new<T, E>(item: T) -> es_entity::EntityEvents<E>
            where
                T: es_entity::IntoEvents<E>,
                E: es_entity::EsEvent,
            {
                item.into_events()
            }

            #[inline(always)]
            fn hydrate_entity<T, E>(events: es_entity::EntityEvents<E>) -> Result<T, EsRepoError>
            where
                T: es_entity::TryFromEvents<E>,
                EsRepoError: From<es_entity::EsEntityError>,
                E: es_entity::EsEvent,
            {
                Ok(T::try_from_events(events)?)
            }

            pub async fn create(
                &self,
                db: &mut sqlx::Transaction<'_, sqlx::Postgres>,
                new_entity: NewEntity
            ) -> Result<Entity, EsRepoError> {
                let id = &new_entity.id;
                let name = &new_entity.name;

                sqlx::query!("INSERT INTO entities (id, name) VALUES ($1, $2)",
                    id as &EntityId,
                    name as &String
                )
                .execute(&mut **db)
                .await?;

                let mut events = Self::convert_new(new_entity);
                let n_events = self.persist_events(&mut **db, &mut events).await?;

                Self::hydrate_entity(events)
            }
        };

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}
