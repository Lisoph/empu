pub mod lexer;
pub mod parser;
pub mod ast;

pub fn parse<'a, I: IntoIterator<Item = char> + 'a>(
    code: I,
) -> Box<Iterator<Item = lexer::Result> + 'a> {
    Box::new(lexer::Lexer::new(code.into_iter()))
}
