.data
num1: .byte 0
num2: .byte 0
res: .byte 0
CONTROL: .word32 0x10000
DATA: .word32 0x10008

.text
 lwu $s0,CONTROL($zero)
 lwu $s1,DATA($zero)
 jal ingreso
 lbu $a0,num1($0)
 lbu $a1,num2($0)
 jal muestra
 halt

ingreso: daddi $t0,$0,9
loop: sd $t0,0($s0)
 lbu $v0,0($s1)
 daddi $v0,$v0, - 0x30
 slti $t1,$v0,0
 bnez $t1,loop
 daddi $v0,$v0,-10
 slti $t1,$v0,10
 beqz $t1,loop
 daddi $v0,$v0,10
 sd $v0,num1($t2)
 daddi $t3,$t3,1
 slti $t1,$t3,2
 beqz $t1,fin
 daddi $t2,$t2,8
 daddi $v1,$v0,0
 j loop
fin: jr $ra

muestra: daddi $s2,$0,res
ld $t0,0($0)
ld $t1,0($0)
ld $t2,0($0)
ld $t3,0($0)
dadd $t0,$a0,$a1
sd $t0,0($s1)
daddi $t1,$0,1
sd $t1,0($s0)
jr $ra

