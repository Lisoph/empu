use super::lexer;
use super::lexer::{FatToken, Position, Token};
use super::ast;
use super::ast::AstNode;

pub fn parse<I: IntoIterator<Item = FatToken>>(tokens: I) -> Option<Parser<I::IntoIter>> {
    let mut input = tokens.into_iter();
    if let Some(first) = input.next() {
        Some(Parser {
            input,
            cur_token: first,
        })
    } else {
        None
    }
}

pub type ParseResult<T = AstNode> = Result<T, Error>;

pub enum Error {
    UnexpectedEof(Position),
    UnexpectedToken(FatToken),
    ExpectedStringLiteral(FatToken),
    ExpectedIntLiteral(FatToken),
    NegativeInteger(FatToken),
    IntegerNotU8(FatToken),
}

struct Eof(Position);

impl From<Eof> for Error {
    fn from(eof: Eof) -> Error {
        Error::UnexpectedEof(eof.0)
    }
}

pub struct Parser<I> {
    input: I,
    cur_token: FatToken,
}

impl<I: Iterator<Item = FatToken>> Parser<I> {
    fn next_token(&mut self) -> Result<FatToken, Eof> {
        let next = self.input.next();
        if let Some(next) = next {
            self.cur_token = next.clone();
            Ok(next)
        } else {
            Err(Eof(self.cur_token.pos))
        }
    }

    fn next_node(&mut self) -> ParseResult {
        match self.cur_token.token {
            Token::AbsoluteLabel(ref lbl) => {
                Ok(AstNode::LabelDeclaration(ast::Label::Absolute(lbl.clone())))
            }
            Token::RelativeLabel(ref lbl) => {
                Ok(AstNode::LabelDeclaration(ast::Label::Relative(lbl.clone())))
            }
            Token::Directive(_) => self.parse_directive(),
            _ => unimplemented!(),
        }
    }

    fn parse_int_literal(&mut self) -> ParseResult<i64> {
        if let Token::IntLiteral(int) = self.next_token()?.token {
            Ok(int)
        } else {
            Err(Error::ExpectedIntLiteral(self.cur_token.clone()))
        }
    }

    fn parse_usize(&mut self) -> ParseResult<usize> {
        let int = self.parse_int_literal()?;
        if int >= 0 {
            Ok(int as usize)
        } else {
            Err(Error::NegativeInteger(self.cur_token.clone()))
        }
    }

    fn parse_u8(&mut self) -> ParseResult<u8> {
        let int = self.parse_int_literal()?;
        if int >= 0 && int <= 255 {
            Ok(int as u8)
        } else {
            Err(Error::IntegerNotU8(self.cur_token.clone()))
        }
    }

    fn parse_string_literal(&mut self) -> ParseResult<String> {
        if let Token::StringLiteral(string) = self.next_token()?.token {
            Ok(string)
        } else {
            Err(Error::ExpectedStringLiteral(self.cur_token.clone()))
        }
    }

    fn parse_directive(&mut self) -> ParseResult {
        if let Token::Directive(dir) = self.cur_token.token.clone() {
            match dir {
                lexer::Directive::Db => {
                    let num = self.parse_usize()?;
                    let val = self.parse_u8().ok();
                    Ok(AstNode::Directive(ast::Directive::DeclareBytes(num, val)))
                }
                lexer::Directive::Ds => Ok(AstNode::Directive(ast::Directive::DeclareString(
                    self.parse_string_literal()?,
                ))),
            }
        } else {
            Err(Error::UnexpectedToken(self.cur_token.clone()))
        }
    }
}

impl<I: Iterator<Item = FatToken>> Iterator for Parser<I> {
    type Item = ParseResult;
    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(_) = self.next_token() {
            Some(self.next_node())
        } else {
            None
        }
    }
}
