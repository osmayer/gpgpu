
simple/bid.elf:     file format elf32-littleriscv


Disassembly of section .text:

00400000 <main>:
  400000:	1f400413          	addi	x8,x0,500
  400004:	f3800493          	addi	x9,x0,-200
  400008:	00840133          	add	x2,x8,x8
  40000c:	009405b3          	add	x11,x8,x9
  400010:	00848633          	add	x12,x9,x8
  400014:	009486b3          	add	x13,x9,x9
  400018:	00a00513          	addi	x10,x0,10
  40001c:	0000168f          	.insn	4, 0x168f
  400020:	00a50033          	add	x0,x10,x10
  400024:	00000073          	ecall
