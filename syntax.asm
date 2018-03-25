; This is a single-line comment

this_is_an_absolute_label:
    iret ; A simple instruction without arguments
    int 255 ; An instruction with an argument written in decimal
    int 0xFF ; Same as above, but written in hexadecimal
    int 0o377 ; Ditto, octal
    int 0b11111111 ; Ditto, binary

    ; An instruction with a unit / size and 2 arguments specified.
    cmp dword 100, 22
    cmp byte 100, @22
    cmp word 100, @@22
    ; etc.

    ;; The unit (byte / word / dword):
    ; A lot of instructions can work with multiple payload sizes.
    ; The unit is used to specify the payload size.
    ; A `cmp byte` means: "compare a byte".
    ; A `mov dword` means: "move a double word (4 bytes)".

    ;; The @ symbol:
    ; Left and right hand side arguments can be preceded by 1 to (3 or 4) @ symbols.
    ; Every single @ symbol adds another level of indirection.
    ; 22 means => 22                 -> The literal value 22, no memory lookup
    ; @22      => memory[22]         -> The value in memory at address 22
    ; @@22     => memory[memory[22]] -> The value in memory at the address specified by the value in memory at address 22
    ; Left hand side arguments of instructions are **always** memory addresses, they can never be literal values.
    ; This means that always one extra @ is implied on the left hand side.
    ; This allows for up to 4 indirections at most, while right hand side arguments can only take 3.


    .this_is_a_sub_label:
    ; Sub-labels are always relative to their parent's absolute label.
    ; Their names are not shared globally throughout a program so there are no namespace collisions.
    ; Refering to them in code works in 2 ways:
    ; 1) From anywhere in the code: Precede them by their parent and a dot => this_is_an_absolute_label.this_is_a_sub_label
    ; 2) From within the same parent label: Just precede them by a dot => .this_is_a_sub_label
    ; Nested sub-labels are not possible.


    db 4
    ; Declare 4 bytes. The produced binary will be padded with 4 bytes at this location, initialized to 0.
    ; Every byte in memory (includes the loaded binary) can be modified at runtime, which allows for 'dynamic' code.
    ; In fact, rewriting data inside functions is necessary for passing arguments and setting the return address.
    ; Rewriting instructions is very dangerous and not recommended, but it is possible.

    db 4 0xFF ; Same as above, but every byte is initialized to 0xFF instead of 0.

    ds "Hello, world!\0"
    ; Declare string (of bytes).
    ; Declares 14 bytes. 13 for "Hello, world!" and 1 for the \0 escape sequence.
    ; Each byte is initialized to each string byte (ASCII code or escape sequence) individually.
    ; Basically just a shorthand for a bunch of declare byte directives.
    ; String sequences support the following escape characters (identical to the C programming language):
    ;   \0 - Zero (0x00)
    ;   \a - Alert (0x07)
    ;   \b - Backspace (0x08)
    ;   \f - Formfeed (0x0C)
    ;   \n - Newline (0x0A)
    ;   \r - Carriage return (0x0D)
    ;   \t - Horizontal tab (0x09)
    ;   \v - Vertical tab (0x0B)
    ;   \\ - Backslash (0x5C)
    ;   \" - Double quotation mark (0x22)
    ; \xHH - Raw bytes whose value is given by HH interpreted as hexadecimal (H can repeat indefinitely; one H is a nibble)
    ;        Example: "Test 123 \xDEADBEEF"


    mov dword 100, $
    ; $ is a special symbol - it evaluates to the memory address of the current instruction (at compile/link-time).
    ; The instruction above moves the address of itself into memory location 100.
    ; This symbol can be offset by a constant like so:
    mov dword 100, $+1
    ; $+1 means the *next* instruction / the one following the current one.
    ; It does **not** mean plus 1 byte!
    ; Positive (+) and negative (-) offsets are allowed.
    ; Positive is implied by default so the + symbol can be omitted.
