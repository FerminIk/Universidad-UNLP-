{
   problema6.pas
   
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


program untitled;

var
	codigo:integer;
	precio:real;
	min1:real;
	min2:real;
	codigoMin1:integer;
	codigoMin2:integer;
	mayor:integer;
	i:integer;
BEGIN
	min1:=9999;
	min2:=9999;
	codigoMin1:=0;
	codigoMin2:=0;
	mayor:=0;
	for i:= 1 to 5 do
	begin
		writeln('introduzca el codigo del producto');
		readln(codigo);
		writeln('introduzca el precio del producto');
		readln(precio);
		if (min1>precio) then
		begin
			min2:=min1;
			min1:=precio;
			codigoMin2:=codigoMin1;
			codigoMin1:=codigo;		
		end;
		if (min2>precio) AND (min1<precio) then
		begin
			min2:=precio;
			codigoMin2:=codigo;
		end;

			{Se que utilizar el else no es lo mas optimo pero lo uso por que se que se leen 200 productos}
		if(precio>16) AND (codigo MOD 2 = 0) then
			mayor:=mayor+1;
	end;
	writeln('La cantidad de productor mayores de 16 pesos con codigo par fueron ', mayor);
	writeln('Los dos productos mas bataros fueron ', codigoMin1,' Y ', codigoMin2);
	
END.

