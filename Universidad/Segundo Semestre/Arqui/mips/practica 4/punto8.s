.data
	num: .word 5
	mul: .word 3
	res: .word 0
.code
	ld r1,num(r0)
	ld r2,mul(r0)
	ld r3,res(r0)
loop:	daddi r2,r2,-1
	bnez r2,loop
	dadd  r3,r3,r1
	sd r3,res(r0)
	halt
