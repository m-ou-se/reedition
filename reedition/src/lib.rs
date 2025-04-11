//! See [the `edition` crate](https://docs.rs/edition/).

use proc_macro::{Group, Span, TokenStream, TokenTree};

#[proc_macro]
pub fn reedition(t: TokenStream) -> TokenStream {
    let mut t = t.into_iter();
    let dummy = t.next().unwrap();
    let stream = t.next().unwrap();
    match stream {
        TokenTree::Group(g) => respan(dummy.span(), g.stream()),
        _ => unreachable!(),
    }
}

fn respan(span: Span, tokens: TokenStream) -> TokenStream {
    tokens
        .into_iter()
        .map(|token| match token {
            TokenTree::Group(group) => {
                let mut fixed = Group::new(group.delimiter(), respan(span, group.stream()));
                fixed.set_span(group.span().resolved_at(span));
                TokenTree::Group(fixed)
            }
            mut token => {
                token.set_span(token.span().resolved_at(span));
                token
            }
        })
        .collect()
}
