  
    .text                       # Declare the code to be in the .text segment
    .global main                # Make main visible to the linker
main:
    addi    t0, zero, 447        # t0 (x5) = 447
    # Custom instruction with opcode 0x7b, func3 0x0, and func7 0x01
    .insn r 0x0F, 0, 0, a0, a1, a2

    nop                             
    nop
    nop
    nop
    nop
    nop
    nop
l
    addi    a0, zero, 0xa       # a0 (x10) = 0xa
    ecall                       # Terminate the simulation by passing 0xa to
                                # ecall in register a0 (x10).

