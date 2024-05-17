{
   punto4.pas
   
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


program punto4;
const
	corte=9999;
	dimF=5;
type
	str20 = string[20];
	regM = record
		cod:integer;
		fecha:str20;
		total:real;
	end;
	regD = record
		cod:integer;
		fecha:str20;
		sesion:real;
	end;
	
	archM = file of regM;
	archD = file of regD;
	
	vector = array [1..dimF] of archD;
	vReg = array [1..dimF] of regD;

procedure leer(var det:archD;var r:regD);
begin
	if (not eof(det)) then
		write(det,r);
	else
		r.cod:=corte;
end;
	
procedure cargarElem(var detalles:vector;var vec:vReg);
var
	i:integer;
begin
	for i:= 1 to dimF do 
		leer(detalles[i],vec[i]);
end;
procedure minimo(var detalles:vector;var vec:vReg;var min:regD);
begin
	for i:= 1 to dimF do begin
		if (min.cod >= vec[i].cod) then begin
			min:=vec[i];
			leer(detalles[i],vec[i]);
		end;
	end;
end;
var
	maestro : archM; detalles:vector; i:integer; vec:vReg; min:regD;
	aux:regD;
BEGIN
	min.cod:=corte;
	rewrite(arch,'/var/log');
	for i:= 1 to 5 do 
		reset(detalles[i]);
	cargarElem(detalles,vec);
	minimo(detalles,vec,min);
	while (min.cod <> corte) do begin
		aux.fecha:=min.fecha;
		while (min <> corte) and (fecha = min.fecha) do begin
			aux.cod:=min.cod;
			aux.sesion:=0;
			while (min <> corte) and (fecha = min.fecha) and (cod = min.cod) do begin
				aux.sesion:=total+min.sesion;
				minimo(detalles,vec,min);
			end;
			write(maestro,aux);
		end;
	end;
	close(maestro);
	for i:=1 to 5 do
		close(detalles[i]);
END.

