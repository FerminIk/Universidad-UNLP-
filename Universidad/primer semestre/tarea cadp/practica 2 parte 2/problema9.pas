{
   problema9.pas
   
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


program problema9;

procedure descomposicion (num:integer;var suma:integer;var cant:integer);
var
	dig:integer;
begin
	while (num <> 0) do
	begin
		dig:= num mod 10;
		suma:=suma+dig;
		cant:=cant+1;
		num := num DIV 10
	end;
end;
var
	sum:integer;
	cantidad:integer;
	numero:integer;
BEGIN
	sum:=0;
	cantidad:=0;
	readln(numero);
	descomposicion(numero,sum,cantidad);
	writeln('la cantidad de digitos fueron de ',cantidad);
	writeln('La suma de esos digitos fue de ', sum);
	
END.
