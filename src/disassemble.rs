use super::*;

pub enum DisassembleError {
    NotEnoughData,
    InvalidInstruction,
}

impl std::cmp::PartialEq for DisassembleError {
    fn eq(&self, rhs: &Self) -> bool {
        use self::DisassembleError::*;
        match (self, rhs) {
            (&NotEnoughData, &NotEnoughData) => true,
            (&InvalidInstruction, &InvalidInstruction) => true,
            _ => false,
        }
    }
}

impl From<()> for DisassembleError {
    fn from(_: ()) -> Self {
        DisassembleError::NotEnoughData
    }
}

pub type DisassembleResult = Result<Instruction, DisassembleError>;

pub struct DisassembleIter<I> {
    iter: I,
}

impl<I: Iterator<Item = u8>> Iterator for DisassembleIter<I> {
    type Item = Option<Instruction>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(byte) = self.iter.next() {
            match Instruction::disassemble(byte, &mut self.iter) {
                Ok(ins) => Some(Some(ins)),
                Err(ref e) if *e == DisassembleError::InvalidInstruction => Some(None),
                Err(_) => None,
            }
        } else {
            None
        }
    }
}

pub fn disassemble<I: Iterator<Item = u8>>(input: I) -> DisassembleIter<I> {
    DisassembleIter { iter: input }
}

impl Instruction {
    pub fn disassemble<I: Iterator<Item = u8>>(b1: u8, rest: &mut I) -> DisassembleResult {
        let id = b1 >> 4;
        match id {
            0b0000 => mov_add_sub_mul_div(b1, rest),
            0b0001 => cmp(b1, rest),
            0b0010 => jg_je_jl_jmp(b1, rest),
            0b0011 => Err(invins()), // TODO: int
            0b0100 => and_or_xor_not_shl_shr(b1, rest),
            _ => Err(invins()),
        }
    }
}

pub fn mov_add_sub_mul_div<I: Iterator<Item = u8>>(b1: u8, rest: &mut I) -> DisassembleResult {
    let unit = Unit::from_id((b1 >> 2) & 0b11).ok_or(invins())?;
    let b2 = rest.read_byte()?;
    let (dest_depth, source_depth) = ((b2 >> 2) & 0b11, b2 & 0b11);

    let dest = rest.read_short()?;
    let (destination, source) = match b1 & 0b01 {
        0b0 => Ok((
            Address {
                location: dest,
                depth: dest_depth,
            },
            Source::Pointer(Address {
                location: rest.read_short()?,
                depth: source_depth,
            }),
        )),
        0b1 => {
            let mut buf = [0u8; 4];
            let _ = rest.read_slice(&mut buf[0..unit.num_bytes() as usize])?;
            let mut val = 0u32;
            for i in 0..unit.num_bytes() as usize {
                val = (val << 8) | buf[i] as u32;
            }
            Ok((
                Address {
                    location: dest,
                    depth: dest_depth,
                },
                Source::Value(val),
            ))
        }
        _ => Err(invins()),
    }?;

    let usd = Usd {
        unit,
        source,
        destination,
    };

    let ins2 = b2 >> 5;
    match ins2 {
        0 => Ok(Instruction::Mov(usd)),
        1 => Ok(Instruction::Add(usd)),
        2 => Ok(Instruction::Sub(usd)),
        3 => Ok(Instruction::Mul(usd)),
        4 => Ok(Instruction::Div(usd)),
        _ => Err(invins()),
    }
}

fn cmp<I: Iterator<Item = u8>>(b1: u8, rest: &mut I) -> DisassembleResult {
    let unit = Unit::from_id((b1 >> 2) & 0b11).ok_or(invins())?;
    let b2 = rest.read_byte()?;
    let (dest_depth, source_depth) = (b1 & 0b11, (b2 >> 5) & 0b11);

    let dest = rest.read_short()?;
    let (destination, source) = match b2 >> 7 {
        0b0 => Ok((
            Address {
                location: dest,
                depth: dest_depth,
            },
            Source::Pointer(Address {
                location: rest.read_short()?,
                depth: source_depth,
            }),
        )),
        0b1 => {
            let mut buf = [0u8; 4];
            let _ = rest.read_slice(&mut buf[0..unit.num_bytes() as usize])?;
            let mut val = 0u32;
            for i in 0..unit.num_bytes() as usize {
                val = (val << 8) | buf[i] as u32;
            }
            Ok((
                Address {
                    location: dest,
                    depth: dest_depth,
                },
                Source::Value(val),
            ))
        }
        _ => Err(invins()),
    }?;

    Ok(Instruction::Cmp(Usd {
        unit,
        source,
        destination,
    }))
}

fn jg_je_jl_jmp<I: Iterator<Item = u8>>(b1: u8, rest: &mut I) -> DisassembleResult {
    let address = Address {
        location: rest.read_short()?,
        depth: b1 & 0b11,
    };
    let ins2 = (b1 >> 2) & 0b11;
    match ins2 {
        0 => Ok(Instruction::Jg(address)),
        1 => Ok(Instruction::Je(address)),
        2 => Ok(Instruction::Jl(address)),
        3 => Ok(Instruction::Jmp(address)),
        _ => Err(invins()),
    }
}

fn and_or_xor_not_shl_shr<I: Iterator<Item = u8>>(b1: u8, rest: &mut I) -> DisassembleResult {
    let b2 = rest.read_byte()?;
    let unit = Unit::from_id(b2 >> 6).ok_or(invins())?;
    let (dest_depth, source_depth) = ((b2 >> 4) & 0b11, (b2 >> 2) & 0b11);

    let dest = rest.read_short()?;
    let (destination, source) = match b2 & 0b01 {
        0b0 => Ok((
            Address {
                location: dest,
                depth: dest_depth,
            },
            Source::Pointer(Address {
                location: rest.read_short()?,
                depth: source_depth,
            }),
        )),
        0b1 => {
            let mut buf = [0u8; 4];
            let _ = rest.read_slice(&mut buf[0..unit.num_bytes() as usize])?;
            let mut val = 0u32;
            for i in 0..unit.num_bytes() as usize {
                val = (val << 8) | buf[i] as u32;
            }
            Ok((
                Address {
                    location: dest,
                    depth: dest_depth,
                },
                Source::Value(val),
            ))
        }
        _ => Err(invins()),
    }?;

    let usd = Usd {
        unit,
        source,
        destination,
    };

    let ins2 = b1 & 0b111;
    match ins2 {
        0 => Ok(Instruction::And(usd)),
        1 => Ok(Instruction::Or(usd)),
        2 => Ok(Instruction::Xor(usd)),
        3 => Ok(Instruction::Not(usd)),
        4 => Ok(Instruction::Shl(usd)),
        5 => Ok(Instruction::Shr(usd)),
        _ => Err(invins()),
    }
}

fn invins() -> DisassembleError {
    DisassembleError::InvalidInstruction
}

trait IterExt {
    fn read_byte(&mut self) -> Result<u8, ()>;
    fn read_short(&mut self) -> Result<u16, ()>;
    fn read_slice(&mut self, buf: &mut [u8]) -> Result<(), ()>;
}

impl<T: Iterator<Item = u8>> IterExt for T {
    fn read_byte(&mut self) -> Result<u8, ()> {
        self.next().ok_or(())
    }

    fn read_short(&mut self) -> Result<u16, ()> {
        Ok((self.read_byte()? as u16) << 8 | self.read_byte()? as u16)
    }

    fn read_slice(&mut self, buf: &mut [u8]) -> Result<(), ()> {
        for i in 0..buf.len() {
            match self.next() {
                Some(byte) => buf[i] = byte,
                None => return Err(()),
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! eq_as_disas {
        ($($instr:expr),*) => {{
            let (binary, _) = assemble(&[
                $($instr),*
            ]).unwrap();
            let disas: Vec<_> = disassemble(binary.into_iter()).collect();
            let mut index = 0;
            $(
                assert_eq!($instr, *disas[index].as_ref().unwrap());
                index += 1;
            )*
        }}
    }

    #[test]
    fn test_mov_add_sub_mul_div() {
        eq_as_disas![
            Instruction::Mov(Usd {
                unit: Unit::Byte,
                source: Source::Value(123),
                destination: Address {
                    location: 0xBEEF,
                    depth: 1,
                }
            },),
            Instruction::Add(Usd {
                unit: Unit::Word,
                source: Source::Value(33),
                destination: Address {
                    location: 0xDEAD,
                    depth: 3,
                }
            }),
            Instruction::Sub(Usd {
                unit: Unit::Word,
                source: Source::Value(33),
                destination: Address {
                    location: 0xDEAD,
                    depth: 3,
                }
            }),
            Instruction::Mul(Usd {
                unit: Unit::Word,
                source: Source::Value(33),
                destination: Address {
                    location: 0xDEAD,
                    depth: 3,
                }
            }),
            Instruction::Div(Usd {
                unit: Unit::Dword,
                source: Source::Value(127),
                destination: Address {
                    location: 0xBABE,
                    depth: 2,
                }
            })
        ];
    }

    #[test]
    fn test_cmp() {
        eq_as_disas![
            Instruction::Cmp(Usd {
                unit: Unit::Byte,
                source: Source::Value(123),
                destination: Address {
                    location: 0xBEEF,
                    depth: 3,
                }
            },),
            Instruction::Cmp(Usd {
                unit: Unit::Byte,
                source: Source::Pointer(Address {
                    location: 0xDEAD,
                    depth: 2,
                },),
                destination: Address {
                    location: 0xBEEF,
                    depth: 1,
                }
            },)
        ]
    }

    #[test]
    fn test_jg_je_jl_jmp() {
        eq_as_disas![
            Instruction::Jg(Address {
                location: 0x100,
                depth: 0,
            },),
            Instruction::Je(Address {
                location: 0x100,
                depth: 1,
            },),
            Instruction::Jl(Address {
                location: 0x100,
                depth: 2,
            },),
            Instruction::Jmp(Address {
                location: 0x100,
                depth: 3,
            },)
        ];
    }

    #[test]
    fn test_and_or_xor_not_shl_shr() {
        eq_as_disas![
            Instruction::And(Usd {
                unit: Unit::Dword,
                source: Source::Value(0xBEEF),
                destination: Address {
                    location: 0x100,
                    depth: 0,
                }
            }),
            Instruction::Or(Usd {
                unit: Unit::Dword,
                source: Source::Value(0xBEEF),
                destination: Address {
                    location: 0x100,
                    depth: 1,
                }
            }),
            Instruction::Xor(Usd {
                unit: Unit::Dword,
                source: Source::Value(0xBEEF),
                destination: Address {
                    location: 0x100,
                    depth: 2,
                }
            }),
            Instruction::Not(Usd {
                unit: Unit::Dword,
                source: Source::Value(0xBEEF),
                destination: Address {
                    location: 0x100,
                    depth: 3,
                }
            }),
            Instruction::Shl(Usd {
                unit: Unit::Dword,
                source: Source::Pointer(Address {
                    location: 0x102,
                    depth: 0,
                },),
                destination: Address {
                    location: 0x100,
                    depth: 3,
                }
            }),
            Instruction::Shr(Usd {
                unit: Unit::Dword,
                source: Source::Pointer(Address {
                    location: 0x102,
                    depth: 1,
                },),
                destination: Address {
                    location: 0x100,
                    depth: 2,
                }
            })
        ]
    }
}
