use darling::FromMeta;
use quote::quote;

#[derive(Default)]
pub struct Columns {
    pub all: Vec<Column>,
}

impl Columns {
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
                            opts: ColumnOpts::from_meta(&meta)?,
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

#[derive(FromMeta)]
pub struct ColumnOpts {
    pub ty: syn::Type,
}

impl ColumnOpts {
    fn new(ty: syn::Type) -> Self {
        ColumnOpts { ty }
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
