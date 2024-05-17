{
   problema6.pas
   
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


program untitled;
type
	productos=record
		cod:integer;
		descrip:string;
		stock:integer;
		min_stock:integer;
		precio:real;
	end;
	lista=^nodo;
	nodo=record
		info:productos;
		sig:lista;
	end;
procedure leerProducto(var produ:productos);
begin
	write('Ingrese el codigo del producto: ');readln(produ.cod);
	if produ.cod<>-1 then
	begin
		write('Ingrese la descripcion del producto: ');readln(produ.descrip);
		write('Ingrese el stock actual del producto: ');readln(produ.stock);
		write('Ingrese el stock minimo del producto: ');readln(produ.min_stock);
		write('Ingrese el precio del producto: ');readln(produ.precio);
	end
	else
		writeln('Ingreso el comando de "FIN"')
end;
procedure agregarProducto (var pI:lista;var pU:lista;produ:productos);
var
nuevo:lista;
begin
	new(nuevo);
	nuevo^.info:=produ;
	nuevo^.sig:=nil;
	if pI=nil then
	begin
		pI:=nuevo;
		pU:=nuevo;
	end
	else
	begin
		pU^.sig:=nuevo;
		pU:=nuevo
	end;
end;
procedure lowStock(pI:lista;cant:integer);
var
suma:integer;
promedio:real;
aux:lista;
begin
	suma:=0;
	aux:=pI;
	while aux<>nil do
	begin
		if (aux^.info.stock < aux^.info.min_stock) then
		begin
			suma:= suma + 1;
		end;
		aux:=aux^.sig;
	end;
	promedio:= (suma/cant)*100;
	writeln('El porcentaje de productos con stock actual menor a su stock minimo es: ',promedio:0:3);
end;
procedure codigoPar(pI:lista);
var
dig,par,aux2:integer;
aux:lista;
begin
	par:=0;
	aux:=pI;
	while (aux<>nil) do
	begin
		aux2:=aux^.info.cod;
		while (aux2<>0) do
		begin
			dig:= aux2 mod 10;
			if ((dig MOD 2)=0) then
				par:= par + 1;
			aux2:= aux2 div 10;
		end;
		if (par>=3) then
			writeln('La descripcion del producto con codigo: ',aux^.info.cod,' es: ',aux^.info.descrip);
		par:=0;
		aux:=aux^.sig;
	end;
end;
procedure preciosCuidados (pI:lista);
var
aux:lista;
cod1,cod2:integer;
min1,min2:real;
begin
	min1:=9999;min2:=9999;cod1:=0;cod2:=0;
	aux:=pI;
	while (aux<>nil) do
	begin
		if aux^.info.precio<min2 then
		begin
			if aux^.info.precio<min1 then
			begin
				min2:=min1;
				min1:=aux^.info.precio;
				cod2:=cod1;
				cod1:=aux^.info.cod;
			end
			else
			begin
				min2:=aux^.info.precio;
				cod2:=aux^.info.cod;
			end;
		end;
		aux:=aux^.sig
	end;
	writeln('Los codigos de los dos precios mas economicos son: ',cod1,' y ',cod2);
end;
var
pri,ult:lista;
produ:productos;
cant:integer;
BEGIN
	cant:=0;
	pri:=nil;
	writeln('------------------------------ INGRESANDO PRODUCTOS-----------------------------');
	writeln('---------------- Para salir, ingrese el codigo de producto "-1" ----------------');
	leerProducto(produ);
	while (produ.cod<>-1) do
	begin
		cant:= cant + 1;
		agregarProducto(pri,ult,produ);
		leerProducto(produ)
	end;
	lowStock(pri,cant);
	codigoPar(pri);
	preciosCuidados(pri);
END.

