.data
car: .ascii "a"
result: .word 0
.code
 ld $a0,car($zero)
 jal ES_VOCAL
 sd $v0,result($zero)
 halt

ES_VOCAL: dadd $t1,$0,$0
 slti $t0,$a0,65
 bnez $t0 ,NOESLETRA
 slti $t0,$a0,90
 daddi $t1,$t1,1
 beqz $t0 ,ESLETRA
 slti $t0,$a0,97
 bnez $t0 ,NOESLETRA
 slti $t0,$a0,122
 slti $t1,$t1,-1
 beqz $t0 ,ESLETRA
 bnez $t0 ,NOESLETRA
NOESLETRA: dadd $v0,$0,$0
 j fin
ESLETRA: slti $t0,$t1,1
 beqz $t0 ,MAYUS
 daddi $t1,$t1,97
 beq $a0,$t1, VOCAL
 daddi $t1,$t1,4
 beq $a0,$t1, VOCAL
 daddi $t1,$t1,4
 beq $a0,$t1, VOCAL
 daddi $t1,$t1,6
 beq $a0,$t1, VOCAL
 daddi $t1,$t1,6
 beq $a0,$t1, VOCAL
 j fin
VOCAL: daddi $v0,$v0,1
 j fin
MAYUS: daddi $t1,$t1,65
 beq $a0,$t1, VOCAL
 daddi $t1,$t1,4
 beq $a0,$t1, VOCAL
 daddi $t1,$t1,4
 beq $a0,$t1, VOCAL
 daddi $t1,$t1,6
 beq $a0,$t1, VOCAL
 daddi $t1,$t1,6
 beq $a0,$t1, VOCAL

fin: jr $ra