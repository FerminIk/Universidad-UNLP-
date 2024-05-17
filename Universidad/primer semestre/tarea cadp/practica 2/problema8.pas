{
   problema8.pas
   
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


program problema8;

var
	dias:integer;
	max:real;
	ventas:real;
	ventasTotal:real;
	cantidad:integer;
	i:integer;
BEGIN
	dias:= 0;
	max:=0;
	for i:= 1 to 31 do
	begin
		cantidad:=0;
		ventasTotal:=0;
		writeln('ingrese el monto');
		readln(ventas);
		while (ventas <> 0) do
		begin
			cantidad:=cantidad+1;
			ventasTotal:=ventasTotal+ventas;
			writeln('ingrese el monto');
			read(ventas);
		end;
		if (ventasTotal>max) then
		begin
			max:=ventasTotal;
			dias:=i;
		end;
		writeln('la cantidad de ventas fueron de ',cantidad);
		writeln('La cantidad de monto fue de ',ventasTotal:2:1);
	end;
	writeln('el dia donde mas se vendio ',dias);
	
END.

