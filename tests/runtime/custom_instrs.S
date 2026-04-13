.macro tid dest
    .insn r 0x0F, 0, 0, \dest, x0, x0
.endm


.macro bid dest
    .insn r 0x0F, 1, 0, \dest, x0, x0
.endm

.macro bdim dest
    .insn r 0x0F, 2, 0, \dest, x0, x0
.endm

.macro gdim dest
    .insn r 0x0F, 3, 0, \dest, x0, x0
.endm

.macro lw.S dest, rs1, imm
    .insn i 0x0F, 4, \dest, \rs1, \imm
.endm