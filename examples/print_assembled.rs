extern crate empu;
extern crate term_painter;

use std::collections::HashMap;

use empu::{Address, Instruction, Source, Unit, Usd};
use empu::Instruction::*;

use term_painter::ToStyle;
use term_painter::Color::*;

fn main() {
    let program = [
        Mov(Usd {
            unit: Unit::Byte,
            source: Source::Value(0xDEAD),
            destination: Address {
                location: 0xBEFF,
                depth: 0,
            },
        }),
        Mov(Usd {
            unit: Unit::Dword,
            source: Source::Value(0xBEEFBEEF),
            destination: Address {
                location: 0xBEEF,
                depth: 1,
            },
        }),
        Add(Usd {
            unit: Unit::Word,
            source: Source::Pointer(Address {
                location: 0x1200,
                depth: 2,
            }),
            destination: Address {
                location: 0xBEEF,
                depth: 1,
            },
        }),
        Not(Usd {
            unit: Unit::Word,
            destination: Address {
                location: 0x1000,
                depth: 0,
            },
            source: Source::Value(0xFFFF),
        }),
        Jmp(Address {
            location: 0x1000,
            depth: 1,
        }),
    ];

    let (binary, map) = empu::assemble(&program).unwrap();
    print_binary(&binary, &map, &program);
}

fn print_binary(binary: &[u8], map: &HashMap<usize, (usize, usize)>, program: &[Instruction]) {
    for (i, &(start, end)) in map.iter() {
        print!(
            "{} - {} bytes.\n  ",
            Yellow.paint(&program[*i]),
            Red.paint(end - start),
        );
        let chunk = &binary[start..end];
        BrightWhite.with(|| {
            chunk.iter().for_each(|b| print!("{:08b} ", b));
            print!("\n  ");
            chunk.iter().for_each(|b| print!("0x{:02X}     ", b));
            println!();
        });
    }
}
