{
   problema5.pas
   
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


program problema5;

type
	a = record
		codigo:integer;
		descripcion:string[20];
		stockAct:integer;
		stockMin:integer;
		precio:real;
	end;
	
	lista=^nodo;
	nodo=record
		producto:a;
		sig:lista;
	end;
	

procedure cargar(var L:lista);
begin
	L:=nil;
end;

procedure leerLista(var prod:a);
begin
	writeln('introduzca el codigo');
	readln(prod.codigo);
	if (prod.codigo <> -1) then begin
		writeln('introduzca una descripcion');
		readln(prod.descripcion);
		writeln('introduzca el stock actual');
		readln(prod.stockAct);
		writeln('introduzca el stock minimo');
		readln(prod.stockMin);
		writeln('introduzca el precio');
		readln(prod.precio);
	end
	else
		writeln('se termino la carga de datos');
end;

procedure cargarDatos(var L:lista; var prod:a;var total:integer);
var
	aux:lista;
begin
	leerLista(prod);
	while prod.codigo <> -1 do begin
		new(aux);
		aux^.producto:=prod;
		aux^.sig:=nil;
		if (L=nil) then
			L:=aux
		else
		begin
			aux^.sig:=L;
			L:=aux;
		end;
		total:=total+1;
		leerLista(prod);
	end;

end;

procedure imprimirLista(l:lista);
begin
	while (l<>nil) do begin
		writeln(l^.producto.codigo);
		writeln(l^.producto.descripcion);
		writeln(l^.producto.stockAct);
		writeln(l^.producto.stockMin);
		writeln(l^.producto.precio:2:2);
		l:=l^.sig;
	end;
end;

function porcentajeDeStockMin(pri:lista;total:integer):real;
var
	suma:integer;
	aux:lista;
begin
	suma:=0;
	aux:=pri;
	while (aux<>nil) do 
	begin
		if aux^.producto.stockAct < aux^.producto.stockMin then
		begin
			suma:=suma+1;
		end;
		aux:=aux^.sig;
	end;
	porcentajeDeStockMin:=(suma/total)*100;
end;

procedure tresPar(L:lista);
var
	aux:lista;
	num:integer;
	cant:integer;
	total:integer;
begin
	total:=0;
	aux:=L;
	num:= aux^.producto.codigo mod 10;
	while (aux<>nil) do begin
		cant:=0;
		while (aux^.producto.codigo <> 0) do begin
			if (num mod 2 = 0) then 
				cant:=cant+1;
			aux^.producto.codigo:=aux^.producto.codigo div 10;
			num:= aux^.producto.codigo mod 10;
		end;
		if (cant >=3) then
			total:=total+1;
		aux:=aux^.sig;
	end;
	writeln('La cantidad de codigos con minimo 3 numeros pares fueron: ',total);
end;

procedure barato(L:lista);
var
	aux:lista;
	cod1:integer;
	cod2:integer;
	min1,min2:real;
begin
	min1:=9999;
	min2:=9999;
	cod1:=0;
	cod2:=0;
	aux:=L;
	writeln('hola');
	while (aux<>nil) do
	begin
		if aux^.producto.precio<min2 then
		begin
			writeln('hola');
			if aux^.producto.precio<min1 then
			begin
				writeln('hola');
				min2:=min1;
				min1:=aux^.producto.precio;
				cod2:=cod1;
				cod1:=aux^.producto.codigo;
			end
			else
			begin
				min2:=aux^.producto.precio;
				cod2:=aux^.producto.codigo;
			end;
		end;
		aux:=aux^.sig
	end;
	writeln('Los codigos de los dos precios mas economicos son: ',cod1,' y ',cod2);
end;

var
	pri:lista;
	prod:a;
	total:integer;
BEGIN
	total:=0;
	cargar(pri);
	cargarDatos(pri,prod,total);
	imprimirLista(pri);
	writeln('este es el porcentaje de productos por debajo de su stock minimo ',porcentajeDeStockMin(pri,total):2:2);
	tresPar(pri);
	barato(pri);
END.

