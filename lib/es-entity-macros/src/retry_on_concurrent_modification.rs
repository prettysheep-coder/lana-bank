pub fn make(input: syn::ItemFn) -> darling::Result<proc_macro2::TokenStream> {
    let sig = &input.sig;
    let body = &input.block;

    let output = quote::quote! {
        #sig {
            let result = #body;
            if result.was_concurrent_modification()
                #body
            else {
                result
            }
        }
    };
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn retry_on_concurrent_modification() {
        let input = parse_quote! {
            #[retry_on_concurrent_modification]
            async fn test(&self) -> Result<(), es_entity::EsRepoError> {
                self.repo.update().await?;
                Ok(())
            }
        };

        let output = make(input).unwrap();
        let expected = quote::quote! {
            async fn test(&self) -> Result<(), es_entity::EsRepoError> {
                let result = {
                    self.repo.update().await?;
                    Ok(())
                };
                if result.was_concurrent_modification() {
                    self.repo.update().await?;
                    Ok(())
                } else {
                    result
                }
            }
        };
        assert_eq!(output.to_string(), expected.to_string());
    }
}
