use std::io::Result as IoResult;
use std::collections::HashMap;

pub mod assembler;
pub mod format_asm;

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
    And(Usd),
    Or(Usd),
    Xor(Usd),
    Not(Usd),
    Shl(Usd),
    Shr(Usd),
}

impl Instruction {
    pub fn instr_str(&self) -> &'static str {
        use Instruction::*;
        match self {
            &Mov(..) => "mov",
            &Add(..) => "add",
            &Sub(..) => "sub",
            &Mul(..) => "mul",
            &Div(..) => "div",
            &Cmp(..) => "cmp",
            &Jg(..) => "jg",
            &Je(..) => "je",
            &Jl(..) => "jl",
            &Jmp(..) => "jmp",
            &And(..) => "and",
            &Or(..) => "or",
            &Xor(..) => "xor",
            &Not(..) => "not",
            &Shl(..) => "shl",
            &Shr(..) => "shr",
        }
    }

    pub fn usd(&self) -> Option<&Usd> {
        use Instruction::*;
        match self {
            &Mov(ref usd) | &Add(ref usd) | &Sub(ref usd) | &Mul(ref usd) | &Div(ref usd)
            | &Cmp(ref usd) | &And(ref usd) | &Or(ref usd) | &Xor(ref usd) | &Not(ref usd)
            | &Shl(ref usd) | &Shr(ref usd) => Some(usd),
            _ => None,
        }
    }

    pub fn address(&self) -> Option<&Address> {
        use Instruction::*;
        match self {
            &Jg(ref adr) |
            &Je(ref adr) |
            &Jl(ref adr) |
            &Jmp(ref adr) => Some(adr),
            _ => None,
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.format_asm(f)
    }
}

pub struct Usd {
    pub unit: Unit,
    pub source: Source,
    pub destination: Address,
}

pub enum Unit {
    Byte,
    Word,
    Dword,
}

impl Unit {
    pub fn id(&self) -> u8 {
        match *self {
            Unit::Byte => 0,
            Unit::Word => 1,
            Unit::Dword => 2,
        }
    }

    pub fn num_bytes(&self) -> u8 {
        match *self {
            Unit::Byte => 1,
            Unit::Word => 2,
            Unit::Dword => 4,
        }
    }
}

pub struct Address {
    pub location: u16,
    pub depth: u8,
}

pub enum Source {
    Value(u32),
    Pointer(Address),
}

impl Source {
    pub fn id(&self) -> u8 {
        match *self {
            Source::Value(_) => 0,
            Source::Pointer(_) => 1,
        }
    }

    pub fn depth(&self) -> Option<u8> {
        if let &Source::Pointer(ref adr) = self {
            Some(adr.depth)
        } else {
            None
        }
    }
}

pub fn assemble(
    instructions: &[Instruction],
) -> IoResult<(Vec<u8>, HashMap<usize, (usize, usize)>)> {
    let mut binary = Vec::new();
    let mut map = HashMap::new();

    for (i, ins) in instructions.iter().enumerate() {
        let start = binary.len();
        ins.assemble(&mut binary)?;
        let end = binary.len();
        map.insert(i, (start, end));
    }

    Ok((binary, map))
}
