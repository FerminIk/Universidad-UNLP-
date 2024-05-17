{
   problema3A.pas
   
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


program problema3;

type
	lista = ^nodo;
	nodo = record
		num : integer;
		sig : lista;
end;
procedure imprimirLista (L:lista);
begin
	while L<>nil do begin
		writeln(L^.num);
		L:=L^.sig;
	end;
end;

procedure armarNodo(var L: lista;var ult: lista; v: integer);
var
	aux : lista;
begin
	new(aux);
	aux^.num := v;
	aux^.sig := nil;
	if (L=nil) then 
		L := aux
	else
		ult^.sig:=aux;
	ult:=aux;
end;
var
	pri : lista;
	valor : integer;
	ult: lista;
begin
	pri := nil;
	writeln('Ingrese un numero');
	read(valor);
	while (valor <> 0) do begin
		armarNodo(pri, ult, valor);
		writeln('Ingrese un numero');
		read(valor);
	end;
	imprimirLista(pri);

end.
