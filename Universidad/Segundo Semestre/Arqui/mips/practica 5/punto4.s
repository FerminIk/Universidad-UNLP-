.data
peso: .double 67.83
talla: .double 1.66
infra: .double 18.5
normal: .double 25.0
sobre: .double 30.0
estado: .word 0
 .code
 l.d f1,talla(r0)
 l.d f2,peso(r0)
 l.d f5,infra(r0)
 mul.d f3,f1,f1
 l.d f6,normal(r0)
 l.d f7,sobre(r0)
 div.d f4,f2,f3
 c.le.d f5,f4
 bc1f infrai
 c.lt.d f6,f4
 bc1f normali
 c.lt.d f7,f4
 bc1f sobrei
 bc1t obeso
infrai: daddi r1,r0,1
j fin
normali:  daddi r1,r0,2
j fin
sobrei: daddi r1,r0,3
j fin
obeso: daddi r1,r0,4
j fin
fin: sd r1,estado(r0) 
 halt 
