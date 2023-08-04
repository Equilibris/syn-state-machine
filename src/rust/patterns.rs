use crate::*;
#[cfg(feature = "printing")]
use quote::TokenStreamExt;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    pub struct Pattern<Attr, Ty> {
        leading peek <- Or;

        entries <- MinLength<Interlace<PatternNoTopAlt<Attr, Ty>, Or>>
    }
}
to_tokens! {
    impl ToTokens for struct Pattern<Attr, Ty> {
        leading peek <- Or;

        entries <- MinLength<Interlace<PatternNoTopAlt<Attr, Ty>, Or>>
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    pub enum PatternNoTopAlt<Attr, Ty> {
        PatternWithoutRange(v <- PatternWithoutRange<Attr, Ty>),
        RangePattern(v <- RangePattern<Ty>)
    }
}
to_tokens! {
    impl ToTokens for enum PatternNoTopAlt<Attr, Ty> {
        PatternWithoutRange(v <- PatternWithoutRange<Attr, Ty>),
        RangePattern(v <- RangePattern<Ty>)
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    pub enum PatternWithoutRange<Attr, Ty> {
        LiteralPattern(v <- LiteralPattern),
        IdentifierPattern(v <- IdentifierPattern<Box<Pattern<Attr, Ty>>>),
        WildcardPattern(v <- WildcardPattern),
        RestPattern(v <- RestPattern),
        ReferencePattern(v <- ReferencePattern<Box<Pattern<Attr, Ty>>>),
        StructPattern(v <- StructPattern<Attr, Ty, Box<Pattern<Attr, Ty>>>),
        TupleStructPattern(v <- TupleStructPattern<Attr, Ty>),
        TuplePattern(v <- TuplePattern<Box<Pattern<Attr, Ty>>>),
        GroupedPattern(v <- GroupedPattern<Box<Pattern<Attr, Ty>>>),
        SlicePattern(v <- SlicePattern<Box<Pattern<Attr, Ty>>>),
        PathPattern(v <- PathPattern<Ty>),
        MacroInvocation(v <- MacroInvocation)
    }
}
to_tokens! {
    impl ToTokens for enum PatternWithoutRange<Attr, Ty> {
        LiteralPattern(v <- LiteralPattern),
        IdentifierPattern(v <- IdentifierPattern),
        WildcardPattern(v <- WildcardPattern),
        RestPattern(v <- RestPattern),
        ReferencePattern(v <- ReferencePattern<Box<Pattern<Attr, Ty>>>),
        StructPattern(v <- StructPattern<Attr, Ty, Box<Pattern<Attr, Ty>>>),
        TupleStructPattern(v <- TupleStructPattern<Attr, Ty>),
        TuplePattern(v <- TuplePattern<Box<Pattern<Attr, Ty>>>),
        GroupedPattern(v <- GroupedPattern<Box<Pattern<Attr, Ty>>>),
        SlicePattern(v <- SlicePattern<Box<Pattern<Attr, Ty>>>),
        PathPattern(v <- PathPattern),
        MacroInvocation(v <- MacroInvocation)
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    pub enum RangePattern <Ty> {
        RangeToInclusivePattern(v <- RangeToInclusivePattern<Ty>),
        RangeInclusivePattern(v <- RangeInclusivePattern<Ty>),
        ObsoleteRangePattern(v <- ObsoleteRangePattern<Ty>),
        RangeFromPattern(v <- RangeFromPattern<Ty>)
    }
}
to_tokens! {
    impl ToTokens for enum RangePattern<Ty> {
        RangeToInclusivePattern(v <- RangeToInclusivePattern<Ty>),
        RangeInclusivePattern(v <- RangeInclusivePattern<Ty>),
        ObsoleteRangePattern(v <- ObsoleteRangePattern<Ty>),
        RangeFromPattern(v <- RangeFromPattern<Ty>)
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    pub struct RangeToInclusivePattern <Ty> {
        <- DotDotEq;
        r <- RangePatternBound<Ty>;
    }
}
to_tokens! {
    impl ToTokens for struct RangeToInclusivePattern<Ty> {
        <- DotDotEq;
        r <- RangePatternBound<Ty>;
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    pub struct RangeFromPattern<Ty> {
        l <- RangePatternBound<Ty>;
        <- DotDot;
    }
}
to_tokens! {
    impl ToTokens for struct RangeFromPattern<Ty> {
        l <- RangePatternBound<Ty>;
        <- DotDot;
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    pub struct RangeInclusivePattern<Ty> {
        l <- RangePatternBound<Ty>;
        <- DotDotEq;
        r <- RangePatternBound<Ty>;
    }
}
to_tokens! {
    impl ToTokens for struct RangeInclusivePattern<Ty> {
        l <- RangePatternBound<Ty>;
        <- DotDotEq;
        r <- RangePatternBound<Ty>;
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    pub struct ObsoleteRangePattern<Ty> {
        l <- RangePatternBound<Ty>;
        <- DotDotDot;
        r <- RangePatternBound<Ty>;
    }
}
to_tokens! {
    impl ToTokens for struct ObsoleteRangePattern<Ty> {
        l <- RangePatternBound<Ty>;
        <- DotDotDot;
        r <- RangePatternBound<Ty>;
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    pub enum RangePatternBound <Ty> {
        Char(v <- CharLit),
        Byte(v <- ByteLit),
        Int(v <- SignedIntegerLit),
        Float(v <- SignedFloatLit),
        Path(v <- PathInExpression<Ty>)
    }
}
to_tokens! {
    impl ToTokens for enum RangePatternBound<Ty> {
        Char(
            lit <- tokens into {
                tokens.append(Literal::from(lit.clone()))
            } to {
                tokens.append(Literal::from(lit.clone()))
            }
        ),
        Byte(
            lit <- tokens into {
                tokens.append(Literal::from(lit))
            } to {
                tokens.append(Literal::from(lit.clone()))
            }
        ),
        Int(v <- SignedIntegerLit),
        Float(v <- SignedFloatLit),
        Path(v <- PathInExpression<Ty>)
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    pub enum LiteralPattern {
        Bool(v <- bool),
        Char(v <- CharLit),
        Byte(v <- ByteLit),
        String(v <- StringLit),
        ByteString(v <- ByteStringLit),
        Int(v <- SignedIntegerLit),
        Float(v <- SignedFloatLit)
    }
}
to_tokens! {
    impl ToTokens for enum LiteralPattern {
        Bool(v <- bool),
        Char(
            lit <- tokens into {
                tokens.append(Literal::from(lit.clone()))
            } to {
                tokens.append(Literal::from(lit.clone()))
            }
        ),
        Byte(
            lit <- tokens into {
                tokens.append(Literal::from(lit.clone()))
            } to {
                tokens.append(Literal::from(lit.clone()))
            }
        ),
        String(
            lit <- tokens into {
                tokens.append(Literal::from(lit.clone()))
            } to {
                tokens.append(Literal::from(lit.clone()))
            }
        ),
        ByteString(
            lit <- tokens into {
                tokens.append(Literal::from(lit.clone()))
            } to {
                tokens.append(Literal::from(lit.clone()))
            }
        ),
        Int(v <- SignedIntegerLit),
        Float(v <- SignedFloatLit)
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    pub struct IdentifierPattern <Pat> {
        r#ref peek <- KwRef;
        r#mut peek <- KwMut;
        id <- Ident : Identifier;
        pat <- Option<Pat> : Option<(At, _)> { pat.map(|v|v.1) }
    }
}
to_tokens! {
    impl ToTokens for struct IdentifierPattern<Pat> {
        r#ref peek <- KwRef;
        r#mut peek <- KwMut;
        id <- Ident;
        pat <- tokens into {
            if let Some(pat) = pat {
                tokens.extend(At::default().into_token_stream());
                tokens.extend(pat.into_token_stream());
            }
        } to {
            if let Some(pat) = pat {
                tokens.extend(At::default().into_token_stream());
                pat.to_tokens(tokens)
            }
        }
    }
}
pub type WildcardPattern = FIdent<"_">;
pub type RestPattern = DotDot;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    pub struct ReferencePattern <Pat> {
        <- And;
        double peek <- And;
        r#mut peek <- KwMut;
        pat <- Pat
    }
}
to_tokens! {
    impl ToTokens for struct ReferencePattern<Pat> {
        <- And;
        double peek <- And;
        r#mut peek <- KwMut;
        pat <- Pat
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    pub struct StructPattern <Attr, Ty, Pat> {
        path <- PathInExpression<Ty>;
        elements <- StructPatternElements<Attr, Pat>
    }
}
to_tokens! {
    impl ToTokens for struct StructPattern<Attr, Ty, Pat> {
        path <- PathInExpression<Ty>;
        elements <- StructPatternElements<Attr, Pat>
    }
}

pub struct StructPatternElements<Attr, Pat> {
    pub fields: Interlace<StructPatternField<Attr, Pat>, Comma>,
    pub et_cetera: Option<StructPatternEtCetera<Attr>>,
}
to_tokens! {
    impl ToTokens for struct StructPatternElements<Attr, Pat> {
        fields <- Interlace<StructPatternField<Attr, Pat>, Comma>;
        <- Comma;
        et_cetera <- tokens into {
            if let Some(et_cetera) = et_cetera {
                tokens.extend(et_cetera.into_token_stream())
            }
        } to {
            if let Some(et_cetera) = et_cetera {
                et_cetera.to_tokens(tokens)
            }
        }
    }
}

impl<'a, Attr: Parse<RustCursor<'a>, ()>, Pat: Parse<RustCursor<'a>, ()>> Parse<RustCursor<'a>, ()>
    for StructPatternElements<Attr, Pat>
{
    type Finalizer = BlackHoleFinalizer<Self>;

    fn parse(input: &mut ParseBuffer<RustCursor<'a>>) -> Result<Self::Finalizer, Error> {
        Ok(
            match input.parse::<Sum2<(_, Sum3<Comma, (Comma, _), ()>), _>>()? {
                Sum2::V0((fields, Sum3::V0(_) | Sum3::V2(_))) => BlackHoleFinalizer(Self {
                    fields,
                    et_cetera: None,
                }),
                Sum2::V0((fields, Sum3::V1((_, et_cetera)))) => BlackHoleFinalizer(Self {
                    fields,
                    et_cetera: Some(et_cetera),
                }),
                Sum2::V1(et_cetera) => BlackHoleFinalizer(Self {
                    fields: Default::default(),
                    et_cetera: Some(et_cetera),
                }),
            },
        )
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    pub enum StructPatternField<Attr, Pat> [ attrs <- Rep<OuterAttribute<Attr>> ] {
        TupleIdx(v <- TupleIndex; <- Colon; p <- Pat),
        Id(v <- Ident : Identifier; <- Colon; p <- Pat),
        Shorthand(r#ref peek <- KwRef; r#mut peek <- KwMut; id <- Ident : Identifier)
    }
}
to_tokens! {
    impl ToTokens for enum StructPatternField<Attr, Pat> [ attrs <- Rep<OuterAttribute<Attr>> ] {
        TupleIdx(
            v <- tokens into {
                tokens.append(Literal::from(v.clone()))
            } to {
                tokens.append(Literal::from(v.clone()))
            };
            <- Colon;
            p <- Pat
        ),
        Id(v <- Ident; <- Colon; p <- Pat),
        Shorthand(r#ref peek <- KwRef; r#mut peek <- KwMut; id <- Ident)
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    pub struct StructPatternEtCetera<Attr> {
        attrs <- Rep<OuterAttribute<Attr>>;
        <- DotDot
    }
}
to_tokens! {
    impl ToTokens for struct StructPatternEtCetera<Attr> {
        attrs <- Rep<OuterAttribute<Attr>>;
        <- DotDot
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    pub struct TupleStructPattern <Ty, Pat> {
        path <- PathInExpression<Ty>;
        items <- TupleStructItems<Pat> : Paren<_> { items.0 }
    }
}
to_tokens! {
    impl ToTokens for struct TupleStructPattern<Ty, Pat> {
        path <- PathInExpression<Ty>;
        items <- tokens into {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Parenthesis, items.into_token_stream()
                )
            );
        } to {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Parenthesis, items.to_token_stream()
                )
            );
        }
    }
}

pub type TupleStructItems<Pat> = InterlaceTrail<Pat, Comma>;

pub type TuplePattern<Pat> = Paren<Option<TuplePatternItems<Pat>>>;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    pub enum TuplePatternItems<Pat> {
        Pat(v <- Pat),
        Rest(v <- RestPattern),
        Items(v <- InterlaceTrail<Pat, Comma> : MinLength<_, 2> { v.0 })
    }
}
to_tokens! {
    impl ToTokens for enum TuplePatternItems<Pat> {
        Pat(v <- Pat),
        Rest(v <- RestPattern),
        Items(v <- InterlaceTrail<Pat, Comma>)
    }
}

pub type GroupedPattern<Pat> = Paren<Pat>;
pub type SlicePattern<Pat> = Bracket<SlicePatternItems<Pat>>;
pub type SlicePatternItems<Pat> = InterlaceTrail<Pat, Comma>;

pub type PathPattern<Ty> = PathInExpression<Ty>;
