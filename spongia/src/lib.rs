pub mod ast;
pub(crate) mod compound;
pub(crate) mod expressions;
pub mod parser;
pub(crate) mod pre_parsing_hooks;
use std::{iter::Cloned, slice::Iter};

use nom::Needed;
pub use parser::parse;
pub use pre_parsing_hooks::pre_parse;
use tokenizer::tokens::CommentedToken;
pub(crate) mod program;
pub(crate) mod token_parsers;
pub(crate) mod whitespace;

#[derive(Debug, Clone, PartialEq)]
pub struct Input<'a, 'b: 'a>(pub &'b [&'a CommentedToken<'a>]);

impl<'a, 'b> std::ops::Deref for Input<'a, 'b> {
    type Target = &'b [&'a CommentedToken<'a>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> nom::Input for Input<'a, '_> {
    type Item = &'a CommentedToken<'a>;
    type Iter = Cloned<Iter<'a, &'a CommentedToken<'a>>>;
    type IterIndices = std::iter::Enumerate<Self::Iter>;

    fn input_len(&self) -> usize {
        self.0.len()
    }

    fn take(&self, index: usize) -> Self {
        Input(&self.0[0..index])
    }

    fn take_from(&self, index: usize) -> Self {
        Input(&self.0[index..])
    }

    fn take_split(&self, index: usize) -> (Self, Self) {
        let (prefix, suffix) = self.0.split_at(index);
        (Input(suffix), Input(prefix))
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.0.iter().position(|b| predicate(b))
    }

    fn iter_elements(&self) -> Self::Iter {
        self.0.iter().cloned()
    }

    fn iter_indices(&self) -> Self::IterIndices {
        self.iter_elements().enumerate()
    }

    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        if self.0.len() >= count {
            Ok(count)
        } else {
            Err(Needed::new(count - self.0.len()))
        }
    }
}

impl std::fmt::Display for Input<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.split_first() {
            None => Ok(()),
            Some((first, rest)) => {
                f.write_fmt(format_args!("{:?}", first.token))?;
                for token in rest {
                    write!(f, " {:?}", token.token)?;
                }
                Ok(())
            }
        }
    }
}

pub(crate) struct InputForDisplay<'a>(&'a [&'a CommentedToken<'a>]);

impl std::fmt::Display for InputForDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.split_first() {
            None => Ok(()),
            Some((first, rest)) => {
                f.write_fmt(format_args!("{}", first))?;
                for token in rest.iter().take(2) {
                    write!(f, " {}", token)?;
                }
                Ok(())
            }
        }
    }
}
