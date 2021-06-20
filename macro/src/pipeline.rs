use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_quote,
    token, Expr, Ident, Result, Token, Stmt,
};

#[derive(Debug)]
pub(super) struct Pipeline {
    first: Expr,
    statements: Vec<Stmt>,
}

impl Parse for Pipeline {
    fn parse(input: ParseStream) -> Result<Self> {
        let first: Expr = input.parse()?;
        
        let mut statements: Vec<Stmt> = vec![];
        while !input.is_empty() {
            input.parse::<Token![=>]>()?;

            let (expr, has_argument) = replace(input, false)?;
            let expr: Expr = parse_quote!(#expr);
            statements.push(if has_argument {
                parse_quote!{
                    let ret = #expr;
                }
            } else {
                parse_quote!{
                    let ret = #expr(ret);
                }
            })
        }
        Ok(Self {
            first,
            statements
        })
    }
}

fn replace(input: ParseStream, recursed: bool) -> Result<(TokenStream, bool)> {
    let mut tokens = TokenStream::new();
    let mut has_argument = false;
    while !input.is_empty() {
        if input.peek(Token![=>]) && !recursed{
            break;
        } else if input.peek(token::Paren) {
            let content;
            let paren_token = parenthesized!(content in input);
            let (token_tree, has_argument_inner) = replace(&content, true)?;
            has_argument |= has_argument_inner;
            paren_token.surround(&mut tokens, |tokens| {
                token_tree.to_tokens(tokens);
            });
        } else if input.peek(Token![_]) {
            let _token = input.parse::<Token![_]>()?;
            let ident = Ident::new("ret", Span::call_site());
            tokens.extend(quote!(#ident));
            has_argument = true;
        } else {
            let token_tree = input.parse::<TokenTree>()?;
            tokens.extend(Some(token_tree));
        }
    }
    Ok((tokens, has_argument))
}

impl ToTokens for Pipeline {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let first = &self.first;
        let statements = &self.statements;
        let expanded = quote!{{
            let ret = #first;
            #( #statements )*
            ret
        }};
        tokens.extend(expanded);
    }
}
