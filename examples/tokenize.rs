extern crate empu;

fn main() {
    let code = r#"
    mov dword 100, -200
    cmp byte @105, 33
    ds "Hello world"
    asdf foo
    iret
    abs_label:
    int 0xFF
        .sub_label:
            mov @1000, @400
            jmp abs_label

    0x01
    0x02
    0xFF
    0xff
    0x0A
    -0x0A
    iret
    0b0
    0b1
    0b11111111
    0b11111110
    -0b11111101
    iret
    0o377
    +0o001

    mov dword 100, $+1
    mov dword 100, $-1
    mov dword 100, $1
    "#;

    for t in empu::assembler::parse(code.chars()) {
        println!("{:?}", t);
    }
}
