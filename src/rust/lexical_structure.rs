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

pub fn get_error_from_ident_or_under<'a>(id: &'a Ident) -> Result<()> {
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
pub fn get_error_from_ident<'a>(id: &'a Ident) -> Result<()> {
    get_error_from_ident_or_under(id)?;

    if id == "_" {
        Err(Error::new(id.span(), "'_' is a reserved identifier"))
    } else {
        Ok(())
    }
}

#[derive(Debug)]
pub struct IdentifierOrUnder(pub Ident);
impl Parse for IdentifierOrUnder {
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
        Ok(Self(
            input.ident_matching(get_error_from_ident_or_under)?.clone(),
        ))
    }
}

impl Peek for IdentifierOrUnder {
    fn peek<'a>(id: Cursor<'a>) -> Option<usize> {
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

#[derive(Debug)]
pub struct Identifier(pub Ident);
impl Parse for Identifier {
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
        Ok(Self(input.ident_matching(get_error_from_ident)?.clone()))
    }
}

impl Peek for Identifier {
    fn peek<'a>(cursor: Cursor<'a>) -> Option<usize> {
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
pub type AndAnd = (FJointPunct<'&'>, FPunct<'&'>);
pub type OrOr = (FJointPunct<'|'>, FPunct<'|'>);
pub type Shl = (FJointPunct<'<'>, FPunct<'<'>);
pub type Shr = (FJointPunct<'>'>, FPunct<'>'>);
pub type PlusEq = (FJointPunct<'+'>, FPunct<'='>);
pub type MinusEq = (FJointPunct<'-'>, FPunct<'='>);
pub type StarEq = (FJointPunct<'*'>, FPunct<'='>);
pub type SlashEq = (FJointPunct<'/'>, FPunct<'='>);
pub type PercentEq = (FJointPunct<'%'>, FPunct<'='>);
pub type CaretEq = (FJointPunct<'^'>, FPunct<'='>);
pub type AndEq = (FJointPunct<'&'>, FPunct<'='>);
pub type OrEq = (FJointPunct<'|'>, FPunct<'='>);
pub type ShlEq = (FJointPunct<'<'>, FJointPunct<'<'>, FPunct<'='>);
pub type ShrEq = (FJointPunct<'>'>, FJointPunct<'>'>, FPunct<'='>);
pub type Eq = FPunct<'='>;
pub type EqEq = (FJointPunct<'='>, FPunct<'='>);
pub type Ne = (FJointPunct<'!'>, FPunct<'='>);
pub type Gt = FPunct<'>'>;
pub type Lt = FPunct<'<'>;
pub type Ge = (FJointPunct<'>'>, FPunct<'='>);
pub type Le = (FJointPunct<'<'>, FPunct<'='>);
pub type At = FPunct<'@'>;
pub type Dot = FPunct<'.'>;
pub type DotDot = (FJointPunct<'.'>, FPunct<'.'>);
pub type DotDotDot = (FJointPunct<'.'>, FJointPunct<'.'>, FPunct<'.'>);
pub type DotDotEq = (FJointPunct<'.'>, FJointPunct<'.'>, FPunct<'='>);
pub type Comma = FPunct<','>;
pub type Semi = FPunct<';'>;
pub type Colon = FPunct<':'>;
pub type PathSep = (FJointPunct<':'>, FPunct<':'>);
pub type RArrow = (FJointPunct<'-'>, FPunct<'>'>);
pub type FatArrow = (FJointPunct<'='>, FPunct<'>'>);
pub type Pound = FPunct<'#'>;
pub type Dollar = FPunct<'$'>;
pub type Question = FPunct<'?'>;
pub type Tilde = FPunct<'~'>;

#[derive(Debug)]
pub struct LifetimeToken(pub Ident);
impl Parse for LifetimeToken {
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
        input.errored_peek::<FPunct<'\''>>()?;

        Ok(Self(input.parse()?))
    }
}

impl Peek for LifetimeToken {
    fn peek<'a>(input: Cursor<'a>) -> Option<usize> {
        <(FPunct<'\''>, Ident)>::peek(input)
    }
}
impl PeekError for LifetimeToken {
    fn error<'a>(input: Cursor<'a>) -> Error {
        <(FPunct<'\''>, Ident)>::error(input)
    }
}
impl FixedPeek for LifetimeToken {
    const SKIP: usize = 2;
}

#[derive(Debug)]
pub struct LifetimeOrLabel(pub Identifier);
impl Parse for LifetimeOrLabel {
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
        input.errored_peek::<FPunct<'\''>>()?;

        Ok(Self(input.parse()?))
    }
}

impl Peek for LifetimeOrLabel {
    fn peek<'a>(input: Cursor<'a>) -> Option<usize> {
        <(FPunct<'\''>, Ident)>::peek(input)
    }
}
impl PeekError for LifetimeOrLabel {
    fn error<'a>(input: Cursor<'a>) -> Error {
        <(FPunct<'\''>, Ident)>::error(input)
    }
}
impl FixedPeek for LifetimeOrLabel {
    const SKIP: usize = 2;
}

#[cfg(test)]
mod tests {
    use crate::*;
    insta_match_test!(it_matches_lifetime, LifetimeToken : 'hi);
}
