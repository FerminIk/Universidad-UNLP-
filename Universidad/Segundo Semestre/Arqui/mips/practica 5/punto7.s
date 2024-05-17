.data
numero: .word 10
tabla: .word 14,4,2,5,6,10,2,6
long: .word 8
res: .word 0
 .code
 ld $a0,numero($zero)
 ld $a1,tabla($zero)
 ld $a2,long($zero)
 jal mayores
 sd $v0,res($zero)
 halt 

mayores: daddi $v0,$zero,0
 	daddi $t1,$zero,0
	daddi $t2,$zero,0
loop: 	slt $t2, $a2, $zero
	bnez  $t2, fin
	ld $t0,tabla($t1)
	slt $t3, $t0, $a0
	bnez $t3,sigue
	daddi $v0,$v0,1
sigue: 	daddi $a2,$a2,-1
	daddi $t1,$t1,8
	j  loop
       	
fin:	jr $ra
