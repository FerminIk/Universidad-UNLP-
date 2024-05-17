
program problema2;
const
	dimF=6;
type
	nombre=string[20];
	vector=array[1..dimF]of nombre;

procedure leerVector(var v:vector;var dimL:integer);
var
	i:integer;
begin
	i:=1;
	dimL:=0;
	writeln('escibre un nombre');
	readln(v[i]);
	while (dimL<dimF) AND (v[i]<>'ZZZ') do
	begin
		dimL:=dimL+1;
		i:=i+1;
		writeln('escibre un nombre');
		readln(v[i]);
	end;
end;

procedure borrarNombre(var v:vector;var dimL:integer; n:nombre);
var
	i:integer;
	ok:boolean;
begin
	ok:=false;
	i:=1;
	while (i<=dimL) and (not ok) do
	begin
		if (v[i]=n) then
		begin
			for i:=i to dimL do
			begin
				v[i]:=v[i+1];
			end;
			dimL:=dimL-1;
			ok:=true;
		end;
		i:=i+1;
	end;
end;

procedure insertar(var v:vector;var dimL:integer);
var
	n:nombre;
	i:integer;
begin
	writeln('introduzca un nombre para ponerlo en la 4 posicion');
	readln(n);
	if (dimL+1)<=dimF then
	begin
		for i:=dimL downto 4 do
			v[i+1]:=v[i];
		v[4]:=n;
		dimL:=dimL+1;
	end;
end;


procedure agregar(var v:vector; var dimL:integer);
var
	n:nombre;
begin
	writeln('introduzca un nombre para agregarlo');
	readln(n);
	if (dimL+1<=dimF) then
		v[dimL+1]:=n;
	dimL:=dimL+1;
end;

var
	dimL:integer;
	v:vector;
	n:nombre;
	i:integer;
BEGIN
	leerVector(v,dimL);
	writeln('escribe el nombre a borrar');
	readln(n);
	borrarNombre(v,dimL,n);
	for i:=1 to dimL do
		writeln(v[i]);
	insertar(v,dimL);
	for i:=1 to dimL do
		writeln(v[i]);
	agregar(v,dimL);
	for i:=1 to dimL do
		writeln(v[i]);
END.

