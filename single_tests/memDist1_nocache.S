    .data
data:
    # some very very random numbers
    .word 100
    .word 213
    .word 240
    .word 330
    .word 344
    .word 349
    .word 447
data_end:
    .text
    .global main

main:
    addi x9, x0, 110
    sw   x9, (gp)

    # place all of the numbers in registers and then immediately increment them
    lw x11, 0(gp)
    addi x11, x11, 1
    
    beq x0, x0, label_1

    lw x12, 4(gp)
    # addi x12, x12, 1

    lw x13, 8(gp)
    # addi x13, x13, 1

label_1:
    lw x14, 12(gp)
    addi x14, x14, 1
   
    lw x15, 16(gp)
    addi x15, x15, 1
    
    bne x0, x0, label_2
    
    lw x16, (gp)
    addi x16, x16, 1
    

label_2: 
    # sum up all of the results for convenience
    add x9, x11, x12
    add x9, x13, x9
    add x9, x14, x9
    add x9, x15, x9
    add x9, x16, x9
    add x9, x17, x9

    addi x10, x0, 0xa
    ecall
