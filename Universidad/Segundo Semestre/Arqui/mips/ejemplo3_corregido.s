.data
CONTROL: .word32 0x10000
DATA: .word32 0x10008
tabla1: .word 20, 1, 14, 7, 2, 58, 18, 7, 12, 11
tabla2: .word 7, 7, 7, 7, 7, 7, 7, 7, 7, 7
long: .word 10
       .code
       dadd $t2, $zero, $zero
       daddi $t5, $zero, 1
       ld $t0, long($zero)
       daddi $t4, $t4, 0
       
loop:  ld $t1, tabla2($t2)
       ld $t3, tabla1($t2)
       bne $t3, $t1, sigo
       lwu $s0, DATA($zero)
       sd $t5, 0($s0); $s0 = dirección de DATA $t5 el numero de pos en la tabla 
       lwu $s1, CONTROL($zero)
       daddi $t6, $zero, 2
       sd $t6, 0($s1); $s1 = dirección de CONTROL y en $t6 el comando 2 - imprimir numero

sigo:  daddi $t0, $t0, -1
       daddi $t2, $t2, 8
       daddi $t5, $t5, 1
       bnez $t0, loop
       halt
