{
   problema5.pas
   
   Copyright 2022 Pc <Pc@DESKTOP-O4SGK1V>
   
   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 2 of the License, or
   (at your option) any later version.
   
   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.
   
   You should have received a copy of the GNU General Public License
   along with this program; if not, write to the Free Software
   Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
   MA 02110-1301, USA.
   
   
}


program problema5;
type

	numeritos = array[1..100] of integer;

procedure leer(var v:numeritos; var dimL:integer);
var
	i:integer;
begin
	i:=1;
	writeln('introduzca un numero');
	readln(v[i]);
	while (v[i]<>0) and (dimL<>100) do
	begin
		dimL:= dimL+1;
		i:=i+1;
		writeln('introduzca un numero');
		readln(v[i]);
	end;
end;

procedure puntoA(v:numeritos; dimL:integer);
var
	x:integer;
begin
	writeln('introduzca un numero del 1 al 100');
	readln(x);
	if (x <= dimL) then
		writeln('el numero es: ',v[x])
	else
		writeln('-1');
end;

procedure intercambio(var v:numeritos; dimL:integer);
var
	x,y:integer;
	aux:integer;
begin
	writeln('introuzca el primer valor');
	readln(x);
	writeln('introduzca el segundo valor');
	readln(y);
	if (x<dimL) AND (y<dimL) AND (x>0) AND (y>0) then
	begin
		aux:=v[x];
		v[x]:=v[y];
		v[y]:=aux;
	end;
end;

procedure sumaTotal(v:numeritos; dimL:integer;var sumaT:integer);
var
	i:integer;
begin
	for i:= 1 to dimL do
		sumaT:=sumaT+v[i];
end;

function promedio(dimL:integer; sumaT:integer):real;
begin
	promedio:=sumaT/dimL;

end;

procedure maximo(v:numeritos;dimL:integer;var max:integer; var posMax:integer);
var
	i:integer;
begin
	for i:=1to dimL do
	begin
		if (v[i]>max) then
		begin
			max:=v[i];
			posMax:=i;
		end;
	end;
end;

procedure minimo(v:numeritos; dimL:integer;var min:integer; var posMin:integer);
var
	i:integer;
begin
	for i:=1to dimL do
	begin
		if (v[i]<min) then
		begin
			min:=v[i];
			posMin:=i;
		end;
	end;
end;

procedure problema5 (var v:numeritos; posMax:integer; minPos:integer);
var
 aux:integer;
begin
	aux:=v[posMax];
	v[posMax]:=v[minPos];
	v[minPos]:=aux;
end;

var
	vector:numeritos;
	dimL:integer;
	sumaT:integer;
	max:integer;
	min:integer;
	posMax:integer;
	posMin:integer;
BEGIN
	dimL:=0;
	sumaT:=0;
	max:=0;
	min:=9999;
	leer(vector,dimL);
	puntoA(vector,dimL);
	intercambio(vector,dimL);
	writeln(vector[1],vector[2],vector[3],vector[4],vector[5]);
	sumaTotal(vector,dimL,sumaT);
	writeln('La suma total de valores es de: ');
	writeln(sumaT);
	readln();
	writeln('el promedio de todos los numeros es de: ');
	writeln(promedio(dimL,sumaT):2:2);
	readln();
	maximo(vector,dimL,max,posMax);
	writeln('el valor maximo introducido es: ');
	writeln(max);
	readln();
	minimo(vector,dimL,min,posMin);
	writeln('el valor minimo introducido es: ');
	writeln(min);
	readln();
	problema5(vector,posMax,posMin);
	writeln('El elemento máximo ',max,' que se encontraba en la posición', posMax,' fue intercambiado con el elemento mínimo ',min, ' que se encontraba en la posición ',posMin);
	readln();
END.
