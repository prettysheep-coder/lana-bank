use darling::ToTokens;
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};

use super::options::*;

pub struct CreateFn<'a> {
    new_entity: &'a syn::Ident,
    entity: &'a syn::Ident,
    error: &'a syn::Ident,
    indexes: &'a Indexes,
}

impl<'a> From<&'a RepositoryOptions> for CreateFn<'a> {
    fn from(opts: &'a RepositoryOptions) -> Self {
        Self {
            new_entity: opts.new_entity(),
            entity: opts.entity(),
            error: opts.err(),
            indexes: &opts.indexes,
        }
    }
}

impl<'a> ToTokens for CreateFn<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let new_entity = self.new_entity;
        let entity = self.entity;
        let error = self.error;

        let index_tokens = self.indexes.columns.iter().map(|column| {
            let ident = &column.name;
            quote! {
                let #ident = &new_entity.#ident;
            }
        });

        let table_name = "users";
        let columns_names: Vec<_> = self
            .indexes
            .columns
            .iter()
            .map(|c| c.name.to_string())
            .collect();
        let placeholders = (1..=self.indexes.columns.len())
            .map(|i| format!("${}", i))
            .collect::<Vec<_>>()
            .join(", ");
        let args: Vec<_> = self
            .indexes
            .columns
            .iter()
            .map(|column| {
                let ident = &column.name;
                match &column.ty {
                    Some(ty) => quote! {
                        #ident as &#ty
                    },
                    None => quote! {
                        #ident
                    },
                }
            })
            .collect();

        let query = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name,
            columns_names.join(", "),
            placeholders,
        );

        tokens.append_all(quote! {
            #[inline(always)]
            fn convert_new<T, E>(item: T) -> EntityEvents<E>
            where
                T: IntoEvents<E>,
                E: EsEvent,
            {
                item.into_events()
            }

            pub async fn create_in_tx(
                &self,
                db: &mut sqlx::Transaction<'_, sqlx::Postgres>,
                new_entity: #new_entity
            ) -> Result<#entity, #error> {
                use es_entity::IntoEvents;

                #(#index_tokens)*
                 sqlx::query!(
                     #query,
                     #(#args),*
                )
                .execute(&mut **db)
                .await?;

                let events = Self::convert_new(new_entity);
                unimplemented!()
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
        let error = Ident::new("EsEntityError", Span::call_site());

        let indexes = Indexes {
            columns: vec![
                IndexColumn {
                    name: Ident::new("id", Span::call_site()),
                    ty: Some(Ident::new("UserId", Span::call_site())),
                },
                IndexColumn {
                    name: Ident::new("name", Span::call_site()),
                    ty: None,
                },
            ],
        };

        let create_fn = CreateFn {
            new_entity: &new_entity,
            entity: &entity,
            error: &error,
            indexes: &indexes,
        };

        let mut tokens = TokenStream::new();
        create_fn.to_tokens(&mut tokens);

        let expected = quote! {
            #[inline(always)]
            fn convert_new<T, E>(item: T) -> EntityEvents<E>
            where
                T: IntoEvents<E>,
                E: EsEvent,
            {
                item.into_events()
            }

            pub async fn create_in_tx(
                &self,
                db: &mut sqlx::Transaction<'_, sqlx::Postgres>,
                new_entity: NewEntity
            ) -> Result<Entity, EsEntityError> {
                use es_entity::IntoEvents;

                let id = &new_entity.id;
                let name = &new_entity.name;

                sqlx::query!("INSERT INTO users (id, name) VALUES ($1, $2)",
                    id as &UserId,
                    name
                )
                .execute(&mut **db)
                .await?;

                let events = Self::convert_new(new_entity);
                unimplemented!()
            }
        };

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}
