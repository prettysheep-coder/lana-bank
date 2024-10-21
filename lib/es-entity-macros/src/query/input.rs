use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

pub struct QueryInput {
    pub(super) executor: syn::Expr,
    pub(super) sql: String,
    pub(super) sql_span: Span,
    pub(super) arg_exprs: Vec<syn::Expr>,
}

impl QueryInput {
    pub(super) fn table_name(&self) -> darling::Result<String> {
        let query = self.sql.to_lowercase();
        let words: Vec<&str> = query.split_whitespace().collect();
        let from_pos = words.iter().position(|&word| word == "from").ok_or(
            darling::Error::custom("Could not identify table name - no 'FROM' clause")
                .with_span(&self.sql_span),
        )?;
        let table_name = words.get(from_pos + 1).ok_or(
            darling::Error::custom("No word after 'FROM' clause").with_span(&self.sql_span),
        )?;
        let table_name = table_name.trim_end_matches(|c: char| !c.is_alphanumeric());
        Ok(table_name.to_string())
    }
}

impl Parse for QueryInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut sql: Option<(String, Span)> = None;
        let mut args: Option<Vec<syn::Expr>> = None;
        let mut executor: Option<syn::Expr> = None;
        let mut expect_comma = false;

        while !input.is_empty() {
            if expect_comma {
                let _ = input.parse::<syn::token::Comma>()?;
            }
            let key: syn::Ident = input.parse()?;

            let _ = input.parse::<syn::token::Eq>()?;

            if key == "executor" {
                executor = Some(input.parse::<syn::Expr>()?);
            } else if key == "sql" {
                sql = Some((
                    Punctuated::<syn::LitStr, syn::Token![+]>::parse_separated_nonempty(input)?
                        .iter()
                        .map(syn::LitStr::value)
                        .collect(),
                    input.span(),
                ));
            } else if key == "args" {
                let exprs = input.parse::<syn::ExprArray>()?;
                args = Some(exprs.elems.into_iter().collect())
            } else {
                let message = format!("unexpected input key: {key}");
                return Err(syn::Error::new_spanned(key, message));
            }

            expect_comma = true;
        }

        let (sql, sql_span) = sql.ok_or_else(|| input.error("expected `sql` key"))?;
        let executor = executor.ok_or_else(|| input.error("expected `executor` key"))?;

        Ok(QueryInput {
            executor,
            sql,
            sql_span,
            arg_exprs: args.unwrap_or_default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn parse_input() {
        let input: QueryInput = parse_quote!(
            executor = &mut **tx,
            sql = "SELECT * FROM users WHERE name = $1",
            args = [id]
        );
        assert_eq!(input.sql, "SELECT * FROM users WHERE name = $1");
        assert_eq!(input.executor, parse_quote!(&mut **tx));
        assert_eq!(input.arg_exprs[0], parse_quote!(id));
    }
}
