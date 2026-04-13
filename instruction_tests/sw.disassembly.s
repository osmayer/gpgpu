
447inputs2/sw.elf:     file format elf32-littleriscv


Disassembly of section .text:

00400000 <main>:
  400000:	10000137          	lui	x2,0x10000
  400004:	100005b7          	lui	x11,0x10000
  400008:	00158593          	addi	x11,x11,1 # 10000001 <data+0x1>
  40000c:	78654637          	lui	x12,0x78654
  400010:	31260613          	addi	x12,x12,786 # 78654312 <data_end+0x686542fe>
  400014:	123456b7          	lui	x13,0x12345
  400018:	68768693          	addi	x13,x13,1671 # 12345687 <data_end+0x2345673>
  40001c:	00c12023          	sw	x12,0(x2) # 10000000 <data>
  400020:	00d5a1a3          	sw	x13,3(x11)
  400024:	00012703          	lw	x14,0(x2)
  400028:	0035a783          	lw	x15,3(x11)
  40002c:	00358803          	lb	x16,3(x11)
  400030:	00a00513          	addi	x10,x0,10
  400034:	00000073          	ecall

Disassembly of section .data:

10000000 <data>:
	...
