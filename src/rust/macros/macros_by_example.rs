use crate::*;
#[cfg(feature = "printing")]
use quote::TokenStreamExt;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    pub struct MacroRulesDefinition {
        macro_token <- FIdent<"macro_rules">;
        macro_name <- Ident;
        macro_rules_def <- MacroRulesDef;
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct MacroRulesDefinition {
        macro_token <- KwMacroRules;
        macro_name <- Ident;
        macro_rules_def <- MacroRulesDef;
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    pub enum MacroRulesDef {
        Parentheses(defs <- Paren<MacroRules>),
        Brackets(defs <- Bracket<MacroRules>),
        Braces(defs <- Brace<MacroRules>),
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for enum MacroRulesDef {
        Parentheses(defs <- Paren<MacroRules>),
        Brackets(defs <- Bracket<MacroRules>),
        Braces(defs <- Brace<MacroRules>),
    }
}

pub type MacroRules = InterlaceTrail<MacroRule, Semi>;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    pub struct MacroRule {
        matcher <- MacroMatcher;
        arrow <- FatArrow;
        transcriber <- MacroTranscriber;
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct MacroRule {
        matcher <- MacroMatcher;
        arrow <- FatArrow;
        transcriber <- MacroTranscriber;
    }
}

pub type MacroTranscriber = DelimTokenTree;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    pub struct MacroMatcher {
        matches <- AnyGroup<Rep<MacroMatch>>
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct MacroMatcher {
        matches <- AnyGroup<Rep<MacroMatch>>
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    pub enum MacroMatch {
        MacroMatcher(v <- Box<MacroMatcher>),
        Id(<- Dollar; id <- Ident; <- Colon; frag <- MacroFragSpec),
        Nest(<- Dollar; vs <- Rep<MacroMatch> : Paren<_> { vs.0 }; sep <- Option<MacroRepSep>; rep <- MacroRepOp),
        Token(v <- TokenTree)
    }
}
to_tokens! {
    impl ToTokens for enum MacroMatch {
        MacroMatcher(v <- Box<MacroMatcher>),
        Id(<- Dollar; id <- Ident; <- Colon; frag <- MacroFragSpec),
        Nest(
            <- Dollar;
            vs <- tokens into {
                tokens.append(
                    proc_macro2::Group::new(
                        proc_macro2::Delimiter::Parenthesis, vs.into_token_stream()
                    )
                );
            } to {
                tokens.append(
                    proc_macro2::Group::new(
                        proc_macro2::Delimiter::Parenthesis, vs.to_token_stream()
                    )
                );
            };
            sep <- tokens into {
                if let Some(v) = sep {
                    tokens.extend(v.into_token_stream())
                }
            } to {
                if let Some(v) = sep {
                    v.to_tokens(tokens)
                }
            };
            rep <- MacroRepOp
        ),
        Token(v <- TokenTree)
    }
}

pub struct MacroFragSpec {
    pub id: Ident,
}
to_tokens! {
    impl ToTokens for struct MacroFragSpec {
        id <- Ident
    }
}
impl<'a> Parse<RustCursor<'a>, ()> for MacroFragSpec {
    type Finalizer = BlackHoleFinalizer<Self>;

    fn parse(input: &mut ParseBuffer<RustCursor<'a>>) -> Result<Self::Finalizer, Error> {
        Ok(BlackHoleFinalizer(Self {
            id: input
                .ident_matching(|id: &Ident| {
                    if id == "block"
                        || id == "expr"
                        || id == "ident"
                        || id == "ite"
                        || id == "lifetime"
                        || id == "literal"
                        || id == "meta"
                        || id == "pat"
                        || id == "pat_param"
                        || id == "path"
                        || id == "stmt"
                        || id == "tt"
                        || id == "ty"
                        || id == "vis"
                    {
                        Ok(())
                    } else {
                        Err(Error::new(id.span(), "Expected bool literal"))
                    }
                })?
                .clone(),
        }))
    }
}

type MacroRepSep = TokenTree;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    pub enum MacroRepOp {
        ZeroOrMore(<- Star),
        OneOrMore(<- Plus),
        OneOrZero(<- Question),
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for enum MacroRepOp {
        ZeroOrMore(<- Star),
        OneOrMore(<- Plus),
        OneOrZero(<- Question),
    }
}
