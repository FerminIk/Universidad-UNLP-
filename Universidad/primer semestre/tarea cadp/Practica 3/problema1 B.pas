{
   problema1 B.pas
   
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


program problema1A;
type
	str20 = string[20];
	alumno = record
		codigo : integer;
		nombre : string;
		promedio : real;
end;

procedure leer(var alu : alumno);
begin
	writeln('Ingrese el codigo del alumno');
	read(alu.codigo);
	if (alu.codigo <> 0) then 
	begin
		writeln('promedio');
		readln(alu.promedio);
		writeln('nombre');
		readln(alu.nombre);
	end;
end;

function comparacion(max:real ; promedio:real):boolean;
var
	ok:boolean;
begin
	if (max<promedio) then
		ok:=true
	else
		ok:=false;
	comparacion:=ok;
	
end;

procedure mayor(alu:alumno ; var max:real; var maxNombre:string);
var
ok:boolean;
begin

	ok:=comparacion(max,alu.promedio);
	if (ok) then
	begin
		maxNombre:= alu.nombre;
		max:=alu.promedio;
	end;
end;
	

var
	alu : alumno;
	cant: integer;
	max:real;
	maxNombre:string;
begin
	max:=-1;
	maxNombre:='a';
	leer(alu);
	cant:=0;
	while (alu.codigo <> 0) do
	begin
		mayor(alu, max, maxNombre);
		cant:= cant+1;
		leer(alu);
	end;
	writeln('la cantidad de alumnos leidos fueron ', cant);
	writeln('el promedio mas alto fue de ',maxNombre);
end.
