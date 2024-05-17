{
   problema7.pas
   
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


program problema7;

var
	nombre:string;
	tiempo:real;
	max1:real;
	max2:real;
	nombreMax1:string;
	nombreMax2:string;
	min1:real;
	min2:real;
	nombreMin1:string;
	nombreMin2:string;
	i:integer;
BEGIN
	max1:=0;
	max2:=0;
	min1:=9999;
	min2:=9999;
	nombreMax1:='a';
	nombreMin1:='a';
	for i:= 1 to 5 do
	begin
		writeln('Escriba el nombre del piloto');
		readln(nombre);
		writeln('escriba el tiempo del piloto');
		readln(tiempo);
		if (tiempo>max1) then
		begin
			max2:=max1;
			nombreMax2:=nombreMax1;
			max1:=tiempo;
			nombreMax1:=nombre;
		end
		else
			if (tiempo>max2) then
			begin
				max2:=tiempo;
				nombreMax2:=nombre;
			end;
			
			
		if (tiempo<min1) then
		begin
			min2:=min1;
			nombreMin2:=nombreMin1;
			min1:=tiempo;
			nombreMin1:=nombre;
		end
		else
			if (tiempo<min2) then
			begin
				min2:=tiempo;
				nombreMin2:=nombre;
			end;
	
	end;
	writeln('Los dos pilotos mas rapidos fueron ',nombreMax1,' y ',nombreMax2);
	writeln('Los dos pilotos mas lentos fueron ',nombreMin1,' y ',nombreMin2);
	
	
END.

