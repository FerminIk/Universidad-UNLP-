    .data
tabla_1:    .word   21,12,5,16,8,39,10,41,4,33
tabla_2:    .word   32,24,15,32,17,28,11,20,44,21
rojo:       .word32 255, 0, 0, 0
verde:      .word32 0, 255, 0, 0
CONTROL:    .word32 0x10000
DATA:       .word32 0x10008

    .code
    lwu $s6, CONTROL($zero)
    lwu $s7, DATA($zero)
    daddi $s5, $zero, 5

    daddi $a1, $zero, 10

    daddi $a0, $zero, tabla_1
    jal MIN_MAX
    dadd $s0, $zero, $v0            ; minimo tabla_1
    dadd $s1, $zero, $v1            ; maximo talba_1

    daddi $a0, $zero, tabla_2
    jal MIN_MAX
    dadd $s2, $zero, $v0            ; minimo talba_2
    dadd $s3, $zero, $v1            ; maximo tabla_2

    lwu $t0, rojo($zero)
    sb $s0, 5($s7)
    sb $s3, 4($s7)
    sw $t0, 0($s7)
    sd $s5, 0($s6)

    lwu $t0, verde($zero)
    sb $s2, 5($s7)
    sb $s1, 4($s7)
    sw $t0, 0($s7)
    sd $s5, 0($s6)

    halt

MIN_MAX:
    daddi $v0, $zero, 9999          ; inicializo minimo en numero muy grande
    daddi $v1, $zero, -9999         ; inicializo máximo en numero muy chico
    dadd $t0, $zero, $a0
    dadd $t3, $zero, $a1
loop:
    ld $t1, 0($t0)
    daddi $t0, $t0, 8
    daddi $t3, $t3, -1
    slt $t2, $t1, $v0
    beqz $t2, sig1                 ; si el mínimo es mas chico que mi numero actual no hago nada
    dadd $v0, $zero, $t1            ; sino actualizo el mínimo con el valor actual
sig1:
    slt $t2, $v1, $t1
    beqz $t2, sig2                  ; si el máximo es mayor que mi numero actual no hago nada
    dadd $v1, $zero, $t1            ; sino actualizo el másximo con el valor actualizo
sig2:
    bnez $t3, loop
    jr $ra
