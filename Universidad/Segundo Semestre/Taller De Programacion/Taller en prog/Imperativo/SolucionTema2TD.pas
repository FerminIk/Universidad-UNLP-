{ Una empresa de excursiones necesita el procesamiento de las ventas realizadas en el año 2019. De cada venta se conoce el número del cliente, día, mes y 
código de excursión (entre 100 y 200).

a) Implementar un módulo que lea ventas, genere y retorne una estructura adecuada para almacenar las ventas agrupadas por mes. De cada venta sólo deben 
almacenarse número cliente y código de excursión, y debe quedar ordenado por número del cliente. La lectura finaliza con el número de cliente 0. 
Se sugiere utilizar el módulo leerRegistro ().
b) Implementar un módulo que reciba la estructura generada en a), genere y retorne una nueva estructura, ordenada por número de cliente, con cada una de
 las ventas almacenadas.
c) Implemente un módulo recursivo que reciba la estructura generada en b), un número de cliente y un código de excursión, y retorne verdadero si dicho 
cliente realizó al menos 3 excursiones incluyendo al código de excursión recibido, o falso en caso contrario.

NOTA: Implementar el programa principal, que invoque a los incisos a, b y c.

 }


program Tema2TurnoD;
const dimF = 12;
      valorGrande = 9999;
type
   rangoExcursiones = 100..200;
   rangoDias = 1..31; rangoMeses = 1..dimF;
   registro = record
                 numCli: integer;
                 dia: rangoDias;
                 mes: rangoMeses;
                 codExcursion: rangoExcursiones;
         	  end;
   datoAGuardar = record
                    numCli: integer;
                    codExcursion: rangoExcursiones;
                  end;
   lista= ^nodo;
   nodo= record
           dato:datoAGuardar;
           sig:lista;
         end;
   vector = array [1..dimF] of lista;


       
procedure CargarInformacion (var v: vector);
{ Implementar un módulo que lea pedidos, genere y retorne una estructura adecuada para almacenar los pedidos agrupados por zona de envío. De cada pedido 
sólo deben almacenarse código del cliente, día y número de combo pedido, y debe quedar ordenado por código del cliente. La lectura finaliza con el código de 
cliente 0. Se sugiere utilizar el módulo leerRegistro ().}
    
    procedure InicializarVector (var v: vector);
    var i: integer;
    begin
      for i:= 1 to dimF do
         v[i]:= nil;
    end;

    procedure leerRegistro (var r : registro);
    begin
      {r.numCli:= Random(500);}
      write ('Ingrese numero: ');
      read (r.numCli);
      if (r.numCli  <> 0)
      then begin
           	 r.dia:= 1 + Random (31);
         	 r.mes := 1 + Random(12); 
             r.codExcursion := 100 + Random(101);
             writeln ('        Dia: ', r.dia, ' Mes: ' , r.mes, ' Codigo excursion: ', r.codExcursion);
           end; 
    end;

    procedure ArmarDatoAGuardar (venta: registro; var elem: datoAGuardar);
    begin
      elem.numCli:= venta.numCli;
      elem.codExcursion:= venta.codExcursion;
    end;

    procedure InsertarElementoEnLista (var l: Lista; elem: datoAGuardar);
    var ant, nuevo, act: lista;
    begin
      new (nuevo);
      nuevo^.dato := elem;
      act := l;
      {Recorro mientras no se termine la lista y no encuentro la posición correcta}
      while (act <> NIL) and (act^.dato.numCli < elem.numCli) do begin
        ant := act;
        act := act^.sig ;
      end;
      if (act = l)  then l:= nuevo   {el dato va al principio}
                    else ant^.sig  := nuevo; {va entre otros dos o al final}
      nuevo^.sig := act ;
    end;
    
var  venta: registro;
     elem: datoAGuardar;
begin
  writeln ('---------------------- Almacenar ventas por mes ----------------------');
  writeln;
  InicializarVector (v);
  leerRegistro(venta);
  while(venta.numCli <> 0) do
  begin
    ArmarDatoAGuardar (venta, elem);
    insertarElementoEnLista ( v [venta.mes], elem);
    leerRegistro (venta);
  end;
end;

   
   
procedure Merge (v: vector; var l: lista);
{ Implementar un módulo que reciba la estructura generada en a), genere y retorne una nueva estructura, ordenada por número de cliente, con cada una de
 las ventas almacenadas. }
   
   Procedure minimo (var v: vector; var ventaMin: datoAGuardar);
   var indiceMin,i:integer;
   Begin
     ventaMin.numCli:= valorGrande;
     for i:= 1 to dimF do
     begin
       if (v[i] <> nil) 
       then if (v [i]^.dato.numCli <= ventaMin.numCli) 
            then begin
                   indiceMin:= i;
                   ventaMin:= v [i]^.dato;  
                 end; 
     end;
     if (ventaMin.numCli <> valorGrande) 
     then v[indiceMin]:= v[indiceMin]^.sig;
   end;
   
   procedure AgregarAtras (var l, ult: lista; elem: datoAGuardar); 
   var  nuevo: lista;
   begin 
     new (nuevo); nuevo^.dato:= elem; nuevo^.sig := nil;
     if (l = nil) then l:= nuevo
                  else ult^.sig := nuevo;
     ult := nuevo;
   end;

var ventaMin: datoAGuardar; ult: lista;
begin
  l:= nil;
  minimo (v, ventaMin);
  while (ventaMin.numCli <> valorGrande) do
  begin
    agregarAtras(l, ult, ventaMin);
    minimo (v, ventaMin);
  end;
end;  

function CumpleCliente(l: lista; numeroClienteABuscar: integer; codigoExcursionABuscar: rangoExcursiones): boolean;
{ Implemente un módulo recursivo que reciba la estructura generada en b), un número de cliente y un código de excursión, y retorne verdadero si dicho 
cliente realizó al menos 3 excursiones incluyendo al código de excursión recibido, o falso en caso contrario.} 
   procedure buscar (l: lista; numeroClienteABuscar: integer; codigoExcursionABuscar: rangoExcursiones; var cantExcursiones: integer; var esta: boolean; var cumple: boolean);
   begin
     if (l = nil) or (l^.dato.numCli > numeroClienteABuscar) 
     then cumple:= false
     else begin
            if (l^.dato.numCli = numeroClienteABuscar)
            then begin
                   cantExcursiones:= cantExcursiones + 1;
                   if (l^.dato.codExcursion = codigoExcursionAbuscar) then esta:= true;
                   if (esta) and (cantExcursiones >= 3) then cumple:= true
                                                        else buscar (l^.sig, numeroClienteABuscar, codigoExcursionABuscar, cantExcursiones, esta, cumple)
                 end
            else buscar (l^.sig, numeroClienteABuscar, codigoExcursionABuscar, cantExcursiones, esta, cumple)
          end;
   end;
   
var cantExcursiones: integer;
    esta, cumple: boolean;
begin
  cantExcursiones:= 0;
  esta:= false;
  buscar (l, numeroClienteABuscar, codigoExcursionABuscar, cantExcursiones, esta, cumple);
  CumpleCliente:= cumple;
end;

var v: vector;
    lMerge, aux: lista;
    numeroClienteABuscar: integer;
    codigoExcursionABuscar: rangoExcursiones;
begin
  CargarInformacion (v);
  Merge (v, lMerge);
  if(lMerge = nil)
  then begin
         writeln;
         writeln('////////// No se realizaron ventas //////////');
         writeln;
       end
  else begin
         writeln;
         writeln ('////////// Merge realizado ///////////');
         writeln;
         aux:= lMerge;
         while (aux <> nil)
         do begin
              writeln (aux^.dato.numCli, '  ', aux^.dato.codExcursion);
              aux:= aux^.sig;
            end;
         write ('Ingrese numero de cliente a buscar: ');
         readln (numeroClienteABuscar);
         write ('Ingrese codigo de excursion a buscar: ');
         readln (codigoExcursionABuscar);
         writeln;
         if (CumpleCliente (lMerge, numeroClienteABuscar, codigoExcursionABuscar))
         then writeln ('El cliente con numero ', numeroClienteABuscar, ' cumple con lo pedido')
         else writeln ('El cliente con numero ', numeroClienteABuscar, ' no cumple con lo pedido');
       end;
end.
       
