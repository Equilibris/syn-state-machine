use crate::*;
#[cfg(feature = "printing")]
use quote::TokenStreamExt;

materialize! {
    on <'a> [RustCursor<'a>]
    pub enum LiteralExpression {
        CharLit       (lit <- CharLit),
        StringLit     (lit <- StringLit),
        ByteLit       (lit <- ByteLit),
        ByteStringLit (lit <- ByteStringLit),
        IntLit        (lit <- IntegerLit),
        FloatLit      (lit <- FloatLit),
        BoolLit       (lit <- bool),
    }
}
to_tokens! {
    impl ToTokens for enum LiteralExpression {
        CharLit(
            lit <- tokens into {
                tokens.append(Literal::from(lit.clone()))
            } to {
                tokens.append(Literal::from(lit.clone()))
            }
        ),
        StringLit(
            lit <- tokens into {
                tokens.append(Literal::from(lit.clone()))
            } to {
                tokens.append(Literal::from(lit.clone()))
            }
        ),
        ByteLit(
            lit <- tokens into {
                tokens.append(Literal::from(lit.clone()))
            } to {
                tokens.append(Literal::from(lit.clone()))
            }
        ),
        ByteStringLit(
            lit <- tokens into {
                tokens.append(Literal::from(lit.clone()))
            } to {
                tokens.append(Literal::from(lit.clone()))
            }
        ),
        IntLit(
            lit <- tokens into {
                tokens.append(Literal::from(lit.clone()))
            } to {
                tokens.append(Literal::from(lit.clone()))
            }
        ),
        FloatLit(
            lit <- tokens into {
                tokens.append(Literal::from(lit.clone()))
            } to {
                tokens.append(Literal::from(lit.clone()))
            }
        ),
        BoolLit(lit <- bool),
    }
}
