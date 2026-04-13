
447inputs/syscalltest.elf:     file format elf32-littleriscv


Disassembly of section .text:

00400000 <main>:
  400000:	00a00293          	addi	x5,x0,10
  400004:	00000513          	addi	x10,x0,0
  400008:	00000073          	ecall
  40000c:	00a00513          	addi	x10,x0,10
  400010:	00000073          	ecall
