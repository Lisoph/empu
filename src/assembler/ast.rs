use super::super::Unit;

pub enum AstNode {
    Instruction(Instruction),
    LabelDeclaration(Label),
    Directive(Directive),
}

pub enum Instruction {
    Mov(Usd),
    Add(Usd),
    Sub(Usd),
    Mul(Usd),
    Div(Usd),
    Cmp(Usd),
    Jg(Address),
    Je(Address),
    Jl(Address),
    Jmp(Address),
    Int(IntegerExpr),
    Iret,
    And(Usd),
    Or(Usd),
    Xor(Usd),
    Not(Usd),
    Shl(Usd),
    Shr(Usd),
}

pub struct Usd {
    pub unit: Unit,
    pub source: Source,
    pub destination: Address,
}

pub enum Source {
    Value(IntegerExpr),
    Pointer(Address),
}

pub enum IntegerExpr {
    Literal(i64),
    LineOffset(i64),
}

pub struct Address {
    pub location: IntegerExpr,
    pub depth: u8,
}

pub enum Label {
    Absolute(String),
    Relative(String),
}

pub enum Directive {
    DeclareBytes(usize, Option<u8>),
    DeclareString(String),
}
