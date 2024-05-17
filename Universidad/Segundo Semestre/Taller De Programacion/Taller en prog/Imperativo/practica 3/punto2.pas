{

Welcome to GDB Online.
GDB online is an online compiler and debugger tool for C, C++, Python, Java, PHP, Ruby, Perl,
C#, OCaml, VB, Swift, Pascal, Fortran, Haskell, Objective-C, Assembly, HTML, CSS, JS, SQLite, Prolog.
Code, Compile, Run and Debug online from anywhere in world.

}
program punto2;
const fin = 0;
type
ventas = record
    cod:integer;
    fecha:integer;
    cantVentas:integer;
end;

arbol = ^nodoArbol;
nodoArbol = record
    dato : ventas;
    HI:arbol;
    HD:arbol;
end;

procedure cargarArbol(var a,a2:arbol);
    procedure inicializarArbol(var a:arbol);
    begin
        a:=nil;
    end;
    
    procedure leerVentas(var v:ventas);
    begin
        write('introduce el codigo de  venta: ');
        read(v.cod);
        if (v.cod = 0) then writeln('Fin de la  carga de datos, se introdujo 0')
        else begin
            write('introduce fecha de la venta: ');
            read(v.fecha);
            write('introduce la cantidad vendidas: ');
            read(v.cantVentas);
        end;
    end;
    
    procedure insertarElemento(var a:arbol; v:ventas);
    begin
        if (a = nil) then begin
            new(a);
            a^.dato:=v;
            a^.HD:=nil;
            a^.HI:=nil;
        end
        else begin if (v.cod>=a^.dato.cod) then insertarElemento(a^.HD,v)
            else insertarElemento(a^.HI,v);
        end;
    end;
    
    procedure insertarArbol2(var a:arbol; v:ventas);
    begin
        v.fecha:= -1;
         if (a = nil) then begin
            new(a);
            a^.dato:=v;
            a^.HD:=nil;
            a^.HI:=nil;
        end
        else begin if (v.cod>=a^.dato.cod) then insertarElemento(a^.HD,v)
            else insertarElemento(a^.HI,v);
        end;
    end;
    
var vent:ventas;
begin
    inicializarArbol(a);
    inicializarArbol(a2);
    leerVentas(vent);
    while (vent.cod <> 0 ) do begin
        insertarElemento(a,vent);
        if (vent.cantVentas < 0) then
            insertarArbol2(a2,vent);
        writeln;
        leerVentas(vent);
    end;
end;

procedure buscarVentas(a:arbol; c:integer; var cant:integer);
begin
    if (a <> nil) then begin
        if (a^.dato.cod = c) then cant:=a^.dato.cantVentas
        else begin
            if (a^.dato.cod > c) then buscarVentas(a^.HI,c,cant)
            else buscarVentas(a^.HD,c,cant);
        end;
    end;
end;


var a:arbol; a2:arbol; codigo:integer; cant:integer;
begin
  cant:=0;
  writeln(' ------------------------------------------------- ');
  cargarArbol(a,a2);
  writeln(' ------------------------------------------------- ');
  writeln;
  write('Introduzca el  codigo que desee buscar sus ventas  ');
  read(codigo);
  buscarVentas(a,codigo,cant);
      writeln(' ------------------------------------------------- ');
      writeln;
      if (cant = 0) then writeln('No se encontraron ventas del producto')
      else writeln('las ventas fueron de: ',cant);
      cant:=0;
      writeln(' ------------------------------------------------- ');
      writeln;
      write('Introduzca el  codigo que desee buscar sus ventas (arbol 2)  ');
      read(codigo);
      buscarVentas(a2,codigo,cant);
      if (cant = 0) then writeln('No se encontraron ventas del producto')
      else writeln('las ventas fueron de: ',cant);
end.
