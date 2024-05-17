{

Welcome to GDB Online.
GDB online is an online compiler and debugger tool for C, C++, Python, Java, PHP, Ruby, Perl,
C#, OCaml, VB, Swift, Pascal, Fortran, Haskell, Objective-C, Assembly, HTML, CSS, JS, SQLite, Prolog.
Code, Compile, Run and Debug online from anywhere in world.

}
program problema2;
const dimF = 8;
corte = -1;
codMax = 9999;
type
    subGenero = 1..8;
    peliculas = record
        codPeli:integer;
        codGenero:subGenero;
        prom:real
    end;
    
    lista = ^nodo;
    nodo = record
        dato : peliculas;
        sig :lista;
    end;
    
    vector = array [1..dimF] of lista;
    
    
procedure cargarInformacion (var v:vector);
    procedure leerPelicula(var p:peliculas);
    begin
        write('Coloque el codigo de la pelicula: ');
        read(p.codPeli);
        if (p.codPeli<>corte) then begin
            write('Coloque el codigo de la genero: ');
            p.codGenero:= random(8)+1;
            write(p.codGenero);
            writeln;
            write('Coloque el codigo de la prom: ');
            p.prom:= random(10);
            write(p.prom:2:2);
            writeln;
        end;
    end;
    
    procedure cargarVector(var v:vector;  p:peliculas; var l:lista );
        
        procedure insertarAtras (var l:lista ; p:peliculas);
        var nue:lista; aux:lista;
        begin
            new(nue);
            nue^.dato:=p;
            nue^.sig:=nil;
            if (l=nil) then l:=nue
            else begin
                aux:=l;
                while (aux^.sig <> nil) do aux:=aux^.sig;
                aux^.sig:=nue;
            end;
        end;
        
    var aux:subGenero;
    begin
        aux:=p.codGenero;
        insertarAtras(v[aux],p)
    end;
    
var p:peliculas; l:lista; 
begin
    l:=nil;
    leerPelicula(p);
    while (p.codPeli <> corte) do begin
        cargarVector(v,p,l);
        writeln;
        leerPelicula(p);
    end;
end;

procedure  insertarOrdernado(var l:lista; aux:peliculas);
    var ant,act:lista; nue:lista;
    begin
        new(nue);
        nue^.dato:=aux;
        nue^.sig:=nil;
        if (l = nil) then l:=nue
        else begin
            act:=l;
            ant:=l;
            while (act<>nil) and (act^.dato.codPeli <= nue^.dato.codPeli) do begin
                ant:=act;
                act:=act^.sig;
            end;
            ant^.sig:=nue;
            nue^.sig:=act;
        end;
    end;

procedure merge(v:vector; var l:lista);

    procedure calcularMin(var v:vector; var aux:peliculas);
    
        procedure comprobarMin( l:lista ; var min:peliculas);
        var aux:peliculas;
        begin
            aux:=  l^.dato;
            if  (aux.codPeli <= min.codPeli) then min:=aux;
        end;
        
    
        
    var i:integer; 
    begin  
        aux.codPeli := codMax;
        for i:= 1 to dimF do begin 
            if (v[i] <> nil) then
                comprobarMin(v[i],aux);
        end;
        if (aux.codPeli <> codMax) then begin
            v[aux.codGenero]:=v[aux.codGenero]^.sig;
        end;
        
    end;
    
var aux:peliculas;
begin
    l:=nil;
    calcularMin(v,aux);
    while  (aux.codPeli <> codMax) do begin
        insertarOrdernado(l,aux);
        calcularMin(v,aux);
    end;
end;

procedure imprimirLista (l:lista);
var aux:peliculas;
begin
    writeln;
    writeln('-------------------------');
    writeln;
    while (l<>nil) do begin
        aux:=l^.dato;
        writeln ('cod: ',aux.codPeli);
        writeln ('genero: ',aux.codGenero);
        writeln ('prom: ',aux.prom:2:2);
        writeln;
        l:=l^.sig;
    end;
end;

procedure imprimirVector (v:vector);
var aux:peliculas; i:integer; 
begin
    for i:= 1 to dimF do begin
        writeln;
        if (v[i]=nil) then writeln('Celda vacia')
        else
            while (v[i]<>nil) do begin
                aux:=v[i]^.dato;
                writeln ('cod: ',aux.codPeli);
                writeln ('genero: ',aux.codGenero);
                writeln ('prom: ',aux.prom:2:2);
                writeln;
                v[i]:=v[i]^.sig;
            end;
    end;
end;

var v:vector; l:lista;
begin
  randomize;
  cargarInformacion(v);
  imprimirVector(v);
  merge(v,l);
  imprimirLista(l);
end.
