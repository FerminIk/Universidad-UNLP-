{
12. Realizar un programa modularizado que lea una secuencia de caracteres y verifique si cumple con el patrón
A$B#, donde:
- A es una secuencia de sólo letras vocales
- B es una secuencia de sólo caracteres alfabéticos sin letras vocales
- los caracteres $ y # seguro existen
Nota: en caso de no cumplir, informar que parte del patrón no se cumplió.
   
}


program problema12;

function comprobarA(c:char):boolean;
var
	ok:boolean;
begin
	if (c = 'a') or (c = 'A') or (c = 'e') or (c = 'E') or (c = 'i') or (c = 'I') or (c = 'o') or (c = 'O') or (c = 'u') or (c = 'U') then
		ok:=true
	else
		ok:=false;
	comprobarA:=ok;


end;

procedure leerA (var cumple:boolean);
var
	c:char;
begin
	writeln('Escriba la secuencia A');
	readln(c);
	while (cumple) AND (c <> '$') do
		if (comprobarA(c)) then
			readln(c)
		else
			cumple:=false;
			
end;

function comprobarB (c:char):boolean;
var
	ok:boolean;
begin

	if (c = 'a') or (c = 'A') or (c = 'e') or (c = 'E') or (c = 'i') or (c = 'I') or (c = 'o') or (c = 'O') or (c = 'u') or (c = 'U') then
		ok:=false
	else
		ok:=true;
	comprobarB:=ok;

end;


procedure leerB (var cumple:boolean);
var
	c:char;
begin
	writeln('Escriba la secuencia B');
	readln(c);
	while (cumple) AND (c <> '#') do
		if (comprobarB(c)) then
			readln(c)
		else
			cumple:=false;
			
end;



var 
	cumple:boolean;


BEGIN

	cumple:=true;
	leerA (cumple);
	if(cumple) then
	begin
		leerB(cumple);
		if(cumple) then
			writeln('Se introdujo correctamente la secuencia de caracteres')
		else
			writeln('la secuencia B no se cumple');
	end
	else
		writeln('La secuencia A no se cumple');

	
	
END.

