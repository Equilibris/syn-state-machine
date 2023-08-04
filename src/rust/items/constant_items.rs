use crate::*;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct ConstantItem <Ty, Expr> {
        <- KwConst;
        id <- Ident : IdentifierOrUnder;
        <- Colon;
        ty <- Ty;
        eq <- Option<Expr> : Option<(Eq, _)> { eq.map(|v|v.1) };
        <- Semi
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct ConstantItem<Ty, Expr> {
        <- KwConst;
        id <- Ident;
        <- Colon;
        ty <- Ty;
        eq <- tokens into {
            if let Some(eq) = eq {
                tokens.extend(Eq::default().into_token_stream());
                tokens.extend(eq.into_token_stream())
            }
        } to {
            if let Some(eq) = eq {
                tokens.extend(Eq::default().into_token_stream());
                tokens.extend(eq.into_token_stream())
            }
        };
        <- Semi
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    insta_match_test!(parse : it_matches_const_item, ConstantItem<Ident, Infallible> : const HELLO: usize;);
}
