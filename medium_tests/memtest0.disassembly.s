
447inputs/memtest0.elf:     file format elf32-littleriscv


Disassembly of section .text:

00400000 <main>:
  400000:	003000b3          	add	x1,x0,x3
  400004:	0ff00293          	addi	x5,x0,255
  400008:	00528333          	add	x6,x5,x5
  40000c:	006303b3          	add	x7,x6,x6
  400010:	7d038413          	addi	x8,x7,2000
  400014:	0050a023          	sw	x5,0(x1)
  400018:	0060a223          	sw	x6,4(x1)
  40001c:	0070a423          	sw	x7,8(x1)
  400020:	0080a623          	sw	x8,12(x1)
  400024:	0000a483          	lw	x9,0(x1)
  400028:	0040a503          	lw	x10,4(x1)
  40002c:	0080a583          	lw	x11,8(x1)
  400030:	00c0a603          	lw	x12,12(x1)
  400034:	00408093          	addi	x1,x1,4
  400038:	0050a023          	sw	x5,0(x1)
  40003c:	0060a223          	sw	x6,4(x1)
  400040:	0070a423          	sw	x7,8(x1)
  400044:	0080a623          	sw	x8,12(x1)
  400048:	ffc0a683          	lw	x13,-4(x1)
  40004c:	0000a703          	lw	x14,0(x1)
  400050:	0040a783          	lw	x15,4(x1)
  400054:	0080a803          	lw	x16,8(x1)
  400058:	009008b3          	add	x17,x0,x9
  40005c:	00a888b3          	add	x17,x17,x10
  400060:	00b888b3          	add	x17,x17,x11
  400064:	00c888b3          	add	x17,x17,x12
  400068:	00d888b3          	add	x17,x17,x13
  40006c:	00e888b3          	add	x17,x17,x14
  400070:	00f888b3          	add	x17,x17,x15
  400074:	010888b3          	add	x17,x17,x16
  400078:	00a00513          	addi	x10,x0,10
  40007c:	00000073          	ecall

Disassembly of section .data:

10000000 <data>:
	...
