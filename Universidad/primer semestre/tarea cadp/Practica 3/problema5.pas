{
Realizar un programa que lea información de autos que están a la venta en una concesionaria. De cada auto se lee:
marca, modelo y precio. La lectura finaliza cuando se ingresa la marca “ZZZ” que no debe procesarse. La
información se ingresa ordenada por marca. Se pide calcular e informar:
a. El precio promedio por marca.
b. Marca y modelo del auto más caro.
   
}


program problema5;
type
	autoRegistro = record
		marca:string;
		modelo:string;
		precio:real;
	end;

procedure leer (var A:autoRegistro);
begin
		writeln('introduzca la marca del auto');
		readln(a.marca);
		writeln('introduzca el modelo del auto');
		readln(a.modelo);
		writeln('introduzca el precio');
		readln(a.precio);
end;

procedure max (a: autoRegistro; maxMarc:string; maxModelo:string; maxPrecio:real);
begin
	if (a.precio >maxPrecio) then
	begin
		maxMarc:=a.marca;
		maxModelo:=a.modelo;
		maxPrecio:=a.precio;
	end;

end;

function promedio(cant:integer; total:real):real;
begin
	promedio:= cant/total;
end;

procedure Marca(a:autoRegistro; var maxMarc:string; var maxModelo:string; var maxPrecio:real);
var
	marcaAct:string;
	cant:integer;
	total:real;
begin
	marcaAct:=a.marca;
	cant:=0;
	total:=0;
	while (marcaAct = a.marca) AND (a.marca <>'ZZZ') do
	begin
		cant:= cant+1;
		total:= total + a.precio;
		max(a, maxMarc, maxModelo,maxPrecio);
		leer(a);
	end;
	writeln('el precio promedio de ',marcaAct, ' es: ',promedio(cant, total):2:2);
end;


var
	auto:autoRegistro;
	maxMarc:string;
	maxModelo:string;
	maxPrecio:real;
BEGIN
	leer(auto);
	maxMarc:='a';
	maxModelo:='a';
	maxPrecio:=-1;
	while (auto.marca <> 'ZZZ') do
		marca(auto,maxMarc,maxModelo,maxPrecio);
	writeln('la marca del auto mas caro ',maxMarc,' su modelo ',maxModelo)

END.

