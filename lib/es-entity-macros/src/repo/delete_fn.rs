use darling::ToTokens;
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};

use super::{options::DeleteOption, RepositoryOptions};

pub struct DeleteFn<'a> {
    id: &'a syn::Ident,
    error: &'a syn::Type,
    table_name: &'a str,
    delete_option: &'a DeleteOption,
}

impl<'a> DeleteFn<'a> {
    pub fn from(opts: &'a RepositoryOptions) -> Self {
        Self {
            id: opts.id(),
            error: opts.err(),
            table_name: opts.table_name(),
            delete_option: &opts.delete,
        }
    }
}

impl<'a> ToTokens for DeleteFn<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if matches!(self.delete_option, DeleteOption::No) {
            return;
        }

        let id_ty = self.id;
        let error_ty = self.error;
        let table_name = self.table_name;

        let query = match self.delete_option {
            DeleteOption::Soft => {
                format!("UPDATE {} SET deleted = TRUE WHERE id = $1", table_name)
            }
            _ => unreachable!(),
        };

        tokens.append_all(quote! {
            pub async fn delete_by_id_in_tx(&self,
                id: &#id_ty,
                db: &mut sqlx::Transaction<'_, sqlx::Postgres>,
             ) -> Result<(), #error_ty> {
                sqlx::query!(#query, id)
                    .execute(self.pool())
                    .await?;
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
    fn delete_fn() {
        let id = Ident::new("EntityId", Span::call_site());
        let error = syn::parse_str("es_entity::EsRepoError").unwrap();
        let delete_option = DeleteOption::Soft;

        let delete_fn = DeleteFn {
            id: &id,
            error: &error,
            table_name: "entities",
            delete_option: &delete_option,
        };

        let mut tokens = TokenStream::new();
        delete_fn.to_tokens(&mut tokens);

        let expected = quote! {
            pub async fn delete_by_id_in_tx(&self,
                id: &EntityId,
                db: &mut sqlx::Transaction<'_, sqlx::Postgres>,
             ) -> Result<(), es_entity::EsRepoError> {
                sqlx::query!("UPDATE entities SET deleted = TRUE WHERE id = $1", id)
                    .execute(self.pool())
                    .await?;
                Ok(())
            }
        };

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}
