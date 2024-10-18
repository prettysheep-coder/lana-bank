use darling::FromMeta;
use syn::{Expr, Type};

#[derive(Default)]
pub struct Fields {
    pub fields: Vec<Field>,
}

impl FromMeta for Fields {
    fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
        let fields = items
            .iter()
            .map(|item| match item {
                darling::ast::NestedMeta::Meta(meta @ syn::Meta::NameValue(name_value)) => {
                    let name = name_value.path.get_ident().cloned().ok_or_else(|| {
                        darling::Error::custom("Expected identifier").with_span(&name_value.path)
                    })?;
                    let field = match name_value.value {
                        Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(ref lit_str),
                            ..
                        }) => Field {
                            name,
                            opts: FieldOpts::new(syn::parse_str(&lit_str.value())?),
                        },
                        _ => Field {
                            name,
                            opts: FieldOpts::from_meta(&meta)?,
                        },
                    };
                    Ok(field)
                }
                _ => Err(darling::Error::custom("Expected name-value pair").with_span(item)),
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Fields { fields })
    }
}

pub struct Field {
    name: syn::Ident,
    opts: FieldOpts,
}

#[derive(FromMeta)]
pub struct FieldOpts {
    ty: syn::Type,
}

impl FieldOpts {
    fn new(ty: Type) -> Self {
        FieldOpts { ty }
    }
}

#[cfg(test)]
mod tests {
    use darling::FromMeta;
    use syn::parse_quote;

    use super::*;

    #[test]
    fn field_from_list() {
        let input: syn::Meta = parse_quote!(thing(ty = "crate::module::Thing"));
        let values = FieldOpts::from_meta(&input).expect("Failed to parse Field");
        assert_eq!(values.ty, parse_quote!(crate::module::Thing));
    }

    #[test]
    fn test_fields_parsing() {
        let input: syn::Meta = parse_quote!(fields(name = "String"));
        let fields = Fields::from_meta(&input).expect("Failed to parse Fields");
        assert_eq!(fields.fields.len(), 1);

        assert_eq!(fields.fields[0].name.to_string(), "name");
    }
}
