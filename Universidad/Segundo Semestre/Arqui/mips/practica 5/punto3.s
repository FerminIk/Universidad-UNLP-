.data
base: .double 5.83
altura: .double 13.47
superficie: .double 0.0
 .code
 l.d f1,base(r0)
 l.d f2, altura(r0)
 daddi r2,r0,2
 mtc1 r2 , f5
 cvt.d.l f6,f5
 mul.d f3,f2,f1
 div.d f4,f3,f5
 s.d f4, superficie(r0)
 halt 
