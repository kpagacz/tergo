use nom::InputTake;
use tokenizer::Token;

impl<'a> InputTake for Token<'a> {
    fn take(&self, count: usize) -> Self {
        todo!()
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        todo!()
    }
}
