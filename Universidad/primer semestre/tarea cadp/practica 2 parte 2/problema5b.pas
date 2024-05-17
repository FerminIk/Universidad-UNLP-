{
   problema5b.pas
   
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


Program ejercicio5b;

    function doble(numA:integer; numB:integer): boolean;
    var 
        dobleB:boolean;
    begin
        if (numB = (numA*2)) then
            dobleB:=true
        else
            dobleB:=false;
        doble:=dobleB;
    end;
var
    numero:integer;
    numero2:integer;
    bool:boolean;
    cant:integer;
    cantDobles:integer;
begin
	writeln('escriba el primero numero');
    readln(numero);
    writeln('escriba el segundo numero');
    readln(numero2);
    cant:=0;
    cantDobles:=0;
    repeat
        bool:=doble (numero,numero2);
        cant:= cant +1;
        if bool then
            cantDobles:= cantDobles+1;
		writeln('escriba el primero numero');
        readln(numero);
		writeln('escriba el segundo numero');        
        readln(numero2);
    until (numero = 0) AND (numero2 = 0);
    writeln('la cantidad de numero procesados ', cant ,' y los dobles fueron ',cantDobles );
end.
