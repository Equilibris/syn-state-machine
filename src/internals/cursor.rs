use std::num::NonZeroUsize;

use proc_macro2::extra::DelimSpan;
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Span, TokenStream, TokenTree};

use crate::{Error, ParseBuffer, ParserCursor, Spanned};

// Copied from syn with very slight changes
#[derive(Debug)]
pub enum Entry {
    Group(Group, usize),
    Ident(Ident),
    Punct(Punct),
    Literal(Literal),
    End(usize),
}

impl TokenBuffer {
    fn recursive_new(entries: &mut Vec<Entry>, stream: TokenStream) {
        for tt in stream {
            match tt {
                TokenTree::Ident(ident) => entries.push(Entry::Ident(ident)),
                TokenTree::Punct(punct) => entries.push(Entry::Punct(punct)),
                TokenTree::Literal(literal) => entries.push(Entry::Literal(literal)),

                TokenTree::Group(group) => {
                    let group_start_index = entries.len();
                    entries.push(Entry::End(0));

                    Self::recursive_new(entries, group.stream());
                    let group_end_index = entries.len();

                    entries.push(Entry::End(group_end_index - group_start_index));

                    let group_end_offset = group_end_index - group_start_index;
                    entries[group_start_index] = Entry::Group(group, group_end_offset);
                }
            }
        }
    }

    pub fn new(stream: TokenStream) -> Self {
        let mut entries = Vec::new();
        Self::recursive_new(&mut entries, stream);
        entries.push(Entry::End(entries.len() + 1));

        Self {
            entries: entries.into_boxed_slice(),
        }
    }
    pub fn begin<'a>(&'a self) -> RustCursor<'a> {
        RustCursor {
            buf: self,
            current: 0,
            end: self.entries.len() - 1,
        }
    }
}

#[derive(Debug)]
pub struct TokenBuffer {
    pub(crate) entries: Box<[Entry]>,
}

impl From<TokenStream> for TokenBuffer {
    fn from(value: TokenStream) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RustCursor<'a> {
    buf: &'a TokenBuffer,
    pub current: usize,
    pub end: usize,
}

impl<'a> RustCursor<'a> {
    pub fn entry(self) -> &'a Entry {
        &self.buf.entries[self.current]
    }

    pub fn next(self) -> RustCursor<'a> {
        let Self { buf, current, end } = self;
        Self {
            buf,
            current: current + 1,
            end,
        }
    }
    pub fn skip_to_end(self) -> RustCursor<'a> {
        let Self {
            buf,
            current: _,
            end,
        } = self;
        Self {
            buf,
            current: end,
            end,
        }
    }
    pub fn prev(self) -> RustCursor<'a> {
        let Self { buf, current, end } = self;
        Self {
            buf,
            current: if current > 0 { current - 1 } else { 0 },
            end,
        }
    }

    pub fn skip(self, count: usize) -> RustCursor<'a> {
        let Self { buf, current, end } = self;

        Self {
            buf,
            end,
            current: current + count,
        }
    }

    /// While the cursor is looking at a `None`-delimited group, move it to look
    /// at the first token inside instead. If the group is empty, this will move
    /// the cursor past the `None`-delimited group.
    ///
    /// WARNING: This mutates its argument.
    fn ignore_none(&mut self) {
        while let Entry::Group(group, _) = self.entry() {
            if group.delimiter() == Delimiter::None {
                let _ = self.advance_by(1);
            } else {
                break;
            }
        }
    }

    pub fn remaining(&self) -> usize {
        self.end - self.current
    }

    pub fn eof(self) -> bool {
        self.current == self.end
    }

    pub fn group(
        mut self,
        delim: Delimiter,
    ) -> Option<(RustCursor<'a>, DelimSpan, RustCursor<'a>)> {
        // If we're not trying to enter a none-delimited group, we want to
        // ignore them. We have to make sure to _not_ ignore them when we want
        // to enter them, of course. For obvious reasons.
        if delim != Delimiter::None {
            self.ignore_none();
        }

        if let Entry::Group(group, end_offset) = self.entry() {
            if group.delimiter() == delim {
                let Self { buf, current, end } = self;

                let span = group.delim_span();

                let end_of_group = self.current + end_offset;

                let inside_of_group = Self {
                    buf,
                    current: current + 1,
                    end: end_of_group,
                };
                let after_group = Self {
                    buf,
                    current: end_of_group + 1,
                    end,
                };
                return Some((inside_of_group, span, after_group));
            }
        }

        None
    }

    pub fn any_group(self) -> Option<(RustCursor<'a>, Delimiter, DelimSpan, RustCursor<'a>)> {
        if let Entry::Group(group, end_offset) = self.entry() {
            let Self { buf, current, end } = self;
            let delim = group.delimiter();

            let span = group.delim_span();

            let end_of_group = self.current + end_offset;

            let inside_of_group = Self {
                buf,
                current: current + 1,
                end: end_of_group,
            };
            let after_group = Self {
                buf,
                current: end_of_group + 1,
                end,
            };
            return Some((inside_of_group, delim, span, after_group));
        }

        None
    }

    pub fn any_group_token(self) -> Option<(Group, RustCursor<'a>)> {
        if let Entry::Group(group, end_offset) = self.entry() {
            let Self { buf, current, end } = self;

            let end_of_group = current + *end_offset;
            let after_group = Self {
                buf,
                current: end_of_group + 1,
                end,
            };
            return Some((group.clone(), after_group));
        }

        None
    }

    /// If the cursor is pointing at a `Ident`, returns it along with a cursor
    /// pointing at the next `TokenTree`.
    pub fn ident(mut self) -> Option<(&'a Ident, RustCursor<'a>)> {
        self.ignore_none();
        match self.entry() {
            Entry::Ident(ident) => Some((ident, self.next())),
            _ => None,
        }
    }

    /// If the cursor is pointing at a `Punct`, returns it along with a cursor
    /// pointing at the next `TokenTree`.
    pub fn punct(mut self) -> Option<(&'a Punct, RustCursor<'a>)> {
        self.ignore_none();
        match self.entry() {
            Entry::Punct(punct) => Some((punct, self.next())),
            _ => None,
        }
    }

    /// If the cursor is pointing at a `Literal`, return it along with a cursor
    /// pointing at the next `TokenTree`.
    pub fn literal(mut self) -> Option<(&'a Literal, RustCursor<'a>)> {
        self.ignore_none();
        match self.entry() {
            Entry::Literal(literal) => Some((literal, self.next())),
            _ => None,
        }
    }

    pub fn token_stream(self) -> TokenStream {
        let mut tts = Vec::new();
        let mut cursor = self;
        while let Some((tt, rest)) = cursor.token_tree() {
            tts.push(tt);
            cursor = rest;
        }
        tts.into_iter().collect()
    }

    pub fn token_tree(self) -> Option<(TokenTree, RustCursor<'a>)> {
        let (tree, len) = match self.entry() {
            Entry::Group(group, end_offset) => (group.clone().into(), *end_offset),
            Entry::Literal(literal) => (literal.clone().into(), 1),
            Entry::Ident(ident) => (ident.clone().into(), 1),
            Entry::Punct(punct) => (punct.clone().into(), 1),
            Entry::End(_) => return None,
        };

        let rest = Self {
            buf: self.buf,
            current: self.current + len,
            end: self.end,
        };
        Some((tree, rest))
    }
}

impl<'a> ParserCursor for RustCursor<'a> {
    type Error = Error;
}
impl<'a> Spanned for RustCursor<'a> {
    type Loc = Span;

    fn span(&self) -> Span {
        match self.entry() {
            Entry::Group(group, _) => group.delim_span().open(),
            Entry::Literal(literal) => literal.span(),
            Entry::Ident(ident) => ident.span(),
            Entry::Punct(punct) => punct.span(),
            Entry::End(step_back) => {
                if step_back > &self.current {
                    Span::call_site()
                } else {
                    let Entry::Group(ref g, _) = self.buf.entries[self.current - step_back] else {
                        panic!("{:#?}", self.buf.entries[self.current - step_back]);
                    };

                    g.delim_span().close()
                }
            }
        }
    }
}

impl<'a> Iterator for RustCursor<'a> {
    type Item = TokenTree;

    fn next(&mut self) -> Option<Self::Item> {
        let Some((a, b)) = self.token_tree() else {
            return None;
        };

        *self = b;

        Some(a)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining(), None)
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.remaining()
    }

    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        let Self { buf, current, end } = self;

        if current < end {
            Self {
                current: end - 1,
                end,
                buf,
            }
            .token_tree()
            .map(|v| v.0)
        } else {
            None
        }
    }

    fn advance_by(&mut self, n: usize) -> std::result::Result<(), std::num::NonZeroUsize> {
        self.current += n;

        if self.current > self.end {
            Err(unsafe { NonZeroUsize::new_unchecked(self.current - self.end) })
        } else {
            Ok(())
        }
    }
}

pub trait ParseBufExt<'a> {
    fn ident_matching<Pred: FnOnce(&'a Ident) -> Result<(), Error>>(
        &mut self,
        pred: Pred,
    ) -> Result<&'a Ident, Error>;
}

impl<'a> ParseBufExt<'a> for ParseBuffer<RustCursor<'a>> {
    fn ident_matching<Pred: FnOnce(&'a Ident) -> Result<(), Error>>(
        &mut self,
        pred: Pred,
    ) -> Result<&'a Ident, Error> {
        match self.cursor.ident() {
            Some((val, cur)) => {
                if let Err(e) = pred.call_once((val,)) {
                    Err(e)
                } else {
                    self.cursor = cur;
                    Ok(val)
                }
            }
            None => Err(Error::new(self.span(), "Expected Ident")),
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::Spanned;

    use super::TokenBuffer;

    #[test]
    fn it_passes_simple_tests() {
        let d = TokenBuffer::new(quote!(hello world));

        let c = d.begin();

        assert_eq!(c.ident().unwrap().0.to_string().as_str(), "hello");
        let c = c.next();
        assert_eq!(c.ident().unwrap().0.to_string().as_str(), "world");
        let c = c.next();
        assert!(c.ident().is_none());
    }

    #[test]
    fn it_matches_simple_groups() {
        let d = TokenBuffer::new(quote!(hello (world) hi));

        let c = d.begin();

        assert_eq!(c.ident().unwrap().0.to_string().as_str(), "hello");
        let c = c.next();

        let (g, _, c) = c.group(proc_macro2::Delimiter::Parenthesis).unwrap();

        assert_eq!(g.ident().unwrap().0.to_string().as_str(), "world");
        assert_eq!(c.ident().unwrap().0.to_string().as_str(), "hi");
    }
    #[test]
    fn it_gets_correct_span() {
        let d = TokenBuffer::new(quote!(hello () hi));

        let c = d.begin();

        assert_eq!(c.ident().unwrap().0.to_string().as_str(), "hello");
        let c = c.next();

        let (g, _, c) = c.group(proc_macro2::Delimiter::Parenthesis).unwrap();

        assert_eq!(c.ident().unwrap().0.to_string().as_str(), "hi");

        println!("{:#?}", g.span());
    }
}
