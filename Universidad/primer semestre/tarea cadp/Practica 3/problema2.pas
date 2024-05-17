{
   problema2.pas
   
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


program problema2;
type
	fecha = record
		dia:integer;
		mes:integer;
		anio:integer;
	end;

procedure leer(var r:fecha);
begin
	with r do
	begin
		writeln('introduzca el dia');
		readln(dia);
		writeln('introduzca el mes (en numero)');
		readln(mes);
		writeln('introduzca el año');
		readln(anio);
	end;

end;

procedure primeros10 (r:fecha;var cant:integer);
begin
	if(r.dia>=1) And (r.dia<=10)then
		cant:=cant+1;
end;

procedure casamientoEneFebMar (r:fecha;var cant:integer);
begin
	if (r.mes >=3) AND (r.mes<=1) then
		cant:=cant+1;

end;
		
var
	registro:fecha;
	cant10:integer;
	cant:integer;
	
BEGIN
	cant10:=0;
	cant:=0;
	leer(registro);
	while (registro.anio=2019) do
	begin
		primeros10(registro,cant10);
		casamientoEneFebMar(registro,cant);
		leer(registro);
	end;
	writeln('La cantidad de casamientos en los primeros 10 dias fueron de: ',cant10);
	writeln('La cantidad de casamientos en enero, febrero y marzo fueron de: ',cant);
	
	
END.

