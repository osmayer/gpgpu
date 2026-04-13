
447inputs2/lw.elf:     file format elf32-littleriscv


Disassembly of section .text:

00400000 <main>:
  400000:	10000137          	lui	x2,0x10000
  400004:	100005b7          	lui	x11,0x10000
  400008:	00158593          	addi	x11,x11,1 # 10000001 <data+0x1>
  40000c:	123451b7          	lui	x3,0x12345
  400010:	68718193          	addi	x3,x3,1671 # 12345687 <data_end+0x2345673>
  400014:	78654237          	lui	x4,0x78654
  400018:	32120213          	addi	x4,x4,801 # 78654321 <data_end+0x6865430d>
  40001c:	00312023          	sw	x3,0(x2) # 10000000 <data>
  400020:	0045a1a3          	sw	x4,3(x11)
  400024:	00012683          	lw	x13,0(x2)
  400028:	00412703          	lw	x14,4(x2)
  40002c:	0035a783          	lw	x15,3(x11)
  400030:	0035a003          	lw	x0,3(x11)
  400034:	00a00513          	addi	x10,x0,10
  400038:	00000073          	ecall

Disassembly of section .data:

10000000 <data>:
	...
