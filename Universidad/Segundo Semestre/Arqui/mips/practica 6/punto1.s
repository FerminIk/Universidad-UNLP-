.data
caracter: .byte 0
CONTROL: .word32 0x10000
DATA: .word32 0x10008

.text
lwu $s0, DATA($zero) 
lwu $s1, CONTROL($zero) 
daddi $s2,$zero,caracter
daddi $t4,$0,30
jal leer
halt 

leer: daddi $t0, $0,9
sd $t0,0($s1)
daddi $t0,$0,0
lbu $t1,0($s0)
dsub $t3,$t1,$t4
beqz $t3,fin

sb $t1,caracter($zero)
daddi $t0,$0,4

sd $s2,0($s0)

sd $t0,0($s1)

ld $t3,0($zero)

j leer

fin: jr $ra

