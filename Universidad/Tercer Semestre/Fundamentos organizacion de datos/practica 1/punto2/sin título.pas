{
   sin título.pas
   
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


program punto2;
const
	nombre = 'entero.dat';
type 
	enteros = file of integer;
	
var 
	numeros : enteros;
	menores,cant,aux: integer;
	total: real;
BEGIN
	menores := 0; total:= 0; cant:=0;
	assign(numeros,nombre);
	reset(numeros);
	while not EOF(numeros) do begin
		writeln('entro');
		read(numeros,aux);
		if aux <= 1500 then
			menores:= menores+1;
		total:= total+aux;
		cant:=cant+1;
		writeln('el numero leido fue: ',aux);
	end;
	total:=total/cant;
	writeln('los numeros menores de 1500 fueron: ',menores,' y el promedio fue de: ',total:2:2);
	close(numeros);
END.

