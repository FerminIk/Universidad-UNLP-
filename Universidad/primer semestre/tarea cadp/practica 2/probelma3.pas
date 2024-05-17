{
   probelma3.pas
   
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

var
	nombre:string;
	nota:integer;
	raspando:integer;
	buenaNota:integer;

BEGIN
	raspando:=0;
	buenaNota:=0;
	repeat
		writeln('ingrese nombre del alumno');
		readln(nombre);
		writeln('ingrese nota del alumno');
		readln(nota);
		if(nota>=8) then
			buenaNota:=buenaNota+1;
		if(nota=7)then
			raspando:=raspando+1;
	until (nombre= 'Zidane Zinedine');
	writeln(raspando);
	writeln(buenaNota);
	
END.


