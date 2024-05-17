{
   problema13.pas
   
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


program problema13;

function cumpleA (c:char):boolean;
var
	ok:boolean;
begin
	if ( c = '$' ) then
		ok:=false
	else
		ok:=true;
	cumpleA:=ok;
end;



procedure leerA (var cumple:boolean; var largo:integer);
var
	c:char;
begin
	writeln('introduzca la secuencia A');
	readln(c);
	while (c <> '%') AND (cumple) do
		if (cumpleA(c)) then
		begin
			readln(c);
			largo:=largo+1;
		end
		else
			cumple:=false;
end;



procedure leerB (var cumple:boolean; var largo:integer);
var
	c:char;
	cant:integer;
	cantA:integer;
	
begin
	cant:=0;
	cantA:=0;
	writeln('introduzca la secuencia B');
	readln(c);
	if (c = '@') then
		cantA:=cantA+1;
	while (c <> '*') do
		begin
			readln(c);
			cant:=cant+1;
			if (c = '@') then
				cantA:=cantA+1;
		end;
		cumple:= (largo = cant) AND (cantA = 3)

end;

var 
	cumple:boolean;
	largo:integer;
BEGIN
	cumple:=true;
	largo:=0;
	leerA(cumple, largo);
	if(cumple) then
	begin
		leerB(cumple, largo);
		if(cumple) then
			writeln('la secuencia fue correcta')
		else
			writeln('La secuencia B es incorrecta');
	end
	else
		writeln('La secuencia A es incorrecta');
	
	
END.

