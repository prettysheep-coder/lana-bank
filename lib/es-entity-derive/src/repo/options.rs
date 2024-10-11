use darling::{FromDeriveInput, FromMeta};

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(attributes(es_repo), map = "Self::update_defaults")]
pub struct RepositoryOptions {
    pub ident: syn::Ident,
    #[darling(default)]
    pub indexes: Indexes,

    #[darling(default, rename = "entity")]
    entity_ident: Option<syn::Ident>,
    #[darling(default, rename = "new_entity")]
    new_entity_ident: Option<syn::Ident>,
    #[darling(default, rename = "err")]
    err_ident: Option<syn::Ident>,
}

impl RepositoryOptions {
    fn update_defaults(mut self) -> Self {
        let entity_name = match &self.entity_ident {
            None => {
                let name = self.ident.to_string();
                let entity_name = if name.ends_with("Repo") {
                    name.strip_suffix("Repo").unwrap()
                } else if name.ends_with("Repository") {
                    name.strip_suffix("Repository").unwrap()
                } else {
                    &name
                };
                self.entity_ident = Some(syn::Ident::new(entity_name, self.ident.clone().span()));
                entity_name.to_string()
            }
            Some(ident) => ident.to_string(),
        };

        if self.new_entity_ident.is_none() {
            self.new_entity_ident = Some(syn::Ident::new(
                &format!("New{}", entity_name),
                self.ident.span(),
            ));
        }

        if self.err_ident.is_none() {
            self.err_ident = Some(syn::Ident::new(
                &format!("{}Error", entity_name),
                self.ident.span(),
            ));
        }
        self
    }

    pub fn entity(&self) -> &syn::Ident {
        self.entity_ident.as_ref().unwrap()
    }

    pub fn new_entity(&self) -> &syn::Ident {
        self.new_entity_ident.as_ref().unwrap()
    }

    pub fn err(&self) -> &syn::Ident {
        self.err_ident.as_ref().unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct Indexes {
    pub columns: Vec<syn::Ident>,
}

impl Default for Indexes {
    fn default() -> Self {
        Self {
            columns: vec![syn::Ident::new("id", proc_macro2::Span::call_site())],
        }
    }
}

impl FromMeta for Indexes {
    fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
        let columns = items
            .iter()
            .map(|item| match item {
                darling::ast::NestedMeta::Meta(syn::Meta::Path(path)) => path
                    .get_ident()
                    .cloned()
                    .ok_or_else(|| darling::Error::custom("Expected identifier").with_span(path)),
                _ => Err(darling::Error::custom("Expected identifier").with_span(item)),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Indexes { columns })
    }
}
