{
   punto3.pas
   
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


program punto3;
const
	dimF = 30;
	corte = 9999;
type
	str20 = string[20];
	producto = record
		cod : integer;
		nombre : str20;
		desc : str20;
		stockDis : integer;
		stockMin : integer;
		precio : real;
	end;

	proDet = record
		cod:integer;
		cantVendida:integer;
	end;
	
	archM = file of producto;
	archD = file of proDet;
	
	vArch : array[1..dimF] of archD;
	
procedure inicializarV(var v:vArch);
var
	i:integer;
begin
	for i:= 0 to 29 do
		reset(v[i]);
end;

procedure leer(var archM:producto; var reg:regDet)
begin
	if (not eof(archM)) then 
		read(archM,reg);
	else
		reg.cod := corte;
end;
	
procedure minimo(var v:vArch; var regDet:proDet);
var
	i:integer; aux :regDet;
begin
	regDet.cod:=999;
	for i:= 0 to 29 do begin
		leer(v[i],aux);
		if (regDet.cod <= aux.cod) then 
			regDet:=aux
		else
			seek(v[i],v[i].filePos-1);
	end;
end;
	
var
	v:vArch; maestro:archM; regDet:proDet; reg:producto ; codAct:integer;
BEGIN
	reset(maestro);
	inicializarV(v);
	total:=0;
	minimo(v,regDet);
	codAct := regDet.cod;
	while (regDet.cod <> corte) do  begin
		read(maestro,reg);
		if (reg.cod = codAct) then begin
			while (regDet.cod <> corte) and (regDet.cod = codAct) do begin
				total := total + regDet.cantVendida;
				minimo(v,regDet);
			end;
			codAct := regDet.cod;
			reg.stockDis := reg.stockDis - total;
			seek(maestro,maestro.filePos-1);
			write(maestro,reg);
		end;
	end;
	close(v);
	close(maestro);
END.

