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


program problema5;

procedure negativo;
var
	numero:integer;
	max:integer;
begin
	writeln('introduzca un numero');
	readln(numero);
	max:=-1;
	while numero >= 0 do
	begin
		if (numero mod 2 = 0) AND (numero >= max) then
			max:=numero;
		writeln('introduzca un numero');
		readln(numero);
	end;
	writeln(max);

end;
begin
	negativo;
end.
