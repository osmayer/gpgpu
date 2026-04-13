
447inputs/memtest1.elf:     file format elf32-littleriscv


Disassembly of section .text:

00400000 <main>:
  400000:	7de00293          	addi	x5,x0,2014
  400004:	5ca00313          	addi	x6,x0,1482
  400008:	33f00393          	addi	x7,x0,831
  40000c:	0000d437          	lui	x8,0xd
  400010:	afe40413          	addi	x8,x8,-1282 # cafe <main-0x3f3502>
  400014:	00518023          	sb	x5,0(x3)
  400018:	006180a3          	sb	x6,1(x3)
  40001c:	00718323          	sb	x7,6(x3)
  400020:	008183a3          	sb	x8,7(x3)
  400024:	0001c483          	lbu	x9,0(x3)
  400028:	0011c503          	lbu	x10,1(x3)
  40002c:	00618583          	lb	x11,6(x3)
  400030:	00718603          	lb	x12,7(x3)
  400034:	00418193          	addi	x3,x3,4
  400038:	00519023          	sh	x5,0(x3)
  40003c:	00819123          	sh	x8,2(x3)
  400040:	00719223          	sh	x7,4(x3)
  400044:	00819323          	sh	x8,6(x3)
  400048:	0001d683          	lhu	x13,0(x3)
  40004c:	0021d703          	lhu	x14,2(x3)
  400050:	00419783          	lh	x15,4(x3)
  400054:	00619803          	lh	x16,6(x3)
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
