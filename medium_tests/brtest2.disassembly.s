
447inputs/brtest2.elf:     file format elf32-littleriscv


Disassembly of section .text:

00400000 <main>:
  400000:	00a00513          	addi	x10,x0,10
  400004:	00100193          	addi	x3,x0,1
  400008:	fff00213          	addi	x4,x0,-1
  40000c:	12300293          	addi	x5,x0,291
  400010:	00c0006f          	jal	x0,40001c <label1>

00400014 <label0>:
  400014:	001282b3          	add	x5,x5,x1
  400018:	00000863          	beq	x0,x0,400028 <label2>

0040001c <label1>:
  40001c:	00728293          	addi	x5,x5,7
  400020:	ff5ff0ef          	jal	x1,400014 <label0>
  400024:	0340006f          	jal	x0,400058 <label8>

00400028 <label2>:
  400028:	00928293          	addi	x5,x5,9
  40002c:	00419663          	bne	x3,x4,400038 <label4>

00400030 <label3>:
  400030:	00528293          	addi	x5,x5,5
  400034:	00005a63          	bge	x0,x0,400048 <label6>

00400038 <label4>:
  400038:	00b28293          	addi	x5,x5,11
  40003c:	fe305ae3          	bge	x0,x3,400030 <label3>

00400040 <label5>:
  400040:	06328293          	addi	x5,x5,99
  400044:	fe3046e3          	blt	x0,x3,400030 <label3>

00400048 <label6>:
  400048:	06f28293          	addi	x5,x5,111
  40004c:	00008067          	jalr	x0,0(x1)

00400050 <label7>:
  400050:	0c828293          	addi	x5,x5,200
  400054:	00000073          	ecall

00400058 <label8>:
  400058:	0d728293          	addi	x5,x5,215
  40005c:	00c000ef          	jal	x1,400068 <label10>

00400060 <label9>:
  400060:	00128293          	addi	x5,x5,1
  400064:	00000073          	ecall

00400068 <label10>:
  400068:	1bf28293          	addi	x5,x5,447
  40006c:	00024663          	blt	x4,x0,400078 <label12>

00400070 <label11>:
  400070:	19028293          	addi	x5,x5,400
  400074:	00000073          	ecall

00400078 <label12>:
  400078:	001282b3          	add	x5,x5,x1
  40007c:	fe025ae3          	bge	x4,x0,400070 <label11>

00400080 <label13>:
  400080:	63d28293          	addi	x5,x5,1597
  400084:	00000073          	ecall
