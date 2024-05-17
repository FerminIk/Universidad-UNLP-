.data
valor: .word 10
result: .word 0
.code
 daddi $sp, $0, 0x400 ; Inicializa el puntero al tope de la pila
 ld $a0, valor($0)
 jal factorial
 sd $v0, result($0)
 halt

factorial: daddi $sp, $sp, -16
sd $ra, 0($sp)
sd $s0, 8($sp)
beqz $a0, fin_rec
dadd $s0, $0, $a0
daddi $a0, $a0, -1
jal factorial
dmul $v0, $v0, $s0
j fin
fin_rec: daddi $v0, $0, 1
fin: ld $s0, 8($sp)
ld $ra, 0($sp)
 daddi $sp, $sp, 16
jr $ra 