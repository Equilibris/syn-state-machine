use crate::*;

// TODO

// materialize! {
//     on <'a> [RustCursor<'a>]
//     pub struct MatchArms {

//         final <- (MatchArm<Attr,Pat,Expr>, )
//     }
// }

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct MatchArm<Attr, Pat, Expr> {
        attrs <- Rep<OuterAttribute<Attr>>;
        pat <- Pat;
        guard <- Option<MatchArmGuard<Expr>>
    }
}
to_tokens! {
    impl ToTokens for struct MatchArm<Attr, Pat, Expr> {
        attrs <- Rep<OuterAttribute<Attr>>;
        pat <- Pat;
        guard <- tokens into {
            if let Some(guard) = guard {
                tokens.extend(guard.into_token_stream())
            }
        } to {
            if let Some(ref guard) = guard {
                guard.to_tokens(tokens)
            }
        }
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct MatchArmGuard<Expr> {
        <- KwIf;
        expr <- Expr;
    }
}
to_tokens! {
    impl ToTokens for struct MatchArmGuard<Expr> {
        <- KwIf;
        expr <- Expr;
    }
}
