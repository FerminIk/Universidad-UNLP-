{
   problema 3.pas
   
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


program problema3;
const
	dimF=4;
type
	numeritos = array[1..dimF] of integer;
	
procedure leer (var v:numeritos; var dimL:integer);
var
	i:integer;
	num:integer;
begin
	for i:= 1 to dimF do
	begin
		writeln('introduzca un numero');
		readln(num);
		v[i]:=num;
		dimL:=dimL+1;
	end;
	

end;



procedure imprimir (v:numeritos; dimL:integer); 
var
	i:integer;
begin
	for i:= 1 to dimL do
		writeln('es ',v[i]);
end;

procedure imprimirAlReves (v:numeritos; dimL:integer);
var
	i:integer;
begin
	for i:= dimL downto 1 do
		writeln('esto ',v[i]);
end;

procedure mitad(v:numeritos; dimL:integer);
var
	i:integer;
begin
	for i:= (dimL div 2) downto 1 do
		writeln('xd ',v[i]);
end;

procedure mitadMasUno(v:numeritos; dimL:integer);
var
	i:integer;
begin
	for i:= (dimL div 2)+1 to dimL do
		writeln('lol ',v[i]);

end;

procedure recorrerDeXaY(v:numeritos; x:integer; y:integer);
var
	i:integer;
begin
	if (x>y) then
	begin
		for i:=x downto y do
			writeln('x to y ', v[i]);
	end
	else
	begin
		for i:=y downto x do
			writeln('y to x ', v[i]);
	end

end;
	
	
var
	vector:numeritos;
	dim:integer;
	x:integer;
	y:integer;
BEGIN
	dim:=0;
	leer(vector,dim);
	imprimir(vector,dim);
	imprimirAlReves(vector,dim);
	mitad(vector,dim);
	mitadMasUno(vector,dim);
	writeln('escriba dos numeros');
	readln(x);
	readln(y);
	recorrerDeXaY(vector,x,y);
END.

