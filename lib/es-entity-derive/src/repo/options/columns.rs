use darling::FromMeta;
use quote::quote;

#[derive(Default)]
pub struct Columns {
    pub all: Vec<Column>,
}

impl Columns {
    pub fn set_id_column(&mut self, ty: &syn::Ident) {
        let mut all = vec![Column::new(
            syn::Ident::new("id", proc_macro2::Span::call_site()),
            syn::parse_str(&ty.to_string()).unwrap(),
        )];
        all.append(&mut self.all);
        self.all = all;
    }

    pub fn all_find_by(&self) -> impl Iterator<Item = &Column> {
        self.all.iter().filter(|c| c.opts.find_by())
    }

    pub fn all_list_by(&self) -> impl Iterator<Item = &Column> {
        self.all.iter().filter(|c| c.opts.list_by())
    }

    pub fn updates_needed(&self) -> bool {
        self.all.len() > 1
    }

    pub fn variable_assignments(&self, ident: syn::Ident) -> proc_macro2::TokenStream {
        let assignments = self
            .all
            .iter()
            .map(|column| column.variable_assignment(&ident));
        quote! {
            #(#assignments)*
        }
    }

    pub fn names(&self) -> Vec<String> {
        self.all.iter().map(|c| c.name.to_string()).collect()
    }

    pub fn placeholders(&self) -> String {
        (1..=self.all.len())
            .map(|i| format!("${}", i))
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn sql_updates(&self) -> String {
        self.all
            .iter()
            .skip(1)
            .enumerate()
            .map(|(idx, column)| format!("{} = ${}", column.name, idx + 2))
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn query_args(&self) -> Vec<proc_macro2::TokenStream> {
        self.all
            .iter()
            .map(|column| {
                let ident = &column.name;
                let ty = &column.opts.ty;
                quote! {
                    #ident as &#ty
                }
            })
            .collect()
    }
}

impl FromMeta for Columns {
    fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
        let all = items
            .iter()
            .map(|item| match item {
                darling::ast::NestedMeta::Meta(meta @ syn::Meta::NameValue(name_value)) => {
                    let name = name_value.path.get_ident().cloned().ok_or_else(|| {
                        darling::Error::custom("Expected identifier").with_span(&name_value.path)
                    })?;
                    let column = match name_value.value {
                        syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(ref lit_str),
                            ..
                        }) => Column::new(name, syn::parse_str(&lit_str.value())?),
                        _ => Column {
                            name,
                            opts: ColumnOpts::from_meta(meta)?,
                        },
                    };
                    Ok(column)
                }
                _ => Err(darling::Error::custom("Expected name-value pair").with_span(item)),
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Columns { all })
    }
}

pub struct Column {
    pub name: syn::Ident,
    pub opts: ColumnOpts,
}

impl Column {
    pub fn new(name: syn::Ident, ty: syn::Type) -> Self {
        Column {
            name,
            opts: ColumnOpts::new(ty),
        }
    }
}

impl Column {
    pub fn name(&self) -> &syn::Ident {
        &self.name
    }

    pub fn ty(&self) -> &syn::Type {
        &self.opts.ty
    }

    fn variable_assignment(&self, ident: &syn::Ident) -> proc_macro2::TokenStream {
        let name = &self.name;
        quote! {
            let #name = &#ident.#name;
        }
    }
}

#[derive(FromMeta)]
pub struct ColumnOpts {
    ty: syn::Type,
    #[darling(default)]
    find_by: Option<bool>,
    #[darling(default)]
    list_by: Option<bool>,
}

impl ColumnOpts {
    fn new(ty: syn::Type) -> Self {
        ColumnOpts {
            ty,
            find_by: Some(true),
            list_by: Some(true),
        }
    }

    fn find_by(&self) -> bool {
        self.find_by.unwrap_or(true)
    }

    fn list_by(&self) -> bool {
        self.list_by.unwrap_or(true)
    }
}

#[cfg(test)]
mod tests {
    use darling::FromMeta;
    use syn::parse_quote;

    use super::*;

    #[test]
    fn column_from_list() {
        let input: syn::Meta = parse_quote!(thing(ty = "crate::module::Thing"));
        let values = ColumnOpts::from_meta(&input).expect("Failed to parse Field");
        assert_eq!(values.ty, parse_quote!(crate::module::Thing));
    }

    #[test]
    fn columns_from_list() {
        let input: syn::Meta = parse_quote!(columns(name = "String"));
        let columns = Columns::from_meta(&input).expect("Failed to parse Fields");
        assert_eq!(columns.all.len(), 1);

        assert_eq!(columns.all[0].name.to_string(), "name");
    }
}
