.data
cadena: .word 1,2,3,4,0
cant: .word 0
.code
	daddi $t0,$0,0
loop:	lbu $a0,cadena($t0)
	beqz $a0,fin
	jal ES_IMPAR
	daddi $t0,$t0,8
	dadd $t4,$t4,$v0
	j loop
fin: 	sd $t4,cant($zero)
	halt


ES_IMPAR: dadd $v0,$0,$0
	daddi $t2,$a0,0
	daddi $t3,$zero,4
lazo:	daddi $t2,$t2,-2
	beqz $t2,par
	daddi $t3,$t3,-1
	beqz $t3,final
	j lazo
par: 	daddi $v0,$v0,1
final:	jr $ra