
447inputs/shifttest.elf:     file format elf32-littleriscv


Disassembly of section .text:

00400000 <main>:
  400000:	bed06193          	ori	x3,x0,-1043
  400004:	01519213          	slli	x4,x3,0x15
  400008:	00619293          	slli	x5,x3,0x6
  40000c:	40a25313          	srai	x6,x4,0xa
  400010:	4102d393          	srai	x7,x5,0x10
  400014:	00a25413          	srli	x8,x4,0xa
  400018:	0102d493          	srli	x9,x5,0x10
  40001c:	00a00513          	addi	x10,x0,10
  400020:	00000073          	ecall
