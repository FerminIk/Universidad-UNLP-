program Ejercicio3;

uses SysUtils;

const
cantArchivos = 5;
valMAX = 9999;

type
  producto = record
           num: integer;
           vendidos: integer;
  end;

  maestro = record
           num: integer;
           nombre: string[20];
           descripcion: string[20];
           stockD: integer;
           stockM: integer;
           precio: real;
  end;

 archivo = file of producto;
 archivoMaestro = file of maestro;

 detalles = array[1..cantArchivos] of archivo;
 productos = array[1..cantArchivos] of producto;

procedure leer(var f: archivo; var p: producto);
begin
    if EOF(f) then begin p.num := valMAX; end
    else begin
        read(f, p);
    end;
end;

 procedure min(var det: detalles; var prod: productos; var ret: producto);
 var i, min: integer;
     pMinimo: producto;

begin
    i:= -1;
    pMinimo.num:= valMAX;
    for i:= 1 to cantArchivos do begin
       if ( prod[i].num < pMinimo.num ) then begin
           pMinimo:= prod[i];
           min:= i;
       end;
    end;
    ret:= pMinimo;

    leer(det[min], prod[min]);

    while ( prod[min].num = ret.num ) do begin
          ret.vendidos:= ret.vendidos + prod[min].vendidos;
          leer(det[min], prod[min]);
    end;
end;

procedure merge ( var m: archivoMaestro; var d: detalles; var p: productos );
var prod: producto;
    mae: maestro;
    i: integer;

begin
    // Prepara los archivos para el analisis
    for i:= 1 to cantArchivos do begin
     reset(d[i]);
     leer(d[i], p[i]);
    end;

    reset(m);

    // Recibe el producto con el menor codigo del vector de productos, avanza el vector de detalles solo para el archivo del que proviene
    min(d, p, prod);
    Read(m, mae);

    // While loop. Procesa todos los registros individuales del vector de archivos detalle
    while ( prod.num <> valMAX ) do begin // No hace falta considerar EOF del maestro porque se asume que los detalles van a terminarse antes
       // Avanzo el maestro hasta que coincida con el producto de codigo minimo. Si ya coincide quiere decir que hay que seguir trabajando sobre el mismo registro del maestro
       while ( mae.num <> prod.num ) do Read(m, mae);

       mae.stockD:= mae.stockD - prod.vendidos;
       // ------- Otros calculos -------

       // Se busca el siguiente producto con el codigo menor
       min(d,p,prod);
       // Si el codigo menor es distinto al que se venía computando, se graba el cambio del maestro
       if ( mae.num <> prod.num ) then begin
           seek(m, filepos(m)-1);
           write(m, mae);
           // ------- Otros informes -------
       end;
    end;

    // ------- Informes globales -------

    // Close de archivos. Los detalles deberian eliminarse
    for i:= 1 to cantArchivos do begin
     close(d[i]);
    end;
    close(m);
end;

var
    m: archivoMaestro;
    d: detalles;
    p: productos;

    i: integer;

begin
  for i:= 1 to cantArchivos do assign(d[i], 'detalle' + IntToStr(i));

  assign(m, 'maestro');
  merge(m, d, p);

end.


