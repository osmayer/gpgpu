
447inputs/brtest0.elf:     file format elf32-littleriscv


Disassembly of section .text:

00400000 <main>:
  400000:	00a00513          	addi	x10,x0,10
  400004:	00000393          	addi	x7,x0,0

00400008 <label0>:
  400008:	0080006f          	jal	x0,400010 <label1>
  40000c:	32738393          	addi	x7,x7,807

00400010 <label1>:
  400010:	00001863          	bne	x0,x0,400020 <label3>

00400014 <label2>:
  400014:	00000863          	beq	x0,x0,400024 <label4>
  400018:	34738393          	addi	x7,x7,839
  40001c:	00000073          	ecall

00400020 <label3>:
  400020:	33738393          	addi	x7,x7,823

00400024 <label4>:
  400024:	70d38393          	addi	x7,x7,1805
  400028:	00000073          	ecall
