use super::*;

use std::fmt::{Error as FmtError, Formatter};
use std::iter;

impl Instruction {
    pub fn format_asm(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        write!(fmt, "{}", self.instr_str().to_uppercase())?;

        if let Some(ref usd) = self.usd() {
            write!(
                fmt,
                " {} {}0x{:X}, {}0x{:X}",
                match usd.unit {
                    Unit::Byte => "byte",
                    Unit::Word => "word",
                    Unit::Dword => "dword",
                },
                indirection(usd.destination.depth as usize),
                usd.destination.location,
                match usd.source {
                    Source::Value(..) => "".to_owned(),
                    Source::Pointer(ref adr) => indirection(adr.depth as usize),
                },
                match usd.source {
                    Source::Value(ref v) => *v,
                    Source::Pointer(ref p) => p.location as u32,
                }
            )?;
        } else if let Some(ref adr) = self.address() {
            write!(
                fmt,
                " {}0x{:X}",
                indirection(adr.depth as usize),
                adr.location
            )?;
        }

        Ok(())
    }
}

fn indirection(depth: usize) -> String {
    iter::repeat("@").take(depth).collect()
}
