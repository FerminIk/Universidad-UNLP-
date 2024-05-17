{
   problema2.pas
   
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


program Vectores;
const
	cant_datos = 3;
type
	vdatos = array[1..cant_datos] of real;
procedure cargarVector(var v:vdatos; var dimL : integer);
var
	num:real;
begin
	writeln('introduzca un numero');
	readln(num);
	while (num<>0) AND NOT(dimL=cant_datos) do
	begin
		dimL:=dimL+1;
		v[dimL]:=num;
		writeln('introduzca un numero');
		readln(num);
	end;
	writeln('se leyo')
end;


procedure modificarVectorySumar(var v:vdatos; dimL : integer; n: real; var suma: real);
var
	i:integer;
begin
	for i:= 1 to dimL do
	begin
		v[i]:=v[i] + n;
		suma:= suma + v[i];
		writeln(v[i])
	end;	
end;
	
	
	
{ programa principal }
var
	datos : vdatos;
	dim : integer;
	num, suma : real;
begin
	dim := 1;
	suma:= 0;
	cargarVector(datos,dim);
	writeln('Ingrese un valor a sumar');
	readln(num);
	modificarVectorySumar(datos,dim,num,suma);
	writeln('La suma de los valores es: ', suma:5:5);
	writeln('Se procesaron: ',dim, ' números')
end.
