{
   punto1.pas
   
   Copyright 2023 renata <renata@DESKTOP-VGI95ON>
   
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


program punto1;

type
	archivo = file of integer;
	
var
	archivo_logico : archivo;
	aux : integer;
BEGIN
	assign(archivo_logico,'entero.dat');
	rewrite(archivo_logico);
	writeln('escriba un numero: ');
	writeln('recuerde que con 30000 se termina la ejecucion');
	readln(aux);
	while (aux <> 30000) do begin
		write(archivo_logico,aux);
		writeln('recuerde que con 30000 se termina la ejecucion');
		readln(aux);
	end;
	writeln(fileSize(archivo_logico));
	close(archivo_logico);
END.

