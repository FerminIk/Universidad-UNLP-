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


var 
	leer:integer;
	min1:integer;
	min2:integer;
	i:integer;

BEGIN
	min1:=9998;
	min2:=9999;
	for i:= 1 to 5 do
	begin
		readln(leer);
		if (min1>=leer) then
			min1:=leer
		else
		begin
			if (min1<leer) AND(min2>=leer)then
			min2:=leer
		end;
	end;
	writeln(min1,' ',min2);
	
END.

