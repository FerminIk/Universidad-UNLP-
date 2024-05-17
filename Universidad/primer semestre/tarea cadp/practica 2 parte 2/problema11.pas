{
   problema11.pas
   
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


program problema11;

procedure apellidoChico(apellido:string; num:integer; var apellidoMin1:string; var apellidoMin2:string; min1:integer; min2:integer);
begin
{¿Por que funciona mal este modulo?}
	if(num < min1)then
	begin
		apellidoMin2:=apellidoMin1;
		apellidoMin1:=apellido;
		min2:=min1;
		min1:=num;
	end;
	if(num < min2) AND (num > min1)then
	begin
		apellidoMin2:=apellido;
		min2:=num;
	end;

end;


procedure nombreGrande(nombre:string; num:integer;var nombreMax1:string; var nombreMax2:string; var max1:integer; var max2:integer);

begin
	if (num > max1)then
	begin
		nombreMax2:=nombreMax1;
		nombreMax1:=nombre;
		max2:=max1;
		max1:=num;
	end;
	if (num > max2) AND (num < max1) then
	begin
		nombreMax2:=nombre;
		max2:=num;
	end;

end;

procedure par(num:integer;var Par:integer);
begin

	if(num MOD 2 = 0) then
		Par:= Par + 1;
end;



var
{datos base}
	numeroDeIns:integer;
	apellido:string;
	nombre:string;
{porcentaje}
	cant:integer;
	pares:integer;
	porcentajePar:real;
{min}
	apellidoMin:string;
	apellidoMin2:string;
	min:integer;
	min2:integer;

{max}
	nombreMax:string;
	nombreMax2:string;
	max:integer;
	max2:integer;


BEGIN
{calcular porcentaje: (cantidad de numeros pares*100/la cantidad total)}

{porcentaje}
	cant:=0;
	pares:=0;
	porcentajePar:=0;

{min}
	apellidoMin:='-';
	apellidoMin2:='-';
	min:=9999;
	min2:=9999;
	
{max}
	nombreMax:='-';
	nombreMax2:='-';
	max:=-1;
	max2:=-2;

	
	repeat
		writeln('introduzca el numero de inscripcion del alumno');
		readln(numeroDeIns);
		writeln('nombre del alumno');
		readln(nombre);
		writeln('apellido del alumno');
		readln(apellido);
{primer procedure (anda mal)}
		apellidoChico(apellido,numeroDeIns,apellidoMin,apellidoMin2,min,min2);
{segundo procedure}
		nombreGrande(nombre,numeroDeIns,nombreMax,nombreMax2,max,max2);
{tercer procedure}
		Par(numeroDeIns,pares);

		cant:= cant + 1;
	until(numeroDeIns = 1200);
	
	porcentajePar:= (pares*100)/cant;
	writeln('el apellido de los alumnos con el codigo mas bajo fueron ',apellidoMin, ' y ', apellidoMin2);
	writeln('el nombre de los alumnos con el codigo mas alto fueron ',nombreMax, ' y ', nombreMax2);
	writeln('el porcentaje de los alumnos inscripto con numeros pares es de: ',porcentajePar:2:2,'%');
	
	
	
END.

