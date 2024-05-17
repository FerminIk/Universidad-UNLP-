program Ej2;
uses crt;
const
valor_alto = 32767;
type
alumnomstro = record
codalumno : integer;
ape : String;
nom : String;
cursadasaprov : integer;
finalesaprov : integer;
end;
alumnodetl = record
codalumno : integer;
info : 0..1; //(0 = cursada aprobada, 1 = final aprobado)
end;
maestro = file of alumnomstro;
detalle = file of alumnodetl;
procedure CrearArchivoAlumnoMaestro (var alumnos : maestro);
var
txt : Text;
aux : alumnomstro;
begin
Assign(txt,'alumnos_maestro.txt');
Reset(txt);
Rewrite(alumnos);
repeat
Readln(txt,aux.codalumno,aux.ape,aux.cursadasaprov,aux.finalesaprov,aux.ape);
Write(alumnos,aux);
until (EOF(txt));
close(txt);
close(alumnos);
end;
Procedure CrearArchivoAlumnoDetalle (var alumnos : detalle);
var
txt : Text;
aux : alumnodetl;
begin
Assign(txt,'alumnos_detalle.txt');
Reset(txt);
Rewrite(alumnos);
repeat
  Readln(txt,aux.codalumno,aux.info);
  Write(alumnos,aux);
until (EOF(txt));
close(txt);
close(alumnos);
end;
procedure Leer(var alumnos_detl : detalle; var alu_detl : alumnodetl);
begin
  if (NOT EOF(alumnos_detl)) then
    Read(alumnos_detl,alu_detl)
  else
    alu_detl.codalumno := valor_alto;
end;
Procedure ExportarAlumnos(var alumnos : maestro);
var
txt : Text;
aux : alumnomstro;
begin
Assign(txt,'alumnos_con_mas_de_4_cursadas.txt');
Rewrite(txt);
Reset(alumnos);
repeat
Read(alumnos,aux);
if (aux.cursadasaprov > 4) then
  Writeln(txt,'Codigo de alumno: ',aux.codalumno,' - Nombre y apellido: ',aux.nom,' ',aux.ape,' - Cantidad de cursadas aprobadas: ',aux.cursadasaprov,' - Cantidad de finales aprobados: ',aux.finalesaprov);
until (EOF(alumnos));
close(txt);
close(alumnos); 
end;
Procedure ActualizarArchivoMaestro (var alumnos : maestro; var alumnos_detl : detalle);
var
alu : alumnomstro;
alu_detl : alumnodetl;
cod : integer;
begin
Reset(alumnos);
Reset(alumnos_detl);
Read(alumnos,alu);
Leer(alumnos_detl,alu_detl);
while (alu_detl.codalumno <> valor_alto) do begin
  cod := alu_detl.codalumno;
  while (cod < alu.codalumno) do
    Read(alumnos,alu);
  if (cod = alu.codalumno) then begin
    while(cod = alu_detl.codalumno) AND (alu_detl.codalumno <> valor_alto) do begin
      if (alu_detl.info = 1) then
        alu.finalesaprov := alu.finalesaprov + 1
      else
        alu.cursadasaprov := alu.cursadasaprov + 1;
      Leer(alumnos_detl,alu_detl);
    end;
    Seek(alumnos,FilePos(alumnos)-1);
    Write(alumnos,alu);
    end  
  else
    while (alu_detl.codalumno > alu.codalumno) AND (alu_detl.codalumno <> valor_alto) do
      Leer(alumnos_detl,alu_detl);
end;
close(alumnos);
close(alumnos_detl);   
end;
var
alumnos : maestro;
alumnos_detl : detalle;
opc : integer;
BEGIN
Read(opc);
case opc of
  1: begin
     Assign(alumnos,'alumnos.dat');
     Assign(alumnos_detl,'alumnos_detalle.dat');
     CrearArchivoAlumnoMaestro(alumnos);
     CrearArchivoAlumnoDetalle(alumnos_detl);
     end;
  2: ActualizarArchivoMaestro(alumnos,alumnos_detl);
  3: ExportarAlumnos(alumnos);
end;	
END.

