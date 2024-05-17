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
var
	suma:integer;
	num:integer;
	min:integer;
	max:integer;
BEGIN
	suma:=0;
	max:=-9999;
	min:=9999;
	repeat
		readln(num);
		if (num>max) then
			max:=num;
		if (num<min) then
			min:=num;
		suma:= suma+num;
	until (num=100);
	writeln('numero maximo ',max);
	writeln('numero minimo ',min);
	writeln('suma total de los numeros ',suma);
END.

