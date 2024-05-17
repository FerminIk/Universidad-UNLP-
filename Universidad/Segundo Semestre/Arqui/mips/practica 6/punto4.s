.data
CONTROL: .word32 0x10000
DATA: .word32 0x10008
mens: .asciiz "bienvenido"
error: .asciiz "ERROR"
clave: .asciiz "abcd"
datos: .byte 0

.text
lwu $s0,CONTROL($0)
lwu $s1,DATA($0)
jal char
daddi $t2,$0,0
jal  respuesta

char: daddi $t0,$0,9
loop: sd $t0,0($s0)
lb $t3,0($s1)
daddi $t2,$t2,1
sd $t3,datos($t4)
daddi $t4,$t4,8
slti $t1,$t2,4
bnez $t1,loop
lb $t3,0($0)
sd $t3,datos($t4)
jr $ra

respuesta: daddi $t0,$0,0
daddi $t1,$0,0
daddi $t3,$0,0
lb $t0,clave($v1)
lb $t1,datos($t2)
slt $t3,$t0,$t1
bnez $t3,error
daddi $v1,$v1,1
daddi $t2,$t2,8
slti $t3,$v1,4
bnez $t3, respuesta
daddi $s3,$zero,mens
j imprimir
error: daddi $s3,$zero,error
imprimir: daddi $t5,$t5,4
sd $s3,0($s1)
sd $t5,0($s0)
fin: jr $ra