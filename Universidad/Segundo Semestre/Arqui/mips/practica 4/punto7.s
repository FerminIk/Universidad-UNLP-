.data
	TABLA: .word 5,6,7,8,9,10,11,12,13,14
	LONG: .word 10
	RES: .word 0
	NUM: .word 10
	CANT: .word 0
.code
	dadd r1 ,r0,r0
	dadd r6,r0,r0
	ld r2, LONG(r0)
	ld r4, NUM(r0)
	ld r5, CANT(r0)
loop:	ld r3, TABLA(r1)
	slt r6, r3 ,r4
	beqz r6,sigue
	daddi r2,r2,-1
	daddi r5,r5,1
sigue:	dadd r6,r0,r0
	bnez r2, loop
	daddi r1,r1,8
	sd r5,CANT(r0)
	halt
