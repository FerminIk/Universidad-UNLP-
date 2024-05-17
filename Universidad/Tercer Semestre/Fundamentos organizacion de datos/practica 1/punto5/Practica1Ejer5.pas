program Practica1Ejer5;


Type
	cadenaStr = String[120];
	
	celular = record
		codigo: integer;
		precio: real;
		marca: cadenaStr;
		descrip: cadenaStr;
		stock: integer;
		stockMin: integer;
		nombre: cadenaStr;
	end;
	
	archivo = file of celular;
	
	procedure Listar (var arch_logico: archivo);
	var	c:celular;
	begin
		reset(arch_logico);
		writeln('Celulares');
		writeln;
		while (not eof(arch_logico)) do begin
			read(arch_logico, c);
			with c do begin
				writeln('Codigo: ', codigo,' Precio: ', precio:2:2,' Marca: ', marca);
				writeln('Stock: ', stock,' Stock min: ', stockMin);
				writeln('Descipcion: ', descrip);
				writeln('Nombre: ', nombre);
			end;
			writeln;
		end;
		close(arch_logico);
	end;
	
	procedure listarCelulares(var arch : archivo);
	var
		reg : celular;
	begin
		reset(arch);
		while(not eof(arch)) do begin
		read(arch,reg);
		if(reg.stock < reg.stockMin) then
			writeln(reg.codigo,'',reg.marca,' ',reg.nombre,' ',reg.precio:2:2,' ',reg.stock, ' ', reg.descrip);
		end;
		close(arch);
	end;

	procedure cargarArchivo(var arch : archivo);
	var
		reg : celular;
		txt : Text;
	begin
		assign(txt,'celulares.txt');
		reset(txt);
		rewrite(arch);
		while(not eof(txt)) do begin
			readln(txt,reg.codigo,reg.precio,reg.marca);
			readln(txt,reg.stock,reg.stockMin,reg.descrip);
			readln(txt,reg.nombre);
			write(arch,reg);
		end;
		close(txt);
		close(arch);
		writeln('Archivo cargado.');
		Listar(arch);
	end;

	procedure contieneCadena(var arch : archivo);
	var
	  reg : celular;
	  cadena : String[120];
	begin
		write('Ingrese la cadena de caracteres ');
		readln(cadena);
		writeln;
		reset(arch);
		while(not eof(arch)) do begin
			read(arch,reg);
			{if(pos(cadena,reg.descrip) > 0) then}
			if(cadena=reg.descrip) then
				writeln('Celular que coincide con la descripcion ingresada: ');
				writeln;
				writeln(reg.codigo,'',reg.marca,' ',reg.nombre,' ',reg.precio:2:2,' ',reg.stock);
				writeln;
		end;
		close(arch);
	end;

	procedure crearArchivoTxt(var arch : archivo);
	var
	  reg : celular;
	  txt : Text;
	begin
		assign(txt,'celularesnuevo.txt');
		reset(arch);
		rewrite(txt);
		writeln;
		writeln('Creando archivo nuevo');
		writeln;
		while(not eof(arch)) do begin
			read(arch,reg);
			writeln(txt,reg.codigo,' ',reg.precio:2:2,' ',reg.marca);
			writeln(txt,reg.stock,' ',reg.stockMin,' ',reg.descrip);
			writeln(txt,reg.nombre);
		end;
		writeln('Finalizado archivo nuevo');
		close(arch);
		close(txt);
	end;
	
var
	arch : archivo;
	op : char;
	n : String[50];
begin

	writeln('Ingrese nombre del archivo ');
	readln(n);
	assign(arch,n);
	repeat
		writeln;
		writeln('Elija una opcion ');
		writeln;
		writeln('1 : cargar el archivo');
		writeln('2 : celulares con stock menor al minimo');
		writeln('3 : celulares que su descripcion contiene la cadena ingresada');
		writeln('4 : crear archivo txt para futuras cargas');
		readln(op);

		case op of
		'1' : cargarArchivo(arch);
		'2' : listarCelulares(arch);
		'3' : contieneCadena(arch);
		'4' : crearArchivoTxt(arch);
		end;
	until(op='5');
	readln;
END.

