use crate::*;

#[derive(Debug, Clone)]
pub struct FPunct<const PUNCT: char>;

impl<const PUNCT: char> Parsable for FPunct<PUNCT> {
    type StateMachine = FixedPunctMachine<PUNCT>;
}

#[derive(Default, Clone, Debug)]
pub struct FixedPunctMachine<const PUNCT: char>;

#[derive(Debug, Clone, thiserror::Error, Default)]
pub enum FixedPunctError<const PUNCT: char> {
    #[error("Expected punct '{}' but got {}", PUNCT, .0)]
    Val(TokenTree),
    #[default]
    #[error("Expected punct '{}' but got termination", PUNCT)]
    Termination,
}

impl<const PUNCT: char> StateMachine for FixedPunctMachine<PUNCT> {
    type Output = FPunct<PUNCT>;
    type Error = FixedPunctError<PUNCT>;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        match val {
            TokenTree::Punct(p) if p.as_char() == PUNCT => {
                ControlFlow::Break(Ok((FPunct::<PUNCT>, 0)))
            }
            p => ControlFlow::Break(Err(Self::Error::Val(p.clone()))),
        }
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        Err(Self::Error::default())
    }

    #[cfg(feature = "execution-debug")]
    fn inspect(&self, depth: usize) {
        println!("{}FixedPunct {}", "  ".repeat(depth), PUNCT);
    }
}

#[derive(Debug, Clone, thiserror::Error, Default)]
pub enum PunctError {
    #[error("Expected punct but got {}", .0)]
    Val(TokenTree),
    #[default]
    #[error("Expected punct but got termination")]
    Termination,
}

pub use proc_macro2::Punct;
use proc_macro2::Spacing;

impl Parsable for Punct {
    type StateMachine = PunctMachine;
}

#[derive(Default)]
pub struct PunctMachine;

impl StateMachine for PunctMachine {
    type Output = proc_macro2::Punct;
    type Error = PunctError;

    fn drive(self, val: &TokenTree) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
        match val {
            TokenTree::Punct(p) => ControlFlow::Break(Ok((p.clone(), 0))),
            p => ControlFlow::Break(Err(Self::Error::Val(p.clone()))),
        }
    }

    fn terminate(self) -> SmResult<Self::Output, Self::Error> {
        Err(Self::Error::default())
    }

    #[cfg(feature = "execution-debug")]
    fn inspect(&self, depth: usize) {
        println!("{}Punct", "  ".repeat(depth));
    }
}

#[derive(Debug, Clone, thiserror::Error, Default)]
pub enum SpacedFixedPunctError<const PUNCT: char> {
    #[error("Expected spacing {:?} but got {:?}", .0, .1)]
    InvalidSpacing(Spacing, Spacing),

    #[error("Expected punct '{}' but got {}", PUNCT, .0)]
    Val(TokenTree),
    #[default]
    #[error("Expected punct '{}' but got termination", PUNCT)]
    Termination,
}

#[derive(Debug, Clone, thiserror::Error, Default)]
pub enum SpacedPunctError {
    #[error("Expected spacing {:?} but got {:?}", .0, .1)]
    InvalidSpacing(Spacing, Spacing),

    #[error("Expected punct but got {}", .0)]
    Val(TokenTree),
    #[default]
    #[error("Expected punct but got termination")]
    Termination,
}

macro_rules! spaced_punct {
    ($name:ident,
     $fixed_name:ident,
     $machine_name:ident,
     $fixed_machine_name:ident,
     $spacing:path,
     $fixed_e:ident,
     $e:ident) => {
        #[derive(Debug, Clone)]
        pub struct $fixed_name<const PUNCT: char>;

        impl<const PUNCT: char> Parsable for $fixed_name<PUNCT> {
            type StateMachine = $fixed_machine_name<PUNCT>;
        }

        #[derive(Default, Clone, Debug)]
        pub struct $fixed_machine_name<const PUNCT: char>;

        impl<const PUNCT: char> StateMachine for $fixed_machine_name<PUNCT> {
            type Output = $fixed_name<PUNCT>;
            type Error = $fixed_e<PUNCT>;

            fn drive(
                self,
                val: &TokenTree,
            ) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
                match val {
                    TokenTree::Punct(p) if p.as_char() == PUNCT => {
                        if p.spacing() == $spacing {
                            ControlFlow::Break(Ok(($fixed_name::<PUNCT>, 0)))
                        } else {
                            ControlFlow::Break(Err(Self::Error::InvalidSpacing(
                                $spacing,
                                p.spacing(),
                            )))
                        }
                    }
                    p => ControlFlow::Break(Err(Self::Error::Val(p.clone()))),
                }
            }

            fn terminate(self) -> SmResult<Self::Output, Self::Error> {
                Err(Self::Error::default())
            }

            #[cfg(feature = "execution-debug")]
            fn inspect(&self, depth: usize) {
                println!("{}FixedPunct {}", "  ".repeat(depth), PUNCT);
            }
        }
        pub struct $name;
        #[derive(Clone, Default)]
        pub struct $machine_name;

        impl Parsable for $name {
            type StateMachine = $machine_name;
        }

        impl StateMachine for $machine_name {
            type Output = proc_macro2::Punct;
            type Error = $e;

            fn drive(
                self,
                val: &TokenTree,
            ) -> ControlFlow<SmResult<Self::Output, Self::Error>, Self> {
                match val {
                    TokenTree::Punct(p) => {
                        if p.spacing() == $spacing {
                            ControlFlow::Break(Ok((p.clone(), 0)))
                        } else {
                            ControlFlow::Break(Err(Self::Error::InvalidSpacing(
                                $spacing,
                                p.spacing(),
                            )))
                        }
                    }
                    _ => ControlFlow::Break(Err(Self::Error::default())),
                }
            }

            fn terminate(self) -> SmResult<Self::Output, Self::Error> {
                Err(Self::Error::default())
            }

            #[cfg(feature = "execution-debug")]
            fn inspect(&self, depth: usize) {
                println!("{}Punct ", "  ".repeat(depth));
            }
        }
    };
}

spaced_punct!(
    AlonePunct,
    FAlonePunct,
    AlonePunctMachine,
    FixedAlonePunctMachine,
    Spacing::Alone,
    SpacedFixedPunctError,
    SpacedPunctError
);
spaced_punct!(
    JointPunct,
    FJointPunct,
    JointPunctMachine,
    FixedJointPunctMachine,
    Spacing::Joint,
    SpacedFixedPunctError,
    SpacedPunctError
);

#[cfg(test)]
mod tests {
    use crate::*;

    insta_match_test!(it_matches_only, Punct : < );
    insta_match_test!(it_matches_fixed, FPunct<'<'> : < );
    insta_match_test!(it_matches_dollar, FPunct<'$'> : $ );

    insta_match_test!(it_matches_joint, (FJointPunct<'\''>, Ident) : 'hello );
    insta_match_test!(it_matches_alone, (FAlonePunct<'\''>, Ident) : 'hello );
    insta_match_test!(it_matches_both, (FJointPunct<'<'>, FAlonePunct<'='>) : <= );

    insta_match_test!(it_matches_dollar_crate, (FPunct<'$'>,FIdent<"crate">) : $crate);
}
