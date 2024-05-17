{
   punto5.pas
   
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


program punto5;
const
	corte = 9999;
	dimF = 50;
type
	str20 = string[20];
	nacimientos = record
		nro:integer;;
		nombre:str20;
		apellido:str20;
		dir:str20;
		matricula:str20;
		madre:str20;
		padre:str20;
	end;
	
	fallecimientos = record
		nro:integer;
		dni:integer;
		nombre:str20;
		apellido:str20;
		matricula:str20;
		fecha:integer;
		hora:real;
		lugar:str20;
	end;
	
	maestro = record;
		nro:integer;
		nombre:str20;
		apellido:str20;
		dir:str20;
		matriculaN:str20;
		madre:str20;
		padre:str20;
		fallecio:boolean;
		matriculaF:str20;
		fecha:integer;
		hora:real;
		lugar:str20;
	end;
	
	archN = file of nacimientos;
	archF = file of fallecimientos;
	archM = file of maestro;
	
	vectorN = array[1..dimF] of nacimientos;
	vectorF = array[1..dimF] of fallecimientos;
	vArchN = array[1..dimF] of archN;
	vArchF = array[1..dimF] of archF;
	
procedure leer(var d:archN; var r:nacimientos);
begin
	if (not eof(d)) then
		read(d,r);
	else
		r.cod:=corte;
end;
	
procedure cargarF(var vFal:vectorF; var f:vArchF);
var
	i:integer;
begin
	for i:= 1 to dimF do
		read(f[i],vFal[i]);
end;

procedure cargarN(var vNac:vectorN; var n:vArchN);
var
	i:integer;
begin
	for i:= 1 to dimF do 
		leer(n[i],vNac[i]);
end;

procedure minimoN(var vNac:vectorN; var n:vArchN; var min:nacimientos);
var
	i:integer;
begin
	min.cod:=corte;
	for i:= 1 to dimF do begin
		if (min.cod >= vNac[i].cod) then begin
			min:= vNac[i].cod;
			leer(n[i],vNac[i]);
		end;
	end;
end;

procedure minimoF(var vFal:vectorF; var f:vArchF; var min:fallecimientos);
var
	i:integer;
begin
	min.cod:=corte;
	for i:= 1 to dimF do begin
		if (min.cod >= vFal[i].cod) then begin
			min:= vFal[i].cod;
			leer(n[i],vFal[i]);
		end;
	end;
end;

procedure cargarAuxN(var aux:maestro; min:nacimientos);
begin
	aux.cod := min.cod;
	aux.nombre := min.nombre;
	aux.apellido := min.apellido;
	aux.dir := min.dir;
	aux.matriculaN := min.matriculaN;
	aux.madre := min.madre;
	aux.padre := min.padre;
	aux.fallecio:=false;
end;

procedure cargarAux(var aux:maestro; minN:nacimientos; minF:fallecimientos);
begin
	aux.cod := minN.cod;
	aux.nombre := minN.nombre;
	aux.apellido := minN.apellido;
	aux.dir := minN.dir;
	aux.matriculaN := minN.matriculaN;
	aux.madre := minN.madre;
	aux.padre := minN.padre;
	aux.fallecio:=true;
	aux.matriculaF:=minF.matriculaF;
	aux.fecha:=minF.fecha;
	aux.hora:=minF.hora;
	aux.lugar:=minF.lugar;
end;

var
	vNac : vectorN; vFal : vectorF;
	m: archM; n:vArchN; f:vArchF;
	minN:nacimientos; minF:fallecimientos; aux:maestro;
	i:integer;
BEGIN
	//di vuelta los archivos y los registros
	assign(maestro,'a');
	for i:= 1 to dimF do begin
		//funcionara esto?
		assign(vNac[i],'b'+i);
		assign(vFal[i],'c'+i);
		reset(vNac[i]);
		reset(vFal[i]);
	end;
	rewrite(maestro);
	cargarF(vFal,f);
	cargarN(vNac,n);
	minimoN(vNac,n,minN);
	//como se crea el archivo, doy por hecho que no tengo  ninguna persona que fallecio y que no tiene acta de nacimiento
	while (minN.cod <> corte) do  begin
		minimoF(vFal,f,minF);
		while (minN.cod <> corte) and (minN.cod < minF.cod) do begin
			cargarAuxN(aux,minN);
			write(maestro,aux);
			minimoN(vNac,n,minN);
		end;
		cargarAux(aux,minN,minF);
		write(maestro,aux);
		minimoN(vNac,n,minN);
	end;
	for i:= 1 to dimF do begin
		close(vNac[i]);
		close(vFal[i]);
	end;
	close(maestro);
END.

