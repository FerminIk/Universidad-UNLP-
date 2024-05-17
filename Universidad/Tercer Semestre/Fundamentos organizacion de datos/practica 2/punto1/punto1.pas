{
   Práctica 2 - Archivos continuación.pdf
   
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
const
	corte = 'ZZZ';
type
	str20 = string[20];
	
	empleado=record
		cod_emp:integer;
		nombre:str20;
		monto_com:real;
	end;
	
	archEmp = file of empleado;
	
procedure leerArch(var arch:archEmp; var emp:empleado);
begin
	if (not eof(arch)) then
		read(arch,emp)
	else
		emp.nombre:=corte;
end;

var
	archCom:archEmp; archExt:archEmp; emp,empCom:empleado; nombre,nuevo,emp_nombre:str20;
BEGIN
	writeln('introduzca el nombre del archivo: ');
	readln(nombre);
	writeln;
	writeln('¡como desea llamar al nuevo archivo?');
	readln(nuevo);
	writeln;
	assign(archCom,nuevo);
	assign(archExt,nombre);
	rewrite(archCom);
	reset(archExt);
	leerArch(archExt,emp);
	while (emp.nombre <> corte) do begin
		empCom.cod_emp:=emp.cod_emp;
		empCom.nombre:=emp.nombre;
		empCom.monto_com:=0;
		while (emp.nombre <> corte) and (empCom.cod_emp = emp.cod_emp) do begin
			empCom.monto_com:=empCom.monto_com + emp.monto_com;
			leerArch(archExt,emp);
		end;
		write(archCom,empCom);
	end;
	close(archExt);
	close(archCom);
END.

