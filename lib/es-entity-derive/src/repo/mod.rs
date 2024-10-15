mod create_fn;
mod options;

use darling::{FromDeriveInput, ToTokens};
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};

use options::RepositoryOptions;

pub fn derive(ast: syn::DeriveInput) -> proc_macro2::TokenStream {
    let opts = match RepositoryOptions::from_derive_input(&ast) {
        Ok(val) => val,
        Err(err) => {
            return err.write_errors();
        }
    };

    let repo = EsRepo::from(&opts);
    quote!(#repo)
}
pub struct EsRepo<'a> {
    repo: &'a syn::Ident,
    create_fn: create_fn::CreateFn<'a>,
}

impl<'a> From<&'a RepositoryOptions> for EsRepo<'a> {
    fn from(opts: &'a RepositoryOptions) -> Self {
        Self {
            repo: &opts.ident,
            create_fn: create_fn::CreateFn::from(opts),
        }
    }
}

impl<'a> ToTokens for EsRepo<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let repo = &self.repo;
        let create_fn = &self.create_fn;

        tokens.append_all(quote! {
            impl #repo {
                #create_fn
            }
        });
    }
}
