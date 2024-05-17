.data
coorX: .byte 0
coorY: .byte 0
color: .byte 255, 0, 255, 0 
CONTROL: .word32 0x10000
DATA: .word32 0x10008

.text
lwu $s0, CONTROL($zero) 
lwu $s1, DATA($zero) 
daddi $t0, $zero, 7 
sd $t0, 0($s0) 
jal leerCoor
sb $v0, 5($s1)  
sb $v1, 4($s1) 
lwu $s2, color($zero) ; $s2 = valor de color a pintar
sw $s2, 0($s1) ; DATA recibe el valor del color a pintar
daddi $t0, $zero, 5 ; $t0 = 5 -> función 5: salida gráfica
sd $t0, 0($s0) ; CONTROL recibe 5 y produce el dibujo del punto
halt 

leerCoor: daddi $t1,$t1,8
loop: sd $t1, 0($s0)
lbu $v0,0($s1)
slti $t2,$v0,50
beqz $t2,loop
y: sd $t1, 0($s0)
lbu $v1,0($s1)
slti $t2,$v1,50
beqz $t2,y
jr $ra