use peekmore::PeekMoreIterator;
use super::tokens::Token;

pub struct TokenStream<'a> {
  iter: PeekMoreIterator<Token>
}

impl<'a> TokenStream<'a> {
  pub fn parse(input: Box<str>) -> Self {
    todo!();
  }
}
