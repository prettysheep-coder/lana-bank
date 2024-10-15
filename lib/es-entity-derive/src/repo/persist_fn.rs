use darling::ToTokens;
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};

use super::options::*;

pub struct PersistFn<'a> {
    entity: &'a syn::Ident,
    error: &'a syn::Ident,
}

impl<'a> From<&'a RepositoryOptions> for PersistFn<'a> {
    fn from(opts: &'a RepositoryOptions) -> Self {
        Self {
            entity: opts.entity(),
            error: opts.err(),
        }
    }
}

impl<'a> ToTokens for PersistFn<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let entity = self.entity;
        let error = self.error;

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
    fn test_create_fn() {
        let entity = Ident::new("Entity", Span::call_site());
        let error = Ident::new("EsRepoError", Span::call_site());

        let create_fn = PersistFn {
            entity: &entity,
            error: &error,
        };

        let mut tokens = TokenStream::new();
        create_fn.to_tokens(&mut tokens);

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
