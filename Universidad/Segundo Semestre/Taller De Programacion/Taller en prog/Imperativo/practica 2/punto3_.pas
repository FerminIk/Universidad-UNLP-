program punto3;
const
    numRandom = 5; // uno mas que el maximo
    dimF = 5;
type
    vector = array [1..dimF] of integer;
    
procedure inicializarVector(var v:vector);
var
    i:integer;
begin
    for i:= 1  to dimF do 
        v[i]:=0;
end;    

procedure vectorRandom(var v:vector; var dimL:integer);
var n:integer;
begin
    n:=0;
    randomize;
    n:=random(6)+1;
    write ('numero: ', n ,' creado por la maquina');
    writeln;
    if (n=0)then writeln('introdujo 0, fin de cargar vector')
    else begin
        if (dimL < dimF) then begin
            dimL:= dimL+1;
            v[dimL]:=n;
            vectorRandom(v,dimL);
        end;
    end;
end;

procedure imprimirvector(v:vector; dimL:integer);
var i:integer;
begin
    for i:= 1 to dimL do 
        writeln('numero: ',  v[i]);
end;

var 
v :vector; 
dimL:integer;
begin
    dimL:=0;
    inicializarVector(v);
    vectorRandom(v,dimL);
    if (dimL = 0) then writeln('Vector vacio, fin del programa')
    else begin
        imprimirvector(v,dimL);
    end;
end.
