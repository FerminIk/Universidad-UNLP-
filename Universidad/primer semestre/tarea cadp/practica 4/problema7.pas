{
   problema7.pas
   
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


program problema7;

type

	numeritos = array[0..9] of integer;

procedure inicializar (var v:numeritos);
var
	i:integer;
begin
	for i:=0 to 9 do
		v[i]:=0;

end;

procedure ocurrencia(var v:numeritos; n:integer);
var
	m:integer;
begin
	while n<>0 do
	begin
		m:=n mod 10;
		writeln(m);
		n:=n div 10;
		v[m]:=v[m] +1;
		writeln(v[m]);
	end;
end;

procedure ocurrenciaPro(v:numeritos);
var
	i:integer;
begin
	for i:=0 to 9 do
	begin
		if (v[i]>=1) then
			writeln('Numero ',i,' : ',v[i],' veces');
	end;
end;

procedure ocurrenciaMax(v:numeritos);
var
	i:integer;
	max:integer;
begin
	max:=0;
	for i:=0 to 9 do
	begin
		if v[i]>max then
			max:=v[i];
	end;
	writeln('el digito mas leido fue: ',i);
end;

procedure ocurrenciaCero(v:numeritos);
var
	i:integer;
begin
	for i:= 0 to 9 do
	begin
		if (v[i]=0) then
			writeln('los digitos que no tuvieron ocurrencia son: ',i);
	end;
end;

var
	n:integer;
	vector:numeritos;


BEGIN
	writeln('introduzca un numero');
	readln(n);
	inicializar(vector);
	while (n<>-1) do
	begin
		ocurrencia(vector,n);
		writeln('introduzca un numero');
		readln(n);
	end;
	ocurrenciaPro(vector);
	ocurrenciaMax(vector);
	ocurrenciaCero(vector);

	
END.

