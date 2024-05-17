.data
	cadena: .asciiz "adbdcdedfdgdhdid" 
	car: .asciiz "d" 
	cant: .word 0 
.code
	dadd r2,r0,r0
	lbu r3,car(r0)
	ld r4,cant(r0)
	daddi r5,r0,16

loop:	dadd r6,r0,r0
	beqz r5,fin
	lbu r1,cadena(r2)
	daddi r5,r5,-1
	daddi r2,r2,1
	dsub r6, r1, r3
	bnez r6, loop
	daddi r4,r4,1
	j loop	
fin:	sd r4,cant(r0)
	halt
	