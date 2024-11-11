use darling::ToTokens;
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};

use super::RepositoryOptions;

pub struct CurrentTime<'a> {
    current_time: &'a Option<syn::Ident>,
}

impl<'a> From<&'a RepositoryOptions> for CurrentTime<'a> {
    fn from(opts: &'a RepositoryOptions) -> Self {
        Self {
            current_time: &opts.current_time,
        }
    }
}

impl<'a> ToTokens for CurrentTime<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let current_time = if let Some(current_time) = self.current_time {
            quote! {
                self.#current_time()
            }
        } else {
            quote! {
                chrono::Utc::now()
            }
        };

        tokens.append_all(quote! {
            #[inline(always)]
            fn current_time(&self) -> chrono::DateTime<chrono::Utc> {
                #current_time
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_time() {
        let current_time = None;

        let current_time = CurrentTime {
            current_time: &current_time,
        };

        let mut tokens = TokenStream::new();
        current_time.to_tokens(&mut tokens);

        let expected = quote! {
            #[inline(always)]
            fn current_time(&self) -> chrono::DateTime<chrono::Utc> {
                chrono::Utc::now()
            }
        };

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}
