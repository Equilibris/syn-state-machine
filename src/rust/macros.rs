use crate::*;

materialize! {
    #[derive(Debug)]
    pub struct MacroInvocation {
        path <- SimplePath;
        <- Not;
        tt <- DelimTokenTree;
    }
}

materialize! {
    #[derive(Debug)]
    pub struct DelimTokenTree {
        stream <- TokenStream : Sum3<Paren<_>,Brace<_>,Bracket<_>> {
            match stream {
                Sum3::V0(a) => a.0,
                Sum3::V1(a) => a.0,
                Sum3::V2(a) => a.0
            }
        }
    }
}

materialize! {
    #[derive(Debug)]
    pub struct MacroInvocationSemi {
        path <- SimplePath;
        <- Not;
        stream <- TokenStream : Sum3<(Paren<_>,PeekAsParse<Semi>),Brace<(_,PeekAsParse<Semi>)>,Bracket<_>> {
            match stream {
                Sum3::V0(a) => a.0.0,
                Sum3::V1(a) => a.0.0,
                Sum3::V2(a) => a.0
            }
        }
    }
}
