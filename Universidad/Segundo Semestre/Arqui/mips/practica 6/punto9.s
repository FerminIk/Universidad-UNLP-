.data
texto: .asciiz "...." 
cant: .word 4
CONTROL: .word 0x10000
DATA: .word 0x10008

.text
ld $s0, DATA($zero) 
ld $s1, CONTROL($zero) 
daddi $t1,$0,4
daddi $t0,$0,9
daddi $t3,$0,0
loop: sd $t0,0($s1)
lbu $t4,0($s0)
sb $t4,texto($t2)
daddi $t2,$t2,1
daddi $t1,$t1,-1
bnez $t1,loop
daddi $t5,$0,texto
sd $t5,0($s0)
daddi $t6,$0,4
sd $t6,0($s1)
halt