use crate::*;

pub type KwAs = FIdent<"as">;
pub type KwBreak = FIdent<"break">;
pub type KwConst = FIdent<"const">;
pub type KwContinue = FIdent<"continue">;
pub type KwCrate = FIdent<"crate">;
pub type KwElse = FIdent<"else">;
pub type KwEnum = FIdent<"enum">;
pub type KwExtern = FIdent<"extern">;
pub type KwFalse = FIdent<"false">;
pub type KwFn = FIdent<"fn">;
pub type KwFor = FIdent<"for">;
pub type KwIf = FIdent<"if">;
pub type KwImpl = FIdent<"impl">;
pub type KwIn = FIdent<"in">;
pub type KwLet = FIdent<"let">;
pub type KwLoop = FIdent<"loop">;
pub type KwMatch = FIdent<"match">;
pub type KwMod = FIdent<"mod">;
pub type KwMove = FIdent<"move">;
pub type KwMut = FIdent<"mut">;
pub type KwPub = FIdent<"pub">;
pub type KwRef = FIdent<"ref">;
pub type KwReturn = FIdent<"return">;
pub type KwLowerSelf = FIdent<"self">;
pub type KwUpperSelf = FIdent<"Self">;
pub type KwStatic = FIdent<"static">;
pub type KwStruct = FIdent<"struct">;
pub type KwSuper = FIdent<"super">;
pub type KwTrait = FIdent<"trait">;
pub type KwTrue = FIdent<"true">;
pub type KwType = FIdent<"type">;
pub type KwUnsafe = FIdent<"unsafe">;
pub type KwUse = FIdent<"use">;
pub type KwWhere = FIdent<"where">;
pub type KwWhile = FIdent<"while">;
pub type KwAsync = FIdent<"async">;
pub type KwAwait = FIdent<"await">;
pub type KwDyn = FIdent<"dyn">;
pub type KwUnion = FIdent<"union">;
pub type KwTry = FIdent<"try">;
pub type KwAbstract = FIdent<"abstract">;
pub type KwBecome = FIdent<"become">;
pub type KwBox = FIdent<"box">;
pub type KwDo = FIdent<"do">;
pub type KwFinal = FIdent<"final">;
pub type KwMacro = FIdent<"macro">;
pub type KwOverride = FIdent<"override">;
pub type KwPriv = FIdent<"priv">;
pub type KwTypeof = FIdent<"typeof">;
pub type KwUnsized = FIdent<"unsized">;
pub type KwVirtual = FIdent<"virtual">;
pub type KwYield = FIdent<"yield">;

pub fn get_error_from_ident_or_under(id: &Ident) -> Result<(), Error> {
    if id == "r#crate" {
        Err(Error::new(id.span(), "'r#crate' is not a valid identifier"))
    } else if id == "r#super" {
        Err(Error::new(id.span(), "'r#super' is not a valid identifier"))
    } else if id == "r#self" {
        Err(Error::new(id.span(), "'r#self' is not a valid identifier"))
    } else if id == "r#Self" {
        Err(Error::new(id.span(), "'r#Self' is not a valid identifier"))
    } else {
        Ok(())
    }
}
pub fn get_error_from_ident(id: &Ident) -> Result<(), Error> {
    get_error_from_ident_or_under(id)?;

    if id == "_" {
        Err(Error::new(id.span(), "'_' is a reserved identifier"))
    } else {
        Ok(())
    }
}

#[derive(Debug)]
pub struct IdentifierOrUnder(pub Ident);
impl<'a> Parse<RustCursor<'a>> for IdentifierOrUnder {
    fn parse(input: &mut ParseBuffer<RustCursor<'a>>) -> Result<Self, Error> {
        Ok(Self(
            input.ident_matching(get_error_from_ident_or_under)?.clone(),
        ))
    }
}

impl<'a> Peek<RustCursor<'a>> for IdentifierOrUnder {
    fn peek(id: &RustCursor) -> Option<usize> {
        match id.ident() {
            Some((id, _)) => {
                if id == "r#crate" || id == "r#super" || id == "r#self" || id == "r#Self" {
                    None
                } else {
                    Some(1)
                }
            }
            None => None,
        }
    }
}
#[cfg(feature = "printing")]
impl quote::ToTokens for IdentifierOrUnder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }

    fn to_token_stream(&self) -> TokenStream {
        self.0.to_token_stream()
    }

    fn into_token_stream(self) -> TokenStream
    where
        Self: Sized,
    {
        self.0.into_token_stream()
    }
}

impl From<IdentifierOrUnder> for Ident {
    fn from(val: IdentifierOrUnder) -> Self {
        val.0
    }
}

#[derive(Debug)]
pub struct Identifier(pub Ident);
impl<'a> Parse<RustCursor<'a>> for Identifier {
    fn parse(input: &mut ParseBuffer<RustCursor<'a>>) -> Result<Self, Error> {
        Ok(Self(input.ident_matching(get_error_from_ident)?.clone()))
    }
}

#[cfg(feature = "printing")]
impl quote::ToTokens for Identifier {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }

    fn to_token_stream(&self) -> TokenStream {
        self.0.to_token_stream()
    }

    fn into_token_stream(self) -> TokenStream
    where
        Self: Sized,
    {
        self.0.into_token_stream()
    }
}

impl From<Identifier> for Ident {
    fn from(val: Identifier) -> Self {
        val.0
    }
}

impl<'a> Peek<RustCursor<'a>> for Identifier {
    fn peek(cursor: &RustCursor) -> Option<usize> {
        match cursor.ident() {
            Some((id, _)) => {
                if id == "r#crate"
                    || id == "r#super"
                    || id == "r#self"
                    || id == "r#Self"
                    || id == "_"
                {
                    None
                } else {
                    Some(1)
                }
            }
            None => None,
        }
    }
}

pub type Plus = FPunct<'+'>;
pub type Minus = FPunct<'-'>;
pub type Star = FPunct<'*'>;
pub type Slash = FPunct<'/'>;
pub type Percent = FPunct<'%'>;
pub type Caret = FPunct<'^'>;
pub type Not = FPunct<'!'>;
pub type And = FPunct<'&'>;
pub type Or = FPunct<'|'>;
pub type AndAnd = P2<FJointPunct<'&'>, FPunct<'&'>>;
pub type OrOr = P2<FJointPunct<'|'>, FPunct<'|'>>;
pub type Shl = P2<FJointPunct<'<'>, FPunct<'<'>>;
pub type Shr = P2<FJointPunct<'>'>, FPunct<'>'>>;
pub type PlusEq = P2<FJointPunct<'+'>, FPunct<'='>>;
pub type MinusEq = P2<FJointPunct<'-'>, FPunct<'='>>;
pub type StarEq = P2<FJointPunct<'*'>, FPunct<'='>>;
pub type SlashEq = P2<FJointPunct<'/'>, FPunct<'='>>;
pub type PercentEq = P2<FJointPunct<'%'>, FPunct<'='>>;
pub type CaretEq = P2<FJointPunct<'^'>, FPunct<'='>>;
pub type AndEq = P2<FJointPunct<'&'>, FPunct<'='>>;
pub type OrEq = P2<FJointPunct<'|'>, FPunct<'='>>;
pub type ShlEq = P3<FJointPunct<'<'>, FJointPunct<'<'>, FPunct<'='>>;
pub type ShrEq = P3<FJointPunct<'>'>, FJointPunct<'>'>, FPunct<'='>>;
pub type Eq = FPunct<'='>;
pub type EqEq = P2<FJointPunct<'='>, FPunct<'='>>;
pub type Ne = P2<FJointPunct<'!'>, FPunct<'='>>;
pub type Gt = FPunct<'>'>;
pub type Lt = FPunct<'<'>;
pub type Ge = P2<FJointPunct<'>'>, FPunct<'='>>;
pub type Le = P2<FJointPunct<'<'>, FPunct<'='>>;
pub type At = FPunct<'@'>;
pub type Dot = FPunct<'.'>;
pub type DotDot = P2<FJointPunct<'.'>, FPunct<'.'>>;
pub type DotDotDot = P3<FJointPunct<'.'>, FJointPunct<'.'>, FPunct<'.'>>;
pub type DotDotEq = P3<FJointPunct<'.'>, FJointPunct<'.'>, FPunct<'='>>;
pub type Comma = FPunct<','>;
pub type Semi = FPunct<';'>;
pub type Colon = FPunct<':'>;
pub type PathSep = P2<FJointPunct<':'>, FPunct<':'>>;
pub type RArrow = P2<FJointPunct<'-'>, FPunct<'>'>>;
pub type FatArrow = P2<FJointPunct<'='>, FPunct<'>'>>;
pub type Pound = FPunct<'#'>;
pub type Dollar = FPunct<'$'>;
pub type Question = FPunct<'?'>;
pub type Tilde = FPunct<'~'>;

materialize! {
    on <'a> [RustCursor<'a>]
    #[derive(Debug)]
    pub struct LifetimeToken {
        <- FPunct<'\''>;
        ident <- Ident
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct LifetimeToken {
        <- FPunct<'\''>;
        ident <- Ident
    }
}

impl<'a> Peek<RustCursor<'a>> for LifetimeToken {
    fn peek(input: &RustCursor) -> Option<usize> {
        <(FPunct<'\''>, Ident)>::peek(input)
    }
}
impl<'a> PeekError<RustCursor<'a>> for LifetimeToken {
    fn error(input: &RustCursor) -> Error {
        <(FPunct<'\''>, Ident)>::error(input)
    }
}
impl FixedPeek for LifetimeToken {
    const SKIP: usize = 2;
}

materialize! {
    on <'a> [RustCursor<'a>]
    #[derive(Debug)]
    pub struct LifetimeOrLabel{
        <- FPunct<'\''>;
        ident <- Ident : Identifier
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct LifetimeOrLabel {
        <- FPunct<'\''>;
        ident <- Ident
    }
}

impl<'a> Peek<RustCursor<'a>> for LifetimeOrLabel {
    fn peek(input: &RustCursor) -> Option<usize> {
        <(FPunct<'\''>, Ident)>::peek(input)
    }
}
impl<'a> PeekError<RustCursor<'a>> for LifetimeOrLabel {
    fn error(input: &RustCursor) -> Error {
        <(FPunct<'\''>, Ident)>::error(input)
    }
}
impl FixedPeek for LifetimeOrLabel {
    const SKIP: usize = 2;
}

pub type TupleIndex = IntegerLit;

#[cfg(test)]
mod tests {
    use crate::*;
    insta_match_test!(it_matches_lifetime, LifetimeToken : 'hi);
}
