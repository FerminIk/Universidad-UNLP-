.data
CONTROL: .word32 0x10000
DATA: .word32 0x10008 
caracter: .byte 0
cero: .asciiz "cero"
uno: .asciiz "uno"
dos: .asciiz "dos"
tres: .asciiz "tres"
cuatro: .asciiz "cuatro"
cinco: .asciiz "cinco"
seis: .asciiz "seis"
siete: .asciiz "siete"
ocho: .asciiz "ocho"
nueve: .asciiz "nueve"

.text
 lwu $s0,DATA($zero)
 lwu $s1,CONTROL($zero)
 jal ingreso
 sd $v0,caracter($zero)
 jal  muestra
 halt
ingreso: daddi $t0,$0,9
sd $t0,0($s1)
lbu $v0,0($s0)
daddi $v0,$v0,- 0x30
slti $t1,$v0,0
bnez $t1,ingreso
daddi $v0,$v0,- 10
slti $t1,$v0,0
beqz $t1,ingreso
jr $ra

muestra: ld $t1,0($zero)
daddi $t1,$v0,10
slti $t2,$t1,1
bnez $t2, cero
slti $t2,$t1,2
bnez $t2, uno
slti $t2,$t1,3
bnez $t2, dos
slti $t2,$t1,4
bnez $t2, tres
slti $t2,$t1,5
bnez $t2, cuatro
slti $t2,$t1,6
bnez $t2, cinco
slti $t2,$t1,7
bnez $t2, seis
slti $t2,$t1,8
bnez $t2, siete
slti $t2,$t1,9
bnez $t2, ocho
nueve: daddi $t3,$0,nueve
j imprimir
ocho: daddi $t3,$zero,ocho
j imprimir
siete: daddi $t3,$zero,siete
j imprimir
seis: daddi $t3,$zero,seis
j imprimir
cinco: daddi $t3,$zero,cinco
j imprimir
cuatro: daddi $t3,$zero,cuatro
j imprimir
tres: daddi $t3,$zero,tres
j imprimir
dos: daddi $t3,$zero,dos
j imprimir
uno: daddi $t3,$zero,uno
j imprimir
cero: daddi $t3,$zero,cero
imprimir: sd $t3,0($s0)
daddi $t4,$0,4
sd $t4,0($s1)
fin: jr $ra

