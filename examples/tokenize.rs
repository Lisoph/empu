extern crate empu;

fn main() {
    let code = "mov dword 100, -200\ncmp byte @105, 33\nds \"Hello world\"\nasdf foo\niret";
    for t in empu::assembler::parse(code.chars()) {
        println!("{:?}", t);
    }
}
