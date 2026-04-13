
447inputs/beqtest.elf:     file format elf32-littleriscv


Disassembly of section .text:

00400000 <main>:
  400000:	0f000413          	addi	x8,x0,240
  400004:	00f00493          	addi	x9,x0,15
  400008:	0f000593          	addi	x11,x0,240
  40000c:	04000263          	beq	x0,x0,400050 <here>

00400010 <back>:
  400010:	00940c63          	beq	x8,x9,400028 <nottaken>
  400014:	02000063          	beq	x0,x0,400034 <taken1>
  400018:	00100093          	addi	x1,x0,1
  40001c:	00a00513          	addi	x10,x0,10
  400020:	00000073          	ecall

00400024 <there>:
  400024:	04000863          	beq	x0,x0,400074 <where>

00400028 <nottaken>:
  400028:	00200093          	addi	x1,x0,2
  40002c:	00a00513          	addi	x10,x0,10
  400030:	00000073          	ecall

00400034 <taken1>:
  400034:	00b40863          	beq	x8,x11,400044 <taken2>
  400038:	00300093          	addi	x1,x0,3
  40003c:	00a00513          	addi	x10,x0,10
  400040:	00000073          	ecall

00400044 <taken2>:
  400044:	00400093          	addi	x1,x0,4
  400048:	00a00513          	addi	x10,x0,10
  40004c:	00000073          	ecall

00400050 <here>:
  400050:	00940063          	beq	x8,x9,400050 <here>
  400054:	fe940ee3          	beq	x8,x9,400050 <here>
  400058:	fe940ce3          	beq	x8,x9,400050 <here>
  40005c:	fe940ae3          	beq	x8,x9,400050 <here>
  400060:	00940863          	beq	x8,x9,400070 <next>
  400064:	00940663          	beq	x8,x9,400070 <next>
  400068:	00940463          	beq	x8,x9,400070 <next>
  40006c:	00940263          	beq	x8,x9,400070 <next>

00400070 <next>:
  400070:	fa000ae3          	beq	x0,x0,400024 <there>

00400074 <where>:
  400074:	f8000ee3          	beq	x0,x0,400010 <back>
