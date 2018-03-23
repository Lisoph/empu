pub enum Token {
    Foo,
}

pub struct Position {
    pub line: usize,
    pub col: usize,
}

pub struct FatToken {
    pub token: Token,
    pub pos: Position,
}

pub enum Error {
    Foo(Position),
}

pub struct FatError {
    pub error: Error,
    pub pos: Position,
}

pub enum Result {
    Success(FatToken),
    Error(FatError),
}

pub struct Lexer<I> {
    input: I,
}

impl<I: Iterator<Item = char>> Iterator for Lexer<I> {
    type Item = Result;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
