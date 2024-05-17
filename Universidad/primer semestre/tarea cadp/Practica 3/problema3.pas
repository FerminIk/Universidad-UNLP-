{
   problema3.pas
   
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

type
	escuelas = record
		cue:integer;
		nombre:string[20];
		cantAlum:integer;
		cantDoc:integer;
		localidad:string[20];
	end;
	
procedure leer(var v:escuelas);
begin
	with v do
	begin
		writeln('Cue');
		readln(cue);
		writeln('Nombre');
		readln(nombre);
		writeln('Cantidad de Alumnos');
		readln(cantAlum);
		writeln('Cantidad de Docentes');
		readln(cantDoc);
		writeln('Localidad');
		readln(localidad);
	end;
end;

function relacion(r:escuelas):real;
begin
	relacion:=r.cantAlum DIV r.cantDoc;
end;

procedure unesco(var cant: integer ;cociente:real);
begin
	if (cociente <= 23435) then
		cant:=cant+1;
end;

procedure cueYNombre(r:escuelas; cociente:real;var max:real;var max2:real;var nomMax:string;var nomMax2:string;var cueMax:integer;var cueMax2:integer);
begin
	if (cociente<max) AND (cociente<max2) then
	begin
		max2:=max;
		nomMax2:=nomMax;
		cueMax2:=cueMax;
		max:=cociente;
		nomMax:=r.nombre;
		cueMax:=r.cue;
	end
	else
	begin
		if (cociente>max) AND (cociente<max2) then
		begin
			max:=cociente;
			nomMax2:=r.nombre;
			cueMax:=r.cue;
			
		end;
	
	end;

end;

var 
	i:integer;
	cociente:real;
	registro:escuelas;
	cantU:integer;
	cueMax:integer;
	nomMax:string[20];
	cueMax2:integer;
	nomMax2:string[20];
	max:real;
	max2:real;
BEGIN
	cantU:=0;
	max:=9999;
	max2:=9999;
	nomMax:='a';
	nomMax2:='a';
	cueMax:=0;
	cueMax2:=0;
	for i:= 1 to 2 do
	begin
		leer(registro);
		cociente:=relacion(registro);
		unesco(cantU,cociente);
		cueYNombre(registro,cociente,max,max2,nomMax,nomMax2,cueMax,cueMax2);
	end;
	writeln('La cantidad de escuelas que cumplen con los requisitos de la unesco fueron de: ', cantU);
	writeln('las dos escuelas con mejores relaciones fueron: ',nomMax,' ',cueMax,' y ',nomMax, ' ', cueMax);
END.

