use crate::*;

materialize! {
    pub struct Pattern<Attr, Ty> {
        leading peek <- Or;

        entries <- MinLength<Interlace<PatternNoTopAlt<Attr, Ty>, Or>>
    }
}

materialize! {
    pub enum PatternNoTopAlt <Attr, Ty> {
        PatternWithoutRange(v <- PatternWithoutRange<Attr, Ty>)
        RangePattern(v <- RangePattern<Ty>)
    }
}

materialize! {
    pub enum PatternWithoutRange<Attr, Ty> {
        LiteralPattern(v <- LiteralPattern)
        IdentifierPattern(v <- IdentifierPattern<Box<Pattern<Attr, Ty>>>)
        WildcardPattern(v <- WildcardPattern)
        RestPattern(v <- RestPattern)
        ReferencePattern(v <- ReferencePattern<Box<Pattern<Attr, Ty>>>)
        StructPattern(v <- StructPattern<Attr, Ty, Box<Pattern<Attr, Ty>>>)
        TupleStructPattern(v <- TupleStructPattern<Attr, Ty>)
        TuplePattern(v <- TuplePattern<Box<Pattern<Attr, Ty>>>)
        GroupedPattern(v <- GroupedPattern<Box<Pattern<Attr, Ty>>>)
        SlicePattern(v <- SlicePattern<Box<Pattern<Attr, Ty>>>)
        PathPattern(v <- PathPattern)
        MacroInvocation(v <- MacroInvocation)
    }
}

materialize! {
    pub enum RangePattern <Ty> {
        RangeToInclusivePattern(v <- RangeToInclusivePattern<Ty>)
        RangeInclusivePattern(v <- RangeInclusivePattern<Ty>)
        ObsoleteRangePattern(v <- ObsoleteRangePattern<Ty>)
        RangeFromPattern(v <- RangeFromPattern<Ty>)
    }
}

materialize! {
    pub struct RangeToInclusivePattern <Ty> {
        <- DotDotEq;
        r <- RangePatternBound<Ty>;
    }
}
materialize! {
    pub struct RangeFromPattern <Ty> {
        l <- RangePatternBound<Ty>;
        <- DotDot;
    }
}
materialize! {
    pub struct RangeInclusivePattern <Ty> {
        l <- RangePatternBound<Ty>;
        <- DotDotEq;
        r <- RangePatternBound<Ty>;
    }
}
materialize! {
    pub struct ObsoleteRangePattern <Ty> {
        l <- RangePatternBound<Ty>;
        <- DotDotDot;
        r <- RangePatternBound<Ty>;
    }
}

materialize! {
    pub enum RangePatternBound <Ty> {
        Char(v <- CharLit)
        Byte(v <- ByteLit)
        Int(v <- SignedIntegerLit)
        Float(v <- SignedFloatLit)
        Path(v <- PathInExpression<Ty>)
    }
}

materialize! {
    pub enum LiteralPattern {
        Bool(v <- bool)
        Char(v <- CharLit)
        Byte(v <- ByteLit)
        String(v <- StringLit)
        ByteString(v <- ByteStringLit)
        Int(v <- SignedIntegerLit)
        Float(v <- SignedFloatLit)
    }
}

materialize! {
    pub struct IdentifierPattern <Pat> {
        r#ref peek <- KwRef;
        r#mut peek <- KwMut;
        id <- Ident : Identifier;
        pat <- Option<Pat> : Option<(At, _)> { pat.map(|v|v.1) }
    }
}
pub type WildcardPattern = FIdent<"_">;
pub type RestPattern = DotDot;

materialize! {
    pub struct ReferencePattern <Pat> {
        <- And;
        double peek <- And;
        r#mut peek <- KwMut;
        pat <- Pat
    }
}

materialize! {
    pub struct StructPattern <Attr, Ty, Pat> {
        path <- PathInExpression<Ty>;
        elements <- StructPatternElements<Attr, Pat>
    }
}

pub struct StructPatternElements<Attr, Pat> {
    pub fields: Interlace<StructPatternField<Attr, Pat>, Comma>,
    pub et_cetera: Option<StructPatternEtCetera<Attr>>,
}

impl<'a, Attr: Parse<'a>, Pat: Parse<'a>> Parse<'a> for StructPatternElements<Attr, Pat> {
    fn parse(input: &mut ParseBuffer<'a>) -> Result<Self> {
        Ok(
            match input.parse::<Sum2<(_, Sum3<Comma, (Comma, _), ()>), _>>()? {
                Sum2::V0((fields, Sum3::V0(_) | Sum3::V2(_))) => Self {
                    fields,
                    et_cetera: None,
                },
                Sum2::V0((fields, Sum3::V1((_, et_cetera)))) => Self {
                    fields,
                    et_cetera: Some(et_cetera),
                },
                Sum2::V1(et_cetera) => Self {
                    fields: Default::default(),
                    et_cetera: Some(et_cetera),
                },
            },
        )
    }
}

materialize! {
    pub enum StructPatternField<Attr, Pat> [ attrs <- Vec<OuterAttribute<Attr>> ] {
        TupleIdx(v <- TupleIndex; <- Colon; p <- Pat)
        Id(v <- Ident : Identifier; <- Colon; p <- Pat)
        Shorthand(r#ref peek <- KwRef; r#mut peek <- KwMut; id <- Ident : Identifier)
    }
}

materialize! {
    pub struct StructPatternEtCetera<Attr> {
        attrs <- Vec<OuterAttribute<Attr>>;
        <- DotDot;
    }
}

materialize! {
    pub struct TupleStructPattern <Ty, Pat> {
        path <- PathInExpression<Ty>;
        items <- TupleStructItems<Pat> : Paren<_> { items.0 }
    }
}

pub type TupleStructItems<Pat> = InterlaceTrail<Pat, Comma>;

pub type TuplePattern<Pat> = Paren<Option<TuplePatternItems<Pat>>>;

materialize! {
    pub enum TuplePatternItems<Pat> {
        Pat(v <- Pat)
        Rest(v <- RestPattern)
        Items(v <- InterlaceTrail<Pat, Comma> : MinLength<_, 2> { v.0 })
    }
}
pub type GroupedPattern<Pat> = Paren<Pat>;
pub type SlicePattern<Pat> = Bracket<SlicePatternItems<Pat>>;
pub type SlicePatternItems<Pat> = InterlaceTrail<Pat, Comma>;

pub type PathPattern = std::convert::Infallible; // TODO
