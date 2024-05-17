{
   punto3.pas
   
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


program punto4;
const
	corte = 'fin';
type
	str20 = string[20];
	empleado = record
		nro : integer;
		apellido : str20;
		nombre : str20;
		edad: integer;
		dni : integer;
	end;
	
	empleado_a = file of empleado;
	
procedure leer_empleados(var e:empleado);
begin
	writeln('introduzca el numero de empleado: ');
	readln(e.nro);
	writeln('introduzca el apellido de empleado: ');
	readln(e.apellido);
	writeln('introduzca el nombre de empleado: ');
	readln(e.nombre);
	writeln('introduzca el edad de empleado: ');
	readln(e.edad);
	writeln('introduzca el dni de empleado: ');
	readln(e.dni);
end;
	
procedure punto_a(var emp:empleado_a);
var
	e:empleado;
begin
	leer_empleados(e);
	while e.apellido <> corte do begin
		write(emp,e);
		leer_empleados(e);
	end;
end;


procedure escribir_empleado(e : empleado);
begin
	writeln('nro de empleado: ',e.nro);
	writeln('nombre de empleado: ',e.nombre);
	writeln('apellido de empleado: ',e.apellido);
	writeln('edad de empleado: ',e.edad);
	writeln('dni de empleado: ',e.dni);
	writeln('');
	writeln('');
end;

procedure inciso_a(var emp:empleado_a);
var
	e:empleado; nom_ape:str20; encontro:boolean;
begin
	encontro:=true;
	writeln('escribe el nombre o apellido del empleado que desea buscar: ');
	readln(nom_ape);
	seek(emp,0);
	while (not eof(emp)) and encontro do begin
		read(emp,e);
		if (nom_ape = e.nombre) or (nom_ape = e.apellido) then begin 
			escribir_empleado(e);
			encontro:=false;
		end;
	end;
end;

procedure inciso_b(var emp:empleado_a);
var 
	e : empleado;
begin
	seek(emp,0);
	while (not eof(emp)) do begin
		read(emp,e);
		escribir_empleado(e);
	end;
end;

procedure inciso_c(var emp:empleado_a);
var 
	e : empleado;
begin
	seek(emp,0);
	while (not eof(emp)) do begin
		read(emp,e);
		if (70 <= e.edad) then
			escribir_empleado(e);
	end;
end;


procedure punto_b(var emp:empleado_a);
begin
	writeln('primer apartado');
	writeln('');
	inciso_a(emp);
	writeln('segundo apartado');
	writeln('');
	inciso_b(emp);
	writeln('tercer apartado');
	writeln('');
	inciso_c(emp);
end;

procedure punto_c(var emp:empleado_a);
var
	e:empleado; cumple:boolean; aux:empleado;
begin
	writeln('agregando empleados... ');
	writeln('');
	cumple:=true;
	leer_empleados(e);
	while (e.apellido <> corte) do begin
		seek(emp,0);
		while (not eof(emp)) and cumple do begin
			read(emp,aux);
			if (aux.nro = e.nro) then
				cumple:=false;
		end;
		if cumple then begin
			writeln('se esta agregando el empleado ');
			write(emp,e);
		end
		else
			writeln('ERROR empleado repetido');
		writeln('');
		leer_empleados(e);
	end;
end;

procedure punto_d(var emp:empleado_a);
var
	modi:integer; nombre:str20; aux:empleado; encontro:boolean; pos:integer;
begin
	encontro:=false;
	pos:=0;
	seek(emp,pos);
	writeln('');
	writeln('apartado para cambiar las edades, introduzca un nombre: ');
	readln(nombre);
	while(not eof(emp)) and (not encontro) do begin
		read(emp,aux);
		if aux.nombre = nombre then
			encontro:=true;
	end;
	writeln('');
	if encontro then begin
		writeln('introduzca la nueva edad de: ',nombre);
		readln(modi);
		aux.edad:=modi;
		pos:=filePos(emp);
		seek(emp,pos-1);
		write(emp,aux);
	end
	else
		writeln('No se encontro el nombre, volvera al menu principal: ');
end;

procedure punto_e(var emp:empleado_a);
var
	texto : empleado_a; aux:empleado;
begin
	writeln('');
	writeln('pasando a los empleados al archivo de texto...');
	assign(texto,'todos_empleados.txt');
	rewrite(texto);
	seek(emp,0);
	while (not eof(emp)) do begin
		read(emp,aux);
		write(texto,aux);
	end;
end;


var
	nro_menu:integer; emp:empleado_a; nombre:str20;
BEGIN
	writeln('desea crear o abrir un archivo, seleccione 1 para crear 2 para abrir');
	readln(nro_menu);
	if (nro_menu = 2) then begin
		writeln('introduce el nombre del archivo');
		readln(nombre);
		assign(emp,nombre);
		reset(emp);
	end
	else begin
		writeln('introduce el nombre del archivo');
		readln(nombre);
		assign(emp,nombre);
		rewrite(emp);
	end;
	writeln('');
	writeln('Bienvenido al menu, selecciona una opcion del 1 al 5');
	writeln('Si quiere finalizar el programa precione 0');
	writeln('');
	readln(nro_menu);
	while (nro_menu <> 0) do begin
		if (nro_menu = 1) then begin
			writeln('opcion 1 seleccionada, creando archivo de empleados: ');
			punto_a(emp);
		end
		else begin
			if (nro_menu = 2) then begin
				writeln('opcion 2 seleccionada, entrando al archivo: ');
				punto_b(emp);
			end
			else begin
				if (nro_menu = 3) then begin
					writeln('opcion 3 seleccionada, entrando al archivo: ');
					punto_c(emp);
				end
				else begin
					if (nro_menu = 4) then begin
						writeln('opcion 4 seleccionada, entrando al archivo: ');
						punto_d(emp);
					end
					else begin
						if (nro_menu = 5) then begin
							writeln('opcion 5 seleccionada, entrando al archivo: ');
							punto_e(emp);
						end
						else 
							writeln('haz introducido un numero no valido');
					end;
				end;
			end;
		end;
		writeln('Bienvenido al menu, selecciona una opcion del 1 al 5');
		writeln('Si quiere finalizar el programa precione 0');
		readln(nro_menu);
	end;
	close(emp);
	writeln('Programa terminado.');
END.

