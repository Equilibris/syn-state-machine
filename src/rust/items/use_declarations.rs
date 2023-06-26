use crate::*;

materialize! {
    #[derive(Debug)]
    pub struct UseDeclaration {
        <- KwUse;
        tree <- UseTree;
        <- Semi;
    }
}

#[derive(Debug)]
pub enum UseTree {
    Star(Option<Option<SimplePath>>),
    Branch(Option<Option<SimplePath>>, InterlaceTrail<Box<Self>, Comma>),
    Simple(SimplePath, Option<IdentifierOrUnder>),
}

impl Parse for UseTree {
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
        Ok(
            match input.parse::<Sum3<
                (
                    Option<(Option<SimplePath>, PeekAsParse<PathSep>)>,
                    PeekAsParse<Star>,
                ),
                (Option<(Option<SimplePath>, PeekAsParse<PathSep>)>, Brace<_>),
                (SimplePath, Option<(KwAs, _)>),
            >>()? {
                Sum3::V0(a) => Self::Star(a.0.map(|v| v.0)),
                Sum3::V1(a) => Self::Branch(a.0.map(|v| v.0), a.1 .0),
                Sum3::V2(a) => Self::Simple(a.0, a.1.map(|v| v.1)),
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    insta_match_test!(+it_matches_simple_path, UseDeclaration : use hello::world; );
    insta_match_test!(+it_matches_simple_path_as, UseDeclaration : use hello::world as h; );
    insta_match_test!(+it_matches_star_path, UseDeclaration : use hello::*; );
    insta_match_test!(+it_matches_complex_path, UseDeclaration :  use { hello::*, world::hi as Hi, nested::{ hello::world, hi }, }; );
}
