use crate::*;
#[cfg(feature = "printing")]
use quote::TokenStreamExt;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct UseDeclaration {
        <- KwUse;
        tree <- UseTree;
        <- Semi;
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct UseDeclaration {
        <- KwUse;
        tree <- UseTree;
        <- Semi;
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum UseTree {
        Star(
            path <- Option<SimplePath> : Option<(_, PeekAsParse<PathSep>)> { path.map(|v|v.0) };
            <- Star;
        ),
        Branch(
            path <- Option<SimplePath> : Option<(_, PeekAsParse<PathSep>)> { path.map(|v|v.0) };
            children <- InterlaceTrail<Box<Self>, Comma> : Brace<_> { children.0 };
        ),
        Simple(
            path <- SimplePath;
            r#as <- Option<IdentifierOrUnder> : Option<(KwAs, _)> {r#as.map(|v|v.1)};
        )
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for enum UseTree {
        Star(
            path <- tokens into {
                if let Some(path) = path {
                    tokens.extend(path.into_token_stream());
                    tokens.extend(PathSep::default().into_token_stream());
                }
            } to {
                if let Some(path) = path {
                    path.to_tokens(tokens);
                    tokens.extend(PathSep::default().into_token_stream());
                }
            };
            <- Star;
        ),
        Branch(
            path <- tokens into {
                if let Some(path) = path {
                    tokens.extend(path.into_token_stream());
                    tokens.extend(PathSep::default().into_token_stream());
                }
            } to {
                if let Some(path) = path {
                    path.to_tokens(tokens);
                    tokens.extend(PathSep::default().into_token_stream());
                }
            };
            children <- tokens into {
                tokens.append(
                    proc_macro2::Group::new(
                        proc_macro2::Delimiter::Brace,
                        children.into_token_stream()
                    )
                );
            } to {
                tokens.append(
                    proc_macro2::Group::new(
                        proc_macro2::Delimiter::Brace,
                        children.to_token_stream()
                    )
                );
            }
        ),
        Simple(
            path <- SimplePath;
            r#as <- tokens into {
                if let Some(r#as) = r#as {
                    tokens.extend(KwAs::default().into_token_stream());
                    tokens.extend(as_ty.into_token_stream())
                }
            } to {
                if let Some(r#as) = r#as {
                    tokens.extend(KwAs::default().into_token_stream());
                    r#as.to_tokens(tokens)
                }
            };
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
