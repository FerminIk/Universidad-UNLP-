{
   problema1a.pas
   
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


program problema1;
const
	dimF = 5;
type
	arreglo=array[1..dimF] of integer;
	
procedure leer(var v:arreglo);
var
	i:integer;
begin
	for i:= 1 to dimF do
	begin
		writeln('escribe un numero');
		readln(v[i]);
	end;
end;

function buscarN(v:arreglo ; n:integer):boolean;
var
	ok:boolean;
	i:integer;
begin
	ok:=false;
	for i:=1 to dimF do
	begin
		if (v[i] = n) then
			ok:=true;
	end;
	buscarN:=ok;
end;

procedure buscar(b:boolean);
begin
	if(b) then
		writeln('el numero estaba en el vector')
	else
		writeln('el numero no estaba en el vector');
end;

var
	v:arreglo;
	n:integer;

BEGIN
	leer(v);
	writeln('introduzca un numero para ver si esa en el vector');
	readln(n);
	buscar(buscarN(v,n));
	
END.

