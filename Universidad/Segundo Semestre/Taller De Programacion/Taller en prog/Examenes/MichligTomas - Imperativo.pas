program MichligTomas;

type
	cadena= string[20];

	impresion = record
		cantP: integer;
		nombre: cadena;
		kb: real;
		usuario:cadena;
	end;
	
	impresiones = record
		usuario:cadena;
		cantDoc:integer;
		cantPag:integer;
		ultimo:cadena;
	end;
	
	
	ultimo = record
		usuario:cadena;
		doc:cadena;
	end;
	
	
	vector = array[1..20] of ultimo;

	arbol= ^nodoArb;
	nodoArb = record
		dato: impresiones;
		HI: arbol;
		HD: arbol;	
	end;

procedure leerRegistro(var reg:impresion);
begin 
	write('Ingrese los Kb'); readln(reg.kb);
	if(reg.kb <> 0) then begin
		write('Ingrese cantidad de paginas: '); readln(reg.cantP);
		write('Ingrese el nombre de documento: '); readln(reg.nombre);
		write('Ingrese el nombre de usuario: '); readln(reg.usuario);
	end;
end;

 procedure InsertarEnArbol(var a: arbol; reg: impresiones);
begin 
 if (a = nil) then begin
 new(a);
a^.dato:= reg;
a^.HI:= nil;
a^.HD:= nil;
 end
 else
 if (a^.dato.usuario > reg.usuario) then
InsertarEnArbol(a^.HI, reg)
 else if (a^.dato.usuario < reg.usuario) then
InsertarEnArbol(a^.HD, reg)
	else begin
		a^.dato.cantPag := a^.dato.cantPag + reg.cantPag;
		a^.dato.cantDoc:= a^.dato.cantDoc +1;
		a^.dato.ultimo:= reg.ultimo;
	end;

end;

procedure crearABB(var a:arbol);
var
    reg:impresion;i:impresiones;
begin
    write('Ingrese informacion de la impresion: ');
    leerRegistro(reg);
    while(reg.kb <> 0) do begin	  
        i.cantPag:=reg.cantP;
		i.cantDoc:=1;
		i.ultimo:=reg.nombre;
		i.usuario:=reg.usuario; 
       InsertarEnArbol(a,i);
       write('Ingrese informacion de la impresion: ');
       leerRegistro(reg);
    end;
end;

{ejercicio B}
procedure MayorPreorden(a:arbol; var reg:impresiones);{busca el menor dato entero en un arbol de registros p}
begin
 if (a <> nil) then begin
    if(a^.dato.cantDoc > reg.cantDoc) then begin
               reg:=a^.dato;
  end;
 MayorPreorden(a^.HI,reg);
 MayorPreorden(a^.HD,reg);
 end;
end;

{ejercicio C}
procedure agregarOrdenado(var v : vector;   u:ultimo;  diml : integer);

begin

  while(diml > 1) and ( u.usuario < v[diml-1].usuario) do begin
    v[diml].usuario := v[diml-1].usuario;{campo por campo o todo de una?}
	v[diml].doc := v[diml-1].doc;
    diml := diml-1;

  end;
   v[diml].usuario:= u.usuario;
   v[diml].doc:=u.doc;

end;

procedure Intervalo(x,y:cadena; a:arbol;var dimL:integer;var v:vector);
var u:ultimo;
begin{intervalo (x,y)}
	if (a <> nil) then begin
		if(x > a^.dato.usuario) then 
			Intervalo(x,y,a^.HD,dimL,v)
		else if (y < a^.dato.usuario) then
			Intervalo(x,y,a^.HI,dimL,v)
				else begin
					dimL:=dimL+1;
					u.usuario:=a^.dato.usuario;
					u.doc:=a^.dato.ultimo;
					agregarOrdenado(v,u,dimL);
					Intervalo(x,y,a^.HD,dimL,v);
					Intervalo(x,y,a^.HI,dimL,v);
				end;
	end;
end;

{impresiones}
procedure ImprimirVector(v: vector; dimL: integer);
var i:integer;
begin
	for i:= 1 to dimL do begin
		write(v[i].usuario,' | ')
	end;
end;

procedure ImprimirArbolEnOrden(a:arbol);
begin
 if (a <> nil) then begin {usando recursion}
 ImprimirArbolEnOrden(a^.HI);
 writeln(a^.dato.usuario);
 writeln(a^.dato.cantDoc);
 writeln(a^.dato.cantPag);
 writeln(a^.dato.ultimo);
 writeln;
 ImprimirArbolEnOrden(a^.HD);
 end;
end;


var a:arbol; usuarioMax:impresiones;x,y:cadena; v:vector; dimL:integer;
BEGIN
	a:=nil;
	crearABB(a);
	MayorPreorden(a,usuarioMax);
	writeln('usuario con mas documentos', usuarioMax.usuario);
	x:='aaa';
	y:='bbb';
	dimL:=0;
	Intervalo(x,y,a,dimL,v);
	writeln('Arbol:');
	ImprimirArbolEnOrden(a);
	writeln('Vector');
	writeln(dimL);
	ImprimirVector(v,dimL);
	
END.
