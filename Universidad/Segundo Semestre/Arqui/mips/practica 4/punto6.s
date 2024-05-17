.data
	A: .word 15
	B: .word 8
	C: .word 8
	D: .word 0
.code
	ld r1,A(r0)
	ld r2,B(r0)
	ld r3,C(r0)
	dadd r4,r0,r0
	dadd r5,r0,r0
	dsub r4,r1,r2
	bnez r4, no_es
	daddi r5,r5,2
	dadd r4,r0,r0
	dsub r4,r1,r3
	bnez r4, no_es1
	daddi r5,r5,1
	j  fin
no_es:	dsub r4,r1,r3
	bnez r4, no_es1
	daddi r5,r5,2
	dadd r4,r0,r0
no_es1:	dsub r4,r2,r3
	bnez r4, fin
	bnez r5, no_prim
	daddi r5,r5,2
	j  fin
no_prim:	daddi r5,r5,1
	j  fin


fin:	sd r5,D(r0)
	halt
