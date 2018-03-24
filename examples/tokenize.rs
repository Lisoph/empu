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
    "#;

    for t in empu::assembler::parse(code.chars()) {
        println!("{:?}", t);
    }
}
