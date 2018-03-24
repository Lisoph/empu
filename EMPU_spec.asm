; EMPU (Eazy memory processing unit) V1.0 Spec

; No registers - all operations operate on memory directly.
; EMPU has one dynamic addressing mode instead of a direct, register indirect and memory indirect.
; The dynamic addressing mode can achieve indirect addressing up to 4 levels. Level 1 is "direct".
; Examples:
    mov word @0x100, 0x400 ; Move the value 0x123 into mem[0x100]
    add word @0x100, 5 ; Add the value 5 to mem[0x100]
    mov word @0x1400, @@0x100 ; Move the value at mem[mem[0x100]] into mem[0x1400]
    mov word @0x1400, @0x405 ; Same as the 3 lines above

    mov word 0x100, 0x200 ; syntax error - left hand side must be an address '@'


; unit = (byte | word | dword)
; Instructions and their encodings
; The first 4 highest bits of an instructions it always the opcode ID.

; U = Unit, I = Ins2 (0 - 4 for mov, add, ...),
; L = lhs addressing level, R = rhs addressing level, D/F = destination address, S = source address,
; A = 8bit value, AB = 16bit value, ABCD = 32bit value
(mov | add | sub | mul | div) unit destination, source ; 0000UU00 III0LLRR DDDDDDDD DDDDDDDD SSSSSSSS SSSSSSSS
(mov | add | sub | mul | div) unit destination, value  ; 0000UU01 III0LL00 FFFFFFFF FFFFFFFF AAAAAAAA (BBBBBBBB (CCCCCCCC DDDDDDDD))

; U = unit, L = lhs addressing level, R = rhs addressing level, A = lhs address, B = rhs address,
; V = 8bit value, VX = 16bit value, VXYZ = 32bit value
cmp unit address1, address2 ; 0001UULL 0RR00000 AAAAAAAA AAAAAAAA BBBBBBBB BBBBBBBB
cmp unit address, value     ; 0001UULL 10000000 AAAAAAAA AAAAAAAA VVVVVVVV(XXXXXXXX(YYYYYYYY ZZZZZZZZ))

; M = Ins2 (0 - 3 for jg, je, ...), L = addressing level, A = address
(jg | je | jl | jmp) address ; 0010MMLL AAAAAAAA AAAAAAAA

; I = interrupt id, A = address

int id ; 00110000 IIIIIIII
iret   ; 00110001

; M = Ins2 (0 - 5 for and, or, ...), U = unit, L = lhs addressing level, D = address,
; A = 8bit value, AB = 16bit value, ABCD = 32bit value
; R = source addressing mode, S = source address
(and | or | xor | not | shl | shr) address value           ; 01000MMM UULL0001 DDDDDDDD DDDDDDDD AAAAAAAA (BBBBBBBB (CCCCCCCC DDDDDDDD))
(and | or | xor | not | shl | shr) unit destination source ; 01000MMM UULLRR00 DDDDDDDD DDDDDDDD SSSSSSSS SSSSSSSS

; Example program:

add:
    add @.a, @.b ; add whatever is in .b to .a
    jmp @.ret
    .a: db 2 ; reserve 2 bytes of memory at this location
    .b: db 2 ; ditto
    .ret: db 2 ; ditto

main:
    ; register interrupt 0xEE handler
    mov 0xEE, handle_ee

    ; calculate 4 + 7 with a function
    mov add.a, 4
    mov add.b, 7
    mov add.ret, $+2 ; set add.ret to the next instruction after this
    jmp add
    mov 0x100, add.a ; store the result of the addition in mem[0x100]
    int 0x12

handle_ee:
    ; this gets called from the cpu in an interrupt
    mov print_str.str, .hello
    mov print_str.ret, $+2
    jmp print_str
    iret ; return from interrupt, continue execution where it left of.
    .hello: ds "Hello world!\0"

print_str:
    ; 0x101: Where the hardware expects the string address to be stored.
    mov 0x101, @.str
    int 0x10 ; 0x10 = print string interrupt
    jmp @.ret
    .str: db 2
    .ret: db 2
