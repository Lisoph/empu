use std::str::FromStr;
use std::result::Result as StdResult;

#[derive(Debug)]
pub enum Token {
    Instruction(Instruction),
    Directive(Directive),
    Unit(Unit),
    At,
    Comma,
    AbsoluteLabel(String),
    RelativeLabel(String),
    IntLiteral(i64),
    StringLiteral(String),
    LabelReference(String),
}

#[derive(Debug)]
pub enum Instruction {
    Mov,
    Add,
    Sub,
    Mul,
    Div,
    Cmp,
    Jg,
    Je,
    Jl,
    Jmp,
    Int,
    Iret,
    And,
    Or,
    Xor,
    Not,
    Shl,
    Shr,
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mov" => Ok(Instruction::Mov),
            "add" => Ok(Instruction::Add),
            "sub" => Ok(Instruction::Sub),
            "mul" => Ok(Instruction::Mul),
            "div" => Ok(Instruction::Div),
            "cmp" => Ok(Instruction::Cmp),
            "jg" => Ok(Instruction::Jg),
            "je" => Ok(Instruction::Je),
            "jl" => Ok(Instruction::Jl),
            "jmp" => Ok(Instruction::Jmp),
            "int" => Ok(Instruction::Int),
            "iret" => Ok(Instruction::Iret),
            "and" => Ok(Instruction::And),
            "or" => Ok(Instruction::Or),
            "xor" => Ok(Instruction::Xor),
            "not" => Ok(Instruction::Not),
            "shl" => Ok(Instruction::Shl),
            "shr" => Ok(Instruction::Shr),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum Directive {
    Db,
    Ds,
}

impl FromStr for Directive {
    type Err = ();
    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "db" => Ok(Directive::Db),
            "ds" => Ok(Directive::Ds),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum Unit {
    Byte,
    Word,
    Dword,
}

impl FromStr for Unit {
    type Err = ();
    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "byte" => Ok(Unit::Byte),
            "word" => Ok(Unit::Word),
            "dword" => Ok(Unit::Dword),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

impl Position {
    fn update(&mut self, c: char) {
        if c == '\n' {
            self.line += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
    }
}

#[derive(Debug)]
pub struct FatToken {
    pub token: Token,
    pub pos: Position,
}

#[derive(Debug)]
pub enum Error {
    WrongCharacter(char, char),
    MissingCharacter(char),
    UnexpectedNewline,
    NotADigit,
    UnexpectedEof,
    InvalidCharacter,
}

#[derive(Debug)]
pub struct FatError {
    pub error: Error,
    pub pos: Position,
}

#[derive(Debug)]
pub enum Result {
    Success(FatToken),
    Error(FatError),
}

impl Result {
    fn token(pos: Position, token: Token) -> Self {
        Result::Success(FatToken { token, pos })
    }

    fn error(pos: Position, error: Error) -> Self {
        Result::Error(FatError { error, pos })
    }
}

pub struct Lexer<I> {
    input: I,
    cur_pos: Position,
    cur_char: char,
    eof_hit: bool,
}

impl<I: Iterator<Item = char>> Lexer<I> {
    pub fn new(input: I) -> Self {
        Self {
            input,
            cur_pos: Position { line: 0, col: 0 },
            cur_char: '\0', // To signal the initial iteration
            eof_hit: false,
        }
    }

    fn try_next_input(&mut self) -> Option<char> {
        if let Some(c) = self.input.next() {
            self.cur_char = c;
            self.cur_pos.update(c);
            Some(c)
        } else {
            self.eof_hit = true;
            None
        }
    }

    fn next_input(&mut self) {
        let _ = self.try_next_input();
    }

    fn collect_while<F2>(
        &mut self,
        init_predicate: Option<&Fn(char) -> bool>,
        predicate: F2,
    ) -> String
    where
        F2: Fn(char) -> bool,
    {
        let mut string = String::new();
        let mut predi = init_predicate.unwrap_or(&predicate);
        loop {
            if let Some(c) = self.try_next_input() {
                if predi(c) {
                    string.push(c);
                } else {
                    break string;
                }
            } else {
                break string;
            }
            predi = &predicate;
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.eof_hit && self.cur_char.is_whitespace() {
            self.next_input();
        }
    }

    fn skip_line(&mut self) {
        while !self.eof_hit && self.cur_char != '\n' {
            self.next_input();
        }
        self.next_input();
    }

    fn next_token(&mut self) -> Result {
        match self.cur_char {
            '@' => {
                let cur_pos = self.cur_pos;
                self.next_input();
                Result::token(cur_pos, Token::At)
            }
            ',' => {
                let cur_pos = self.cur_pos;
                self.next_input();
                Result::token(cur_pos, Token::Comma)
            }
            '"' => self.parse_string_literal(),
            '.' => self.parse_relative_label(),
            c if c.is_digit(10) || c == '-' || c == '+' => self.parse_int_literal(),
            c if c.is_alphabetic() => {
                let start = self.cur_pos;
                // TODO: Inefficient. Turn collect_while into an interator?
                let string = c.to_string() + &self.collect_while(None, |c| {
                    c.is_alphabetic() || c.is_digit(10) || c == '_'
                });
                if let Ok(dir) = Directive::from_str(&string) {
                    Result::token(start, Token::Directive(dir))
                } else if let Ok(ins) = Instruction::from_str(&string) {
                    Result::token(start, Token::Instruction(ins))
                } else if let Ok(unit) = Unit::from_str(&string) {
                    Result::token(start, Token::Unit(unit))
                } else if self.cur_char == ':' {
                    self.next_input();
                    Result::token(start, Token::AbsoluteLabel(string))
                } else {
                    Result::token(start, Token::LabelReference(string))
                }
            }
            _ => Result::error(self.cur_pos, Error::InvalidCharacter),
        }
    }

    fn parse_string_literal(&mut self) -> Result {
        // TODO: Escape sequences
        if self.cur_char == '"' {
            let start = self.cur_pos;
            let string = self.collect_while(None, |c| c != '\n' && c != '"');
            let end_char = self.cur_char;
            self.next_input();
            match end_char {
                '"' => Result::token(start, Token::StringLiteral(string)),
                '\n' => Result::error(self.cur_pos, Error::UnexpectedNewline),
                _ if self.eof_hit => Result::error(self.cur_pos, Error::MissingCharacter('"')),
                c => Result::error(self.cur_pos, Error::WrongCharacter(c, '"')),
            }
        } else {
            Result::error(self.cur_pos, Error::WrongCharacter(self.cur_char, '"'))
        }
    }

    fn parse_relative_label(&mut self) -> Result {
        if self.cur_char == '.' {
            let start = self.cur_pos;
            let string = self.collect_while(Some(&|c: char| c.is_alphabetic() || c == '_'), |c| {
                c.is_alphabetic() || c.is_digit(10) || c == '_'
            });
            match self.cur_char {
                ':' => {
                    self.next_input();
                    Result::token(start, Token::RelativeLabel(string))
                }
                c => Result::error(self.cur_pos, Error::WrongCharacter(c, ':')),
            }
        } else {
            Result::error(self.cur_pos, Error::WrongCharacter(self.cur_char, '.'))
        }
    }

    fn parse_int_literal(&mut self) -> Result {
        let mut start_character = self.cur_char;
        let start = self.cur_pos;
        let prefix = if start_character == '+' || start_character == '-' {
            let prefix = start_character;
            if let Some(c) = self.try_next_input() {
                start_character = c;
            } else {
                return Result::error(self.cur_pos, Error::UnexpectedEof);
            }
            Some(prefix)
        } else {
            None
        };

        if let Some(next) = self.try_next_input() {
            let cur_pos = self.cur_pos;

            let mut parse_digits = |radix: u32, start_digit: u32| -> Result {
                let mut val = start_digit as i64;
                while let Some(c) = self.try_next_input() {
                    if let Some(d) = c.to_digit(radix) {
                        val = val * radix as i64 + d as i64;
                    } else {
                        break;
                    }
                }

                if let Some(prefix) = prefix {
                    val *= if prefix == '-' { -1 } else { 1 };
                }
                Result::token(start, Token::IntLiteral(val))
            };

            match (start_character, next) {
                ('0', 'x') => parse_digits(16, 0),
                ('0', 'o') => parse_digits(8, 0),
                ('0', 'b') => parse_digits(2, 0),
                (c, 'x') | (c, 'o') | (c, 'b') => {
                    Result::error(cur_pos, Error::WrongCharacter(c, '0'))
                }
                ('0', c) => if let Some(d) = c.to_digit(10) {
                    parse_digits(10, d)
                } else {
                    Result::error(cur_pos, Error::NotADigit)
                },
                (c1, c2) => if let Some(d1) = c1.to_digit(10) {
                    if let Some(d2) = c2.to_digit(10) {
                        parse_digits(10, d1 * 10 + d2)
                    } else {
                        Result::error(cur_pos, Error::NotADigit)
                    }
                } else {
                    Result::error(start, Error::NotADigit)
                },
            }
        } else {
            if let Some(start_digit) = start_character.to_digit(10) {
                Result::token(start, Token::IntLiteral(start_digit as i64))
            } else {
                Result::error(start, Error::NotADigit)
            }
        }
    }
}

impl<I: Iterator<Item = char>> Iterator for Lexer<I> {
    type Item = Result;
    fn next(&mut self) -> Option<Self::Item> {
        // Bootstrap first iteration
        if self.cur_char == '\0' {
            if let Some(first) = self.input.next() {
                self.cur_char = first;
            } else {
                self.eof_hit = true;
            }
        }

        // Main iteration driver
        self.skip_whitespace();
        if !self.eof_hit {
            let res = self.next_token();
            if let Result::Error(_) = res {
                self.skip_line();
            }
            Some(res)
        } else {
            None
        }
    }
}
