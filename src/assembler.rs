use super::*;

use std::io::{Result as IoResult, Write};

/**
    And(Usd),
    Or(Usd),
    Xor(Usd),
    Not(Usd),
    Shl(Usd),
    Shr(Usd),
 */
impl Instruction {
    pub fn assemble(&self, mut out: &mut Write) -> IoResult<()> {
        use Instruction::*;
        match self {
            &Mov(ref usd) => mov_add_sub_mul_div(out, 0, &usd),
            &Add(ref usd) => mov_add_sub_mul_div(out, 1, &usd),
            &Sub(ref usd) => mov_add_sub_mul_div(out, 2, &usd),
            &Mul(ref usd) => mov_add_sub_mul_div(out, 3, &usd),
            &Div(ref usd) => mov_add_sub_mul_div(out, 4, &usd),
            &Cmp(ref usd) => {
                out.write_byte(usd.unit.id() << 2 | usd.destination.depth)?;
                out.write_byte(usd.source.id() << 7 | usd.source.depth().unwrap_or(0) << 5)?;
                out.write_short(usd.destination.location)?;
                match usd.source {
                    Source::Pointer(ref adr) => out.write_short(adr.location)?,
                    Source::Value(ref val) => for i in (0..usd.unit.num_bytes()).rev() {
                        out.write_byte((val >> i * 8) as u8)?;
                    },
                }
                Ok(())
            }
            &Jg(ref adr) => jump(out, 0, adr),
            &Je(ref adr) => jump(out, 1, adr),
            &Jl(ref adr) => jump(out, 2, adr),
            &Jmp(ref adr) => jump(out, 3, adr),
            &And(ref usd) => and_or_xor_not_shl_shr(out, 0, usd),
            &Or(ref usd) => and_or_xor_not_shl_shr(out, 1, usd),
            &Xor(ref usd) => and_or_xor_not_shl_shr(out, 2, usd),
            &Not(ref usd) => and_or_xor_not_shl_shr(out, 3, usd),
            &Shl(ref usd) => and_or_xor_not_shl_shr(out, 4, usd),
            &Shr(ref usd) => and_or_xor_not_shl_shr(out, 5, usd),
        }
    }
}

fn mov_add_sub_mul_div(mut out: &mut Write, id: u8, usd: &Usd) -> IoResult<()> {
    out.write_byte(usd.unit.id() << 2 | usd.source.id())?;
    out.write_byte(id << 5 | usd.destination.depth << 2 | usd.source.depth().unwrap_or(0))?;
    out.write_short(usd.destination.location)?;
    match usd.source {
        Source::Pointer(ref adr) => out.write_short(adr.location)?,
        Source::Value(ref val) => for i in (0..usd.unit.num_bytes()).rev() {
            out.write_byte((val >> i * 8) as u8)?;
        },
    }
    Ok(())
}

fn jump(mut out: &mut Write, id: u8, adr: &Address) -> IoResult<()> {
    out.write_byte(id << 2 | adr.depth)?;
    out.write_short(adr.location)
}

fn and_or_xor_not_shl_shr(mut out: &mut Write, id: u8, usd: &Usd) -> IoResult<()> {
    out.write_byte(id)?;
    out.write_byte(
        usd.unit.id() << 6 | usd.destination.depth << 4 | usd.source.depth().unwrap_or(0) << 2
            | usd.source.id(),
    )?;
    out.write_short(usd.destination.location)?;
    match usd.source {
        Source::Pointer(ref adr) => out.write_short(adr.location)?,
        Source::Value(ref val) => for i in (0..usd.unit.num_bytes()).rev() {
            out.write_byte((val >> i * 8) as u8)?;
        },
    }
    Ok(())
}

trait WriteExt {
    fn write_byte(&mut self, byte: u8) -> IoResult<()>;
    fn write_short(&mut self, short: u16) -> IoResult<()>;
}

impl<T: Write> WriteExt for T {
    fn write_byte(&mut self, byte: u8) -> IoResult<()> {
        self.write_all(&[byte])
    }

    fn write_short(&mut self, short: u16) -> IoResult<()> {
        self.write_all(&[(short >> 8) as u8, (short & 0xF) as u8])
    }
}
