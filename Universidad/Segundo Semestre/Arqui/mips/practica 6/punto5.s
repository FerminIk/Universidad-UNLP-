.data
control: .word32 0x10000
data: .word32 0x10008
msj: .asciiz "Introduzca un valor"
msj1: .asciiz "Ahora introduzca su exponente"
valor: .double 0
exponente: .byte 0
.text
lwu $s0,control($0)
daddi $a1,$0,4
daddi $a0,$0,8
lwu $s1,data($0)
daddi $s2,$0,msj
daddi $s3,$0,msj1
jal leer
daddi  $a0,$0,0
jal a_la_potencia
halt

leer: sd $s2,0($s1)
sd $a1,0($s0)
sd $a0,0($s0)
lbu $v0,0($s1)
sd $s3,0($s1)
sd $a1,0($s0)
sd $a0,0($s0)
lbu $v1,0($s1)
jr $ra

a_la_potencia: daddi $a0,$0,1
beqz $v1,potenciaCero
loop: daddi $v1,$v1,-1
beqz $v1,imprimir
dadd $v0,$v0,$v0
j loop
potenciaCero: daddi $v0,$0,0
daddi $v0,$0,1
imprimir: sd $v0,0($s1)
sd $a0,0($s0)
jr $ra

