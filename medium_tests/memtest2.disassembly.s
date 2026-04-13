
447inputs/memtest2.elf:     file format elf32-littleriscv


Disassembly of section .text:

00400000 <main>:
  400000:	003000b3          	add	x1,x0,x3
  400004:	00008f13          	addi	x30,x1,0
  400008:	04008093          	addi	x1,x1,64
  40000c:	0ff00293          	addi	x5,x0,255
  400010:	20100313          	addi	x6,x0,513
  400014:	40300393          	addi	x7,x0,1027
  400018:	00001437          	lui	x8,0x1
  40001c:	80140413          	addi	x8,x8,-2047 # 801 <main-0x3ff7ff>
  400020:	00001937          	lui	x18,0x1
  400024:	000029b7          	lui	x19,0x2
  400028:	00004a37          	lui	x20,0x4
  40002c:	00008ab7          	lui	x21,0x8
  400030:	0050a023          	sw	x5,0(x1)
  400034:	0060a023          	sw	x6,0(x1)
  400038:	0070a023          	sw	x7,0(x1)
  40003c:	0080a023          	sw	x8,0(x1)
  400040:	0000a483          	lw	x9,0(x1)
  400044:	0060a223          	sw	x6,4(x1)
  400048:	0040a503          	lw	x10,4(x1)
  40004c:	0060a423          	sw	x6,8(x1)
  400050:	0080a583          	lw	x11,8(x1)
  400054:	0070a423          	sw	x7,8(x1)
  400058:	0080a603          	lw	x12,8(x1)
  40005c:	0080a623          	sw	x8,12(x1)
  400060:	0040a683          	lw	x13,4(x1)
  400064:	0050a823          	sw	x5,16(x1)
  400068:	00c0a703          	lw	x14,12(x1)
  40006c:	0120aa23          	sw	x18,20(x1)
  400070:	0130ac23          	sw	x19,24(x1)
  400074:	0140ae23          	sw	x20,28(x1)
  400078:	0350a023          	sw	x21,32(x1)
  40007c:	0200ab03          	lw	x22,32(x1)
  400080:	0140ab83          	lw	x23,20(x1)
  400084:	01c0ac03          	lw	x24,28(x1)
  400088:	0180ac83          	lw	x25,24(x1)
  40008c:	02408213          	addi	x4,x1,36
  400090:	00522023          	sw	x5,0(x4) # 0 <main-0x400000>
  400094:	00622223          	sw	x6,4(x4) # 4 <main-0x3ffffc>
  400098:	00722423          	sw	x7,8(x4) # 8 <main-0x3ffff8>
  40009c:	00822623          	sw	x8,12(x4) # c <main-0x3ffff4>
  4000a0:	00022d03          	lw	x26,0(x4) # 0 <main-0x400000>
  4000a4:	00c22d83          	lw	x27,12(x4) # c <main-0x3ffff4>
  4000a8:	00422e03          	lw	x28,4(x4) # 4 <main-0x3ffffc>
  4000ac:	00822e83          	lw	x29,8(x4) # 8 <main-0x3ffff8>
  4000b0:	02508a23          	sb	x5,52(x1)
  4000b4:	02608aa3          	sb	x6,53(x1)
  4000b8:	02709b23          	sh	x7,54(x1)
  4000bc:	0340af03          	lw	x30,52(x1)
  4000c0:	0320ac23          	sw	x18,56(x1)
  4000c4:	0330ae23          	sw	x19,60(x1)
  4000c8:	0380af83          	lw	x31,56(x1)
  4000cc:	0340ac23          	sw	x20,56(x1)
  4000d0:	0380af03          	lw	x30,56(x1)
  4000d4:	0320ac23          	sw	x18,56(x1)
  4000d8:	0330ac23          	sw	x19,56(x1)
  4000dc:	0340ac23          	sw	x20,56(x1)
  4000e0:	0380a203          	lw	x4,56(x1)
  4000e4:	00008f13          	addi	x30,x1,0
  4000e8:	000f2103          	lw	x2,0(x30)
  4000ec:	010f2183          	lw	x3,16(x30)
  4000f0:	020f2203          	lw	x4,32(x30)
  4000f4:	030f2283          	lw	x5,48(x30)
  4000f8:	030f0313          	addi	x6,x30,48
  4000fc:	00032383          	lw	x7,0(x6)
  400100:	ff032403          	lw	x8,-16(x6)
  400104:	fe032483          	lw	x9,-32(x6)
  400108:	fd032503          	lw	x10,-48(x6)
  40010c:	000f2583          	lw	x11,0(x30)
  400110:	004f2603          	lw	x12,4(x30)
  400114:	008f2683          	lw	x13,8(x30)
  400118:	00cf2703          	lw	x14,12(x30)
  40011c:	000f2783          	lw	x15,0(x30)
  400120:	008f2803          	lw	x16,8(x30)
  400124:	010f2883          	lw	x17,16(x30)
  400128:	018f2903          	lw	x18,24(x30)
  40012c:	00900133          	add	x2,x0,x9
  400130:	00a10133          	add	x2,x2,x10
  400134:	00c10133          	add	x2,x2,x12
  400138:	00d10133          	add	x2,x2,x13
  40013c:	00e10133          	add	x2,x2,x14
  400140:	01610133          	add	x2,x2,x22
  400144:	01710133          	add	x2,x2,x23
  400148:	01810133          	add	x2,x2,x24
  40014c:	01910133          	add	x2,x2,x25
  400150:	01a10133          	add	x2,x2,x26
  400154:	01b10133          	add	x2,x2,x27
  400158:	01c10133          	add	x2,x2,x28
  40015c:	01d10133          	add	x2,x2,x29
  400160:	01e10133          	add	x2,x2,x30
  400164:	01f10133          	add	x2,x2,x31
  400168:	00700513          	addi	x10,x0,7
  40016c:	00a00513          	addi	x10,x0,10
  400170:	00000073          	ecall

Disassembly of section .data:

10000000 <data>:
10000000:	ffff                	0xffff
10000002:	ffff                	0xffff
10000004:	fffe                	c.fswsp	f31,252(x2)
10000006:	ffff                	0xffff
10000008:	fffd                	c.bnez	x15,10000006 <data+0x6>
1000000a:	ffff                	0xffff
1000000c:	fffc                	c.fsw	f15,124(x15)
1000000e:	ffff                	0xffff
10000010:	fff5                	c.bnez	x15,1000000c <data+0xc>
10000012:	ffff                	0xffff
10000014:	fff4                	c.fsw	f13,124(x15)
10000016:	ffff                	0xffff
10000018:	fffffff3          	csrrci	x31,0xfff,31
1000001c:	fff2                	c.fswsp	f28,252(x2)
1000001e:	ffff                	0xffff
10000020:	ffffffeb          	0xffffffeb
10000024:	ffea                	c.fswsp	f26,252(x2)
10000026:	ffff                	0xffff
10000028:	ffe9                	c.bnez	x15,10000002 <data+0x2>
1000002a:	ffff                	0xffff
1000002c:	ffe8                	c.fsw	f10,124(x15)
1000002e:	ffff                	0xffff
10000030:	ffe1                	c.bnez	x15,10000008 <data+0x8>
10000032:	ffff                	0xffff
10000034:	ffe0                	c.fsw	f8,124(x15)
10000036:	ffff                	0xffff
10000038:	ffdf ffff ffde      	0xffdeffffffdf
1000003e:	ffff                	0xffff
	...
