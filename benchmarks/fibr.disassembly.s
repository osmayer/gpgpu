
benchmarks/fibr.elf:     file format elf32-littleriscv


Disassembly of section .text:

00400000 <_start>:
  400000:	068000ef          	jal	x1,400068 <main>
  400004:	00050113          	addi	x2,x10,0
  400008:	00058193          	addi	x3,x11,0
  40000c:	00a00513          	addi	x10,x0,10
  400010:	00000073          	ecall

00400014 <fibr>:
  400014:	ff010113          	addi	x2,x2,-16
  400018:	00112623          	sw	x1,12(x2)
  40001c:	00812423          	sw	x8,8(x2)
  400020:	00912223          	sw	x9,4(x2)
  400024:	00050413          	addi	x8,x10,0
  400028:	00050663          	beq	x10,x0,400034 <fibr+0x20>
  40002c:	00100793          	addi	x15,x0,1
  400030:	00f51e63          	bne	x10,x15,40004c <fibr+0x38>
  400034:	00040513          	addi	x10,x8,0
  400038:	00c12083          	lw	x1,12(x2)
  40003c:	00812403          	lw	x8,8(x2)
  400040:	00412483          	lw	x9,4(x2)
  400044:	01010113          	addi	x2,x2,16
  400048:	00008067          	jalr	x0,0(x1)
  40004c:	fff50513          	addi	x10,x10,-1
  400050:	fc5ff0ef          	jal	x1,400014 <fibr>
  400054:	00050493          	addi	x9,x10,0
  400058:	ffe40513          	addi	x10,x8,-2
  40005c:	fb9ff0ef          	jal	x1,400014 <fibr>
  400060:	00a48433          	add	x8,x9,x10
  400064:	fd1ff06f          	jal	x0,400034 <fibr+0x20>

00400068 <main>:
  400068:	ff010113          	addi	x2,x2,-16
  40006c:	00112623          	sw	x1,12(x2)
  400070:	00f00513          	addi	x10,x0,15
  400074:	00000097          	auipc	x1,0x0
  400078:	fa0080e7          	jalr	x1,-96(x1) # 400014 <fibr>
  40007c:	00c12083          	lw	x1,12(x2)
  400080:	01010113          	addi	x2,x2,16
  400084:	00008067          	jalr	x0,0(x1)
  400088:	0000                	unimp
	...
