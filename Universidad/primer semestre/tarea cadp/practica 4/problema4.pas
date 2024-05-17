{
   problema4.pas
   
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


program problema4;
type

	numeritos = array[1..100] of integer;

procedure leer(var v:numeritos; var dimL:integer);
var
	i:integer;
begin
	for i:=1 to 5 do
	begin
		writeln('introduzca un numero');
		readln(v[i]);
		dimL:= dimL+1;
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

function maximo(v:numeritos;dimL:integer):integer;
var
	i:integer;
	max:integer;
begin
	max:=-1;
	for i:=1to dimL do
	begin
		if (v[i]>max) then
			max:=v[i];
	end;
	maximo:=max;
end;

function minimo(v:numeritos;dimL:integer):integer;
var
	i:integer;
	min:integer;
begin
	min:=9999;
	for i:=1to dimL do
	begin
		if (v[i]<min) then
			min:=v[i];
	end;
	minimo:=min;
end;


var
	vector:numeritos;
	dimL:integer;
	sumaT:integer;
BEGIN
	dimL:=0;
	sumaT:=0;
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
	writeln('el valor maximo introducido es: ');
	writeln(maximo(vector,dimL));
	readln();
	writeln('el valor minimo introducido es: ');
	writeln(minimo(vector,dimL));
	readln();
END.

