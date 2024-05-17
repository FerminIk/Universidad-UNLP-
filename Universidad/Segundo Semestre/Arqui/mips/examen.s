.data
control: .word32 0x10000
data: .word32 0x10008
msj: .asciiz "El resultado es:"
valor: .double 12.0
x: .double 0
.code
lwu $s0,control($0)
lwu $s1,data($0)
daddi $t0,$0,8
sd $t0,0($s0)
lbu $a0,0($s1)
ld $t2,valor($0)
slt $a1,$a0,$t2
beqz $a1, mayor
dsub $t3,$a0,$t2
dmul $t3,$t3,$a0
j imprimir
mayor: dsub $t3,$t2, $a0
ddiv $t3,$t3,$a0
imprimir: daddi $t4,$0,msj

sd $t3,0($s1)
daddi $t0,$0,3
sd $t0,0($s0)
halt
