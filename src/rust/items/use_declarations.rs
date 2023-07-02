use crate::*;

materialize! {
    #[derive(Debug)]
    pub struct UseDeclaration {
        <- KwUse;
        tree <- UseTree;
        <- Semi;
    }
}

materialize! {
    #[derive(Debug)]
    pub enum UseTree {
        Star(
            path <- Option<Option<SimplePath>> : Option<(_, PeekAsParse<PathSep>)> { path.map(|v|v.0) };
            <- Star;
        )
        Branch(
            path <- Option<Option<SimplePath>> : Option<(_, PeekAsParse<PathSep>)> { path.map(|v|v.0) };
            children <- InterlaceTrail<Box<Self>, Comma> : Brace<_> { children.0 };
        )
        Simple(
            path <- SimplePath;
            r#as <- Option<IdentifierOrUnder> : Option<(KwAs, _)> {r#as.map(|v|v.1)};
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
