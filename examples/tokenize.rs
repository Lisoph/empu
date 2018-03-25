extern crate empu;

fn main() {
    let code = r#"mov dword 100, -0x200
    cmp byte @105, 0b00101011
    ds "Hello world"
    asdf foo
    iret
    abs_label:
    int 0xFF
        .sub_label:
            mov @0o333, @400
            jmp abs_label
    ; Comment 1
    iret ; Comment 2
    ; Comment 3 int 255
    "#;

    for t in empu::assembler::parse(code.chars()) {
        println!("{:?}", t);
    }
}
