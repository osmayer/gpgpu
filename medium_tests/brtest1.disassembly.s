
447inputs/brtest1.elf:     file format elf32-littleriscv


Disassembly of section .text:

00400000 <main>:
  400000:	00a00513          	addi	x10,x0,10

00400004 <label0>:
  400004:	00100293          	addi	x5,x0,1
  400008:	0180006f          	jal	x0,400020 <label1>
  40000c:	70050513          	addi	x10,x10,1792
  400010:	00006013          	ori	x0,x0,0
  400014:	00006013          	ori	x0,x0,0
  400018:	06400293          	addi	x5,x0,100
  40001c:	00000073          	ecall

00400020 <label1>:
  400020:	02001263          	bne	x0,x0,400044 <label3>
  400024:	00006013          	ori	x0,x0,0
  400028:	00006013          	ori	x0,x0,0
  40002c:	53900313          	addi	x6,x0,1337

00400030 <label2>:
  400030:	00000e63          	beq	x0,x0,40004c <label4>
  400034:	00006013          	ori	x0,x0,0
  400038:	00006013          	ori	x0,x0,0
  40003c:	34700393          	addi	x7,x0,839
  400040:	00000073          	ecall

00400044 <label3>:
  400044:	40400413          	addi	x8,x0,1028
  400048:	00000073          	ecall

0040004c <label4>:
  40004c:	44700393          	addi	x7,x0,1095
  400050:	00000073          	ecall
