{Teniendo en cuenta la tabla, calcular la memoria estatica, dinamica y el tiempo de ejecución.

Tipo de dato	Memoria
Char			1 byte
Integer			6 byte
Real			8 byte
Boolean			1 byte
String			Longitud + 1 byte
Puntero			4 byte
}

program Ejercicio5;
Const 
  dimF = 100;
Type

  rango = 1..dimF;
  vector = array [rango] of ^real;
  
Var	           //MEMORIA ESTÁTICA = 400 + 12 = 412b
  v: vector;	   {Vector de Punteros: 4 * 100 = 400}
  dimL, i: integer;        {Dos variabes integer: 6 + 6 = 12}
  
Begin	           //MEMORIA DINÁMICA = 600 - 300 = 300b
  dimL:= 50;
  for i:= 1 to dimL do begin
    new(v[i]);	            {(4(nodo) + 8(real)) * 50 = 600b}
    read(v[i]^);
  end;
  for i:= 1 to 25 do 
    v[i]:=nil;					
  for i:=26 to 50 do
    dispose(v[i]);	            {-(4(nodo) + 8(real) * 25 = 300b}
End.
{MEMORIA TOTAL = 300b + 412b = 712b}


begin
  dimL:=50;				{1}
  
  for i:=1 to dimL do begin 		{3*50 + 2 = 152}
    new(v[i]);				
    read(v[i]^);
  end;
  
  for i:=1 to 25 do 		{3*25 + 2 = 77}	
    v[i]:= nil;			{1*25 = 25}
	
  for i:=26 to 50 do		{3*25 + 2 = 77}
    dispose(v[i]);			
end.
{TIEMPO DE EJECUCIÓN TOTAL = 1 + 152 + 77 + 25 + 77 = 332}

