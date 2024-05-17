.data
control: .word32 0x10000
data: .word32 0x10008
color: .byte 255,0,0,0
.code
lwu $s0,control($zero)
lwu $s1,data($zero)
lwu  $s2,color($zero)
daddi $t0,$t0,9
jal leer
daddi $t0,$0,0
daddi $t1,$0,0
j imprimir

leer: sd $t0,0($s0)
lbu $t1,0($s1)
daddi $t1,$t1,-0x30
slti $t2,$t1,0
bnez $t2,leer
daddi $t1,$t1,-10
slti $t2,$t1,0
beqz $t2,leer
daddi $v0,$t1,10
jr $ra

imprimir: sw $s2,0($s1)
daddi $t0,$t0,5
beqz $v0,cero
daddi $v0,$v0,-1
beqz $v0,uno
daddi $v0,$v0,-1
beqz $v0,dos
daddi $v0,$v0,-1
beqz $v0,tres
daddi $v0,$v0,-1
beqz $v0,cuatro
daddi $v0,$v0,-1
beqz $v0,cinco
daddi $v0,$v0,-1
beqz $v0,seis
daddi $v0,$v0,-1
beqz $v0,siete
daddi $v0,$v0,-1
beqz $v0,ocho
j nueve
cero: jal lineaInf
jal lineaSup
jal lineaInfIzq
jal lineaInfDer
jal lineaSupIzq
jal lineaSupDer
jal puntoCenIzq
jal puntoCenDer
j fin

uno: jal lineaInfDer
jal lineaSupDer
jal puntoCenDer
jal puntoCenDer
j fin

dos: jal lineaInf
jal lineaSup
jal lineaDiag
jal lineaSupIzq
jal puntoInfDer
jal puntoInfIzq
j fin

tres: jal lineaInf
jal lineaSup
jal lineaCen
jal lineaSupDer
jal lineaInfDer
j fin

cuatro: jal lineaCen
jal lineaInfDer
jal lineaSupDer
jal lineaSupIzq
jal puntoCenDer
jal puntoCenIzq
j fin

cinco: jal lineaInf
jal lineaSup
jal lineaCen
jal lineaSupIzq
jal lineaInfDer
jal puntoCenIzq
jal puntoSupIzq
jal puntoSupDer
jal puntoInfIzq
j fin

seis: jal lineaInf
jal lineaSup
jal lineaCen
jal lineaInfIzq
jal lineaSupIzq
jal lineaInfDer
jal puntoCenIzq
j fin 

siete: jal lineaSup
jal lineaDiag
jal puntoSupIzq
jal puntoSupDer
j fin

ocho: jal lineaInf
jal lineaSup
jal lineaCen
jal lineaInfIzq
jal lineaInfDer
jal lineaSupIzq
jal lineaSupDer
j fin

nueve: jal lineaInf
jal lineaSup
jal lineaCen
jal lineaSupIzq
jal lineaInfDer
jal lineaSupDer
jal puntoCenDer
fin: halt

lineaInf: daddi $t3,$zero,0
daddi $t4,$zero,1
daddi $t5,$zero,0
daddi $t3,$t3,1
loop: daddi $t4,$t4,1
daddi $t5,$t5,1
sb $t4,5($s1)
sb $t3,4($s1)
sd $t0,0($s0)
slti $t6,$t5,3
bnez $t6,loop
jr $ra

lineaSup: daddi $t3,$zero,0
daddi $t4,$zero,1
daddi $t5,$zero,0
daddi $t3,$t3,8
loop1: daddi $t4,$t4,1
daddi $t5,$t5,1
sb $t4,5($s1)
sb $t3,4($s1)
sd $t0,0($s0)
slti $t6,$t5,3
bnez $t6,loop1
jr $ra

lineaCen: daddi $t3,$zero,0
daddi $t4,$zero,1
daddi $t5,$zero,0
daddi $t3,$t3,5
loop2: daddi $t4,$t4,1
daddi $t5,$t5,1
sb $t4,5($s1)
sb $t3,4($s1)
sd $t0,0($s0)
slti $t6,$t5,3
bnez $t6,loop2
jr $ra

lineaInfIzq: daddi $t3,$zero,1
daddi $t5,$zero,0
daddi $t4,$0,1
loop3: daddi $t3,$t3,1
daddi $t5,$t5,1
sb $t4,5($s1)
sb $t3,4($s1)
sd $t0,0($s0)
slti $t6,$t5,3
bnez $t6,loop3
jr $ra

lineaInfDer: daddi $t3,$zero,1
daddi $t4,$zero,5
daddi $t5,$zero,0
loop4: daddi $t3,$t3,1
daddi $t5,$t5,1
sb $t4,5($s1)
sb $t3,4($s1)
sd $t0,0($s0)
slti $t6,$t5,3
bnez $t6,loop4
jr $ra

lineaSupIzq: daddi $t3,$zero,5
daddi $t5,$zero,0
daddi $t4,$0,1
loop5: daddi $t3,$t3,1
daddi $t5,$t5,1
sb $t4,5($s1)
sb $t3,4($s1)
sd $t0,0($s0)
slti $t6,$t5,2
bnez $t6,loop5
jr $ra

lineaSupDer: daddi $t3,$zero,5
daddi $t4,$zero,5
daddi $t5,$zero,0
loop6: daddi $t3,$t3,1
daddi $t5,$t5,1
sb $t4,5($s1)
sb $t3,4($s1)
sd $t0,0($s0)
slti $t6,$t5,2
bnez $t6,loop6
jr $ra

lineaDiag: daddi $t3,$zero,1
daddi $t4,$zero,0
daddi $t5,$zero,0
loop7: daddi $t3,$t3,1
daddi $t4,$t4,1
daddi $t5,$t5,1
sb $t4,5($s1)
sb $t3,4($s1)
sd $t0,0($s0)
slti $t6,$t5,3
bnez $t6,loop7
daddi $t3,$t3,1
daddi $t4,$t4,1
daddi $t5,$t5,1
sb $t4,5($s1)
sb $t3,4($s1)
sd $t0,0($s0)
daddi $t3,$t3,1
daddi $t4,$t4,1
daddi $t5,$t5,1
sb $t4,5($s1)
sb $t3,4($s1)
sd $t0,0($s0)
daddi $t3,$t3,1
sb $t4,5($s1)
sb $t3,4($s1)
sd $t0,0($s0)
jr $ra

puntoCenIzq: daddi $t3,$zero,5
daddi $t4,$zero,1
sb $t4,5($s1)
sb $t3,4($s1)
sd $t0,0($s0)
jr $ra

puntoCenDer: daddi $t3,$zero,5
daddi $t4,$zero,5
sb $t4,5($s1)
sb $t3,4($s1)
sd $t0,0($s0)
jr $ra

puntoInfIzq: daddi $t3,$zero,1
daddi $t4,$zero,1
sb $t4,5($s1)
sb $t3,4($s1)
sd $t0,0($s0)
jr $ra

puntoInfDer: daddi $t3,$zero,1
daddi $t4,$zero,5
sb $t4,5($s1)
sb $t3,4($s1)
sd $t0,0($s0)
jr $ra

puntoSupIzq: daddi $t3,$zero,8
daddi $t4,$zero,1
sb $t4,5($s1)
sb $t3,4($s1)
sd $t0,0($s0)
jr $ra

puntoSupDer: daddi $t3,$zero,8
daddi $t4,$zero,5
sb $t4,5($s1)
sb $t3,4($s1)
sd $t0,0($s0)
jr $ra