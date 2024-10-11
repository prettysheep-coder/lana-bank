use darling::ToTokens;
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};

use super::options::RepositoryOptions;

pub struct CreateFn<'a> {
    new_entity: &'a syn::Ident,
    entity: &'a syn::Ident,
    error: &'a syn::Ident,
}

impl<'a> From<&'a RepositoryOptions> for CreateFn<'a> {
    fn from(opts: &'a RepositoryOptions) -> Self {
        Self {
            new_entity: opts.new_entity(),
            entity: opts.entity(),
            error: opts.err(),
        }
    }
}

impl<'a> ToTokens for CreateFn<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let new_entity = self.new_entity;
        let entity = self.entity;
        let error = self.error;

        tokens.append_all(quote! {
            pub async fn create_in_tx(
                &self,
                db: &mut sqlx::Transaction<'_, sqlx::Postgres>,
                new_entity: #new_entity
            ) -> Result<#entity, #error> {
                // let row = sqlx::query!(
                //     r#"INSERT INTO #entity_name (name, email, password_hash)
                //     VALUES ($1, $2, $3)
                //     RETURNING *"#,
                //     new_user.name,
                //     new_user.email,
                //     new_user.password_hash
                // )
                // .fetch_one(&mut **db)
                // .await?;

                // Ok(Self::from_row(row))
            }
        });
    }
}
