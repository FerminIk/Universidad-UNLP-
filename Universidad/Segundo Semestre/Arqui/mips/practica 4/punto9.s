.data
	a: .word 5
	x: .word 3
	y: .word 1
.code
	ld r1,a(r0)
	ld r2,x(r0)
	ld r3,y(r0)
loop:	beqz r1, fin
	daddi r1,r1,-1
	dadd r2,r2,r3	
	bnez r1, loop
	nop

fin:	halt
