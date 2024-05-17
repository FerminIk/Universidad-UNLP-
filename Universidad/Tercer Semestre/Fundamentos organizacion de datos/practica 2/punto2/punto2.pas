{
   punto2.pas
   
   Copyright 2023 Mar <Mar@DESKTOP-D71C293>
   
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


program punto2;
const valor_alto = 9999;
type
	str20 = string[20];
	alumno = record
		cod : integer;
		apellido : str20;
		nombre:str20;
		cursada:integer;
		conFinal:integer;
	end;
	
	detReg = record
		cod : integer;
		cursada : boolean;
		conFinal : boolean;
	end;
	
	archAlu = file of alumno;
	archDet = file of detReg;
	
procedure seDispone(var arch : archAlu);

procedure cargarDetalle(archD);

procedure leer(var archD:archDet; var datos:detReg);
begin
	if (not eof(archD)) then 
		read(archD,datos);
	else
		datos.cod := valor_alto;
end;

procedure puntoA(var arch:archAlu; var archD:archDet);
var
	datos : detReg; alum:alumnos; cont:integer;
begin
	reset(arch);
	reset(archD);
	cont:=0;
	if (not eof((archD)) then begin
		read(arch,alum);
		leer(archD,datos);
		while (datos.cod <> valor_alto) do  begin
			while (alum.cod < datos.cod) and (datos.cod <> valor_alto) do begin
				read(arch,alum);
				cont:=cont+1;
			end;
			if(datos.cursada) then 
				alum.cursada:=alum.cursada+1;
			else 
				alum.conFinal:=alum.conFinal+1;
			seek(arch,cont);
			write(arch,alum);
		end;
		leer(archD,datos);
	end;
end;

procedure imprimirAlumno(alum:alumno);
begin
	writeln(alum.cod);
	writeln(alum.apellido);
	writeln(alum.nombre);
	writeln(alum.cursada);
	writeln(alum.conFinal);
end;

procedure puntoB(var arch:archAlu);
var
	alum:alumno; texto: TextFile;
begin
	reset(arch);
	rewrite(texto,'alumnosConCursada');
	if (not eof(arch)) then begin
		read(arch,alum);
		while (not eof(arch) do begin
			if (alum.cursada >= 4) then begin
				imprimirAlumno(alum);
				writeln(texto,alum.cod,alum.apellido);
				writeln(texto,alum.cursada,alum.conFinal,alum.nombre);
			end;
			read(arch,alum);
		end;
	end;
	close(texto);
end;
	
var
	nro:integer; arch:archAlu; archD:archDet;
BEGIN
	writeln('Elije una de las siguientes opciones');
	writeln('0.termina programa');
	writeln('1.Actualiza el archivo maestro');
	writeln('2.Lista en un archivo de texto los alumnos que tengan mas de cuatro materias con cursada aprobada');
	readln(nro);
	seDispone(arch);
	cargarDetalle(archD);
	while (nro = 0) do begin
		case nro of
		1: puntoA(arch,archD);
		2: puntoB(arch,archD);
		end;
		writeln('Elije una opcion');
		readln(nro);
	end;
	close(arch);
	close(archD);
END.

