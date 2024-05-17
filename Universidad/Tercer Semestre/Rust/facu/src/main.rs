/*
4- En base al ejercicio 10 del tp#3 implemente lo siguiente:

a- Realice todos los tests de la funcionalidad implementada obteniendo un coverage
de por lo menos 90%

b- Tanto los libros con sus copias como la administración de préstamos se realizan
sobre archivos en formato JSON. Realice las modificaciones pertinentes para poder hacerlo
así. No debe modificar los tests hechos en el punto a. Si puede agregar más en caso de que
haga métodos nuevos para cumplir con este punto . Recuerde también que se debe seguir
manteniendo un coverage de al menos 90%.
 */

/*
  crates usados: 
  cargo-tarpaulin = "0.25.2"       ----https://crates.io/crates/cargo-tarpaulin
  rand = "0.8.5"                   ----https://crates.io/crates/rand
  serde_json = "1.0.96"            ----https://crates.io/crates/serde_json
  serde = { version = "1.0", features = ["derive"] }
 */

/*
  Explicaciones y puntos de interes
    -linea 44 : Estructuras
    -linea 96 : implementaciones y nota
    -linea 281: Vector de prestamos
    -linea 311: nota fecha
    -linea 493: nota all news
    -linea 560: explicacion y nota IMPORTANTE
 */

 use rand::{distributions::{Alphanumeric, DistString},Rng, random};
 use serde::{Deserialize,Serialize};
 use std::{fs::File, clone, path};
 use std::io::prelude::*;
 use std::io::{BufWriter, Write};


fn main(){
}



//Estructuras
  #[derive(Clone,Serialize,Deserialize,PartialEq)]
  struct Bibloteca{
    nombre:String,
    dir:String,
    dis_libro:Vec<LibrosDis>,
    prestamos:Vec<Prestamo>
  }
#[derive(Clone,Serialize,Deserialize,PartialEq)]
  struct Prestamo{
    libro:Libro,
    cliente:Cliente,
    fecha_venc:Fecha,
    fecha_dev:Option<Fecha>,
    estado:String
  }
#[derive(Clone,Default,Serialize,Deserialize,PartialEq)]
  struct Cliente{
    nombre:String,
    telefono:String,
    dir_electronica:String
  }
#[derive(Clone,Serialize,Deserialize,PartialEq)]
  struct LibrosDis{
    ejemplares:u32,
    libro:Libro
  }

#[derive(Clone,Serialize,Deserialize,PartialEq)]
  struct Libro{
    titulo:String,
    autor:String,
    num_pag:u32,
    genero:Genero
  }

#[derive(Copy, Clone,Serialize,Deserialize,PartialEq)]
  enum Genero{
    Novela,
    Infantil,
    Tecnico,
    Otros
  }

  #[derive(Debug,Clone,Default,Serialize,Deserialize,PartialEq)]
  struct Fecha{
    dia: u8,
    mes:u8,
    anio:i32
  }

  //implementaciones

    /*
    Las implementaciones de bibloteca modificadas para poder leer, escribir y modificar
    archivos .json
   */

  impl Bibloteca{
    fn new(nombre:String,dir:String,dis_libro:Vec<LibrosDis>,prestamos:Vec<Prestamo>) -> Bibloteca {
      Bibloteca{nombre,dir,dis_libro,prestamos}
    }

    fn copias_disponibles(&self, libro_buscar:Libro,path: &str)->Option<u32> {
        if let Ok(mut file) = File::open(path) {
          let mut data = String::new();
          if let Ok(_) = file.read_to_string(&mut data) {
              if let Ok(json) = serde_json::from_str::<Vec<LibrosDis>>(&data) {
                   if let Some(cantidad) = json.iter().find(|l| l.libro == libro_buscar){
                    return Some(cantidad.ejemplares);
                  }
              } 
          } 
      }
      println!("El libro no se encuentra en la biblioteca.");
      None
    }


    fn tomar_copia(&mut self, libro_buscar: Libro,path: &str) {
       if let Ok(mut file) = File::open(path) {
        let mut data = String::new();
        if let Ok(_) = file.read_to_string(&mut data) {
            if let Ok(mut json) = serde_json::from_str::<Vec<LibrosDis>>(&data) {
                if let Some(index) = json.iter().position(|libro| libro.libro == libro_buscar) {
                    if json[index].ejemplares > 0 {
                        json[index].ejemplares -= 1;
                        self.guardar_datos_libros_dis(path, &json); // Guardar los cambios
                    } else {
                        println!("No hay copias disponibles de este libro para tomar.");
                    }
                } else {
                    println!("El libro no se encuentra en la biblioteca.");
                }
            }
        }
      } else {
          println!("direccion equivocada");
      }
    }

    fn devolver_copia(&mut self, libro_buscar: Libro,path: &str) {
      if let Ok(mut file) = File::open(path) {
          let mut data = String::new();
          if let Ok(_) = file.read_to_string(&mut data) {
              if let Ok(mut json) = serde_json::from_str::<Vec<LibrosDis>>(&data) {
                if let Some(index) = json.iter().position(|libro| libro.libro == libro_buscar) {
                  json[index].ejemplares += 1;
                  self.guardar_datos_libros_dis(path,&json);
                }else {
                  println!("El libro no se encuentra en la biblioteca.");
                }
              } 
          } 
        } else {
            println!("direccion equivocada");
        }
    }

    //agregado para guardar la informacion modificada
    fn guardar_datos_libros_dis(&self, path: &str, libros_disponibles: &[LibrosDis]) {
        if let Ok(mut file) = File::create(path) {
        if let Ok(json) = serde_json::to_string_pretty(libros_disponibles) {
            if let Err(err) = file.write_all(json.as_bytes()) {
                println!("Error al guardar los datos: {:?}", err);
            }
        }
      }
    }

    //agregado para guardar la informacion modificada
    fn guardar_datos_prestamos(&self,path: &str) {
        if let Ok(mut file) = File::create(path) {
            if let Ok(json) = serde_json::to_string_pretty(&self.prestamos) {
                if let Err(err) = file.write_all(json.as_bytes()) {
                    println!("Error al guardar los datos: {:?}", err);
                }
            }
        }
    }


    fn cantidad_de_ejemplares_en_prestamo(&mut self,hoy:Fecha,cliente:Cliente,path: &str) -> u32 {
      if let Ok(mut file) = File::open(path) {
        let mut data = String::new();
        if let Ok(_) = file.read_to_string(&mut data) {
            if let Ok(mut json) = serde_json::from_str::<Vec<Prestamo>>(&data) {
              return json.iter().filter(|p|p.cliente == cliente && p.estado == "En Prestamo").count() as u32;
          } 
        } 
      } 
      println!("Error al abrir el archivo para escritura.");
      return 0;
    }

    //agregado
    fn crear_prestamo(&mut self,fecha_venc:Fecha,cliente:Cliente,libro:Libro,path:&str){
      let estado = "en préstamo".to_string();
      let fecha_dev:Option<Fecha> = None;
      self.tomar_copia(libro.clone(), path);
      let aux = Prestamo { libro, cliente, fecha_venc, fecha_dev, estado };
      self.prestamos.push(aux);
    }


    fn realizar_prestamo(&mut self,mut hoy:Fecha,cliente:Cliente,libro:Libro,path_dis:&str,path_prestamo:&str) ->bool {
        if self.copias_disponibles(libro.clone(),path_dis.clone()).unwrap() >= 1 && self.cantidad_de_ejemplares_en_prestamo(hoy.clone(),cliente.clone(),path_prestamo.clone()) < 5 {
              self.crear_prestamo(hoy, cliente, libro,path_dis);
              self.guardar_datos_prestamos(path_prestamo);
              return true;
          }
      return false;
    }
   
    //agregado
    fn devolver_prestamo(&self,libro:Libro,cliente:Cliente,path:&str) -> Option<Prestamo> {
      if let Ok(mut file) = File::open(path) {
        let mut data = String::new();
        if let Ok(_) = file.read_to_string(&mut data) {
              if let Ok(mut json) = serde_json::from_str::<Vec<Prestamo>>(&data) {
                  for i in 0..json.len() {
                    let aux_prestamo = &mut json[i];
                    if  aux_prestamo.cliente.telefono == cliente.telefono && aux_prestamo.libro.titulo == libro.titulo {
                      return Some(aux_prestamo.clone());
                    }
                  }
              }
        }
      }
      return None;
    }

    fn prestamos_por_vencer(&self,mut hoy:Fecha) ->Vec<Prestamo> {
      let mut aux:Vec<Prestamo> = Vec::new();
      if let Ok(mut file) = File::open("prestamos.json") {
        let mut data = String::new();
        if let Ok(_) = file.read_to_string(&mut data) {
          if let Ok(mut json) = serde_json::from_str::<Vec<Prestamo>>(&data) {
            aux = json.iter().filter(|por_vencer| por_vencer.estado == "En Prestamo" && !por_vencer.fecha_venc.es_mayor(hoy.clone())).cloned().collect()
          }
        }
      }
      return aux;
    }

    fn prestamos_vencido(&self,hoy:Fecha) ->Vec<Prestamo> {
      let mut aux:Vec<Prestamo> = Vec::new();
      if let Ok(mut file) = File::open("prestamos.json") {
        let mut data = String::new();
        if let Ok(_) = file.read_to_string(&mut data) {
              if let Ok(mut json) = serde_json::from_str::<Vec<Prestamo>>(&data) {
                  aux = json.iter().filter(|vencido| vencido.fecha_venc.es_mayor(hoy.clone())).cloned().collect()
              }
        }
      }
      return aux;
    }

    fn devolver_libro(&mut self,libro:Libro,cliente:Cliente,hoy:Fecha,path: &str) ->bool {

       if let Ok(mut file) = File::open(path) {
        let mut data = String::new();
        if let Ok(_) = file.read_to_string(&mut data) {
          if let Ok(mut json) = serde_json::from_str::<Vec<Prestamo>>(&data) {
            if let Some(encontre) = json.iter_mut().find(|p| p.libro == libro && p.cliente == cliente){
              encontre.estado = "devuelto".to_string();
              encontre.fecha_dev = Some(hoy);
            }
            self.guardar_datos_prestamos(path);
          }
        }
      }
      return false;
    }

    //Ver ultimo- Vector de prestamos random

    fn crear_vector_prestamos(&mut self,path_dis:&str,path_prestamo:&str){
      let binding = self.dis_libro.clone();
      let iterador = binding.iter();
      let hoy = crear_fecha_random();
      for aux_libros in iterador{
        if aux_libros.libro.titulo != "harry potter" {
          let mut rng = rand::thread_rng();
          for i in 0..rng.gen_range(2..5){
            let cliente_aux = crear_cliente_random();
            let libro = aux_libros.libro.clone();
            self.realizar_prestamo(hoy.clone(), cliente_aux, libro,path_dis,path_prestamo);
          }
        }
      }
    }
  }

  /*
  Las implementaciones de fechas utilizadas en el ejercicio 3 de la practica 3
   */

  impl Fecha{
    fn new(dia:u8,mes:u8,anio:i32) -> Fecha{
      Fecha{dia,mes,anio}
    }

    fn es_fecha_valida(&self) -> bool {
      let mut aux:bool = false;
      //ver si existe el mes
      if self.mes <= 12 && self.mes >= 1 {
        /*identificar mes. en este caso separamos
        a febrero de los otros meses*/
        if self.mes == 2 {
          //ver si es bisiesto
          if self.es_bisiesto() {
            //comprobar si el dia es valido 29 dias
              if self.dia <= 29 && self.dia >=1 {
                aux = true;
              }
          } else {
            //comprobar si el dia es valido 28 dias
              if self.dia <= 28 && self.dia >=1 {
                aux = true;
              }
          }
        } else {
          /*Partimos el año en dos secciones 
          (primeros 7 meses sin febrero)
          y luego los ultimos 5 meses */

          //de enero a julio
          if self.mes <= 7 {
            //Si es impar tiene 31 dias
            if self.mes % 2 != 0 {
              //comprobar si el dia es valido 31  dias
              if self.dia <= 31 && self.dia >=1 {
                aux = true;
              }
            } else {
              //comprobar si el dia es valido 30 dias
              if self.dia <= 30 && self.dia >=1 {
                aux = true;
              }
            }
          //de agosto a diciembre
          } else {
            if self.mes % 2 == 0 {
              //comprobar si el dia es valido 31  dias
              if self.dia <= 31 && self.dia >=1 {
                aux = true;
              }
            } else {
              //comprobar si el dia es valido 30 dias
              if self.dia <= 30 && self.dia >=1 {
                aux = true;
              }
            }
          }
        }
      }
      aux
    }

    fn es_bisiesto(&self) -> bool {
      let mut b:bool = false;
      if self.anio % 4 == 0 {
        b = true;
      }
      return b
    }

    fn sumar_dias(&mut self, dias: u8) {
      let mut aux = self.clone();
      aux.dia += dias;
      if aux.es_fecha_valida() {
        self.dia += dias;
      } else {
        aux.dia = aux.dia-dias;
        for i in 0..dias {
          aux.dia += 1;
          if !aux.es_fecha_valida() {
            aux.mes += 1;
            aux.dia = 1;
            if !aux.es_fecha_valida(){
              aux.anio += 1;
              aux.mes = 1;
            }
          }
        }
       self.dia = aux.dia;
       self.mes = aux.mes;
       self.anio = aux.anio;
      }
    }

    fn restar_dias(&mut self, dias: u8) {
        if self.dia > dias {
          self.dia = self.dia-dias;
        } else {
          for i in 0..dias {
            if self.dia - 1 != 0 {
              self.dia -= 1;
            } else {
              self.mes -= 1;
              if !self.es_fecha_valida(){
                self.anio -= 1;
                self.mes = 12;
              }
              //Vemos que dia es
              self.dia = 31;
              if !self.es_fecha_valida(){
                self.dia = 30;
                if !self.es_fecha_valida(){
                  self.dia = 29;
                  if !self.es_fecha_valida(){
                    self.dia = 28;
                  }
                }
              }
            }
          }
        
      }
    }


    fn es_mayor(&self,otra_fecha:Fecha) -> bool {
       if self.anio > otra_fecha.anio {
            true
        } else if self.anio == otra_fecha.anio {
            if self.mes > otra_fecha.mes {
                true
            } else if self.mes == otra_fecha.mes {
                self.dia > otra_fecha.dia
            } else {
                false
            }
        } else {
            false
        }
    }  

    //agregado
    fn modificar_fecha_random(&mut self) {
      let mut rng = rand::thread_rng();
      let anio:i32 = random();
      self.anio = anio;
      let mes:u8 = rng.gen_range(1..12);
      self.mes= mes;
      let dia:u8;
      if self.es_bisiesto() && self.mes==2 {
        dia= rng.gen_range(1..28);
      } else {
        dia = rng.gen_range(1..30);
      }
      self.dia = dia;
    }
  
  }
   
  
    

    /*
      Todos los news de cada estructura (a excepcion de fecha y bibloteca)
     */

    impl LibrosDis{
      fn new(libro:Libro,ejemplares: u32) -> LibrosDis {
      LibrosDis{ejemplares,libro}
      }
    }
    impl Libro{
      fn new(titulo:String,autor:String,num_pag:u32,genero:Genero) -> Libro {
        Libro{titulo,autor,num_pag,genero}
      }
    }

    impl Prestamo{
      fn new(libro:Libro,cliente:Cliente,fecha_venc:Fecha,fecha_dev:Option<Fecha>,estado:String) -> Prestamo{
        Prestamo{libro,cliente,fecha_venc,fecha_dev,estado}
      }
    }

    impl Cliente{
      fn new(nombre:String,telefono:String,dir_electronica:String) -> Cliente{
        Cliente{nombre,telefono,dir_electronica}
      }
    }



    /*NOTA: Hoy 12/06/23 Vi la clase de Mocking, yo habia hecho generadores 
    Random para cada estructura para probar el coverage, las dejo en el codigo final
    por simple comodidad, ya que todos los test crean estas estructuras

    Tambien hay un generador random para el vector de prestamos, pero ese 
    esta implementado dentro de la estructura bibloteca (Esto lo hice asi
    porque los prestamos son sobre los libros disponibles)
   */
    fn elegir_genero_random() -> Genero {
       let mut rng = rand::thread_rng();
       match rng.gen_range(0..=4) { 
            0 => Genero::Novela,
            1 => Genero::Infantil,
            2 => Genero::Tecnico,
            _ => Genero::Otros,
        }
    }

    fn crear_libro_random() ->Libro {
      let titulo= Alphanumeric.sample_string(&mut rand::thread_rng(), 2);
      let autor= Alphanumeric.sample_string(&mut rand::thread_rng(), 2);
      let num_pag:u32 = random();
      let genero = elegir_genero_random();
      Libro { titulo, autor, num_pag, genero}
    }

    fn crear_libro_dis_random() ->LibrosDis{
      let ejemplares:u32 = random();
      let libro = crear_libro_random();
      LibrosDis { ejemplares, libro }
    }

    fn crear_cliente_random() ->Cliente {
      let nombre = Alphanumeric.sample_string(&mut rand::thread_rng(), 4);
      let telefono = Alphanumeric.sample_string(&mut rand::thread_rng(), 1);
      let dir_electronica = Alphanumeric.sample_string(&mut rand::thread_rng(), 10);
      Cliente{nombre,telefono,dir_electronica}
    }

    fn crear_vector_libros_disponibles(path: &str) -> Vec<LibrosDis> {
      let limite:u8 = random();
      let mut vector:Vec<LibrosDis> = Vec::new();
      for i in 0..limite {
        let aux = crear_libro_dis_random();
        vector.push(aux);
      }
      let titulo = "harry potter".to_string();
      let autor = "Rowling".to_string();
      let num_pag = 200;
      let genero = Genero::Novela;
      let libro_buscar = Libro::new(titulo, autor, num_pag, genero);
      let libro_buscar_dis = LibrosDis::new(libro_buscar, 4);
      vector.push(libro_buscar_dis);
      let file = File::create(path).unwrap();
      let mut writer = BufWriter::new(file);
      serde_json::to_writer(&mut writer, &vector);
      return vector;
    }

    fn crear_fecha_random() ->Fecha {
      let mut aux = Fecha::new(1,1,1);
      aux.modificar_fecha_random();
      return aux;
      }
    

/*
      Comienzo de los test, el primer test crea los archivos "dis_libro.json"
      y "prestamos.json" de forma aleatoria, a excepcion del ultimo libro que es
      Harry Potter, este libro sera el que buscaremos. Estos archivos seran leidos
       por todos los test.

      En cada test se crean la misma bibloteca, la bibloteca de la unlp, y se buscara
      el mismo libro, harry potter.

      Edit: Tambien agrege a un cliente llamado Juan, el cual buscaremos en los test
      donde se pida buscar a un cliente
 */
#[test]
fn calcular_copias_disponibles_test() {
  //creamos el primero de los dos archivos
  let dis_libro = crear_vector_libros_disponibles("dis_libro.json");
  //creacion bibloteca
  let nombre = "bibloteca_unlp".to_string();
  let dir = "-".to_string();
  let prestamos:Vec<Prestamo> = Vec::new();
  let mut bibloteca = Bibloteca::new(nombre,dir,dis_libro,prestamos);
  //creamos el segundo
  bibloteca.crear_vector_prestamos("dis_libro.json","prestamos.json");
  //libro a buscar
  let titulo = "harry potter".to_string();
  let autor = "Rowling".to_string();
  let num_pag = 200;
  let genero = Genero::Novela;
  let libro_buscar = Libro::new(titulo, autor, num_pag, genero);
  let copias_disponibles = bibloteca.copias_disponibles(libro_buscar,"dis_libro.json");
  assert_eq!(bibloteca.dis_libro[bibloteca.dis_libro.len()-1].ejemplares,4);
  let esperable:Option<u32> = Some(4);
  assert_eq!(copias_disponibles,esperable);
}

#[test]
fn calcular_copias_disponibles_test_path_no_existe() {
  //creamos el primero de los dos archivos
  let dis_libro = crear_vector_libros_disponibles("dis_libro.json");
  //creacion bibloteca
  let nombre = "bibloteca_unlp".to_string();
  let dir = "-".to_string();
  let prestamos:Vec<Prestamo> = Vec::new();
  let mut bibloteca = Bibloteca::new(nombre,dir,dis_libro,prestamos);
  //creamos el segundo
  bibloteca.crear_vector_prestamos("dis_libro.json","prestamos.json");
  //libro a buscar
  let titulo = "harry potter".to_string();
  let autor = "Rowling".to_string();
  let num_pag = 200;
  let genero = Genero::Novela;
  let libro_buscar = Libro::new(titulo, autor, num_pag, genero);
  let copias_disponibles = bibloteca.copias_disponibles(libro_buscar,"no_existe");
  let esperable:Option<u32> = None;
  assert_eq!(copias_disponibles,esperable);
}

#[test]
fn calcular_copias_recogidas_test(){
 //creamos el primero de los dos archivos
  let dis_libro = crear_vector_libros_disponibles("dis_libro.json");
  //creacion bibloteca
  let nombre = "bibloteca_unlp".to_string();
  let dir = "-".to_string();
  let prestamos:Vec<Prestamo> = Vec::new();
  let mut bibloteca = Bibloteca::new(nombre,dir,dis_libro,prestamos);
  //creamos el segundo
  bibloteca.crear_vector_prestamos("dis_libro.json","prestamos.json");
  //libro a buscar
  let titulo = "harry potter".to_string();
  let autor = "Rowling".to_string();
  let num_pag = 200;
  let genero = Genero::Novela;
  let libro_buscar = Libro::new(titulo, autor, num_pag, genero);
  bibloteca.tomar_copia(libro_buscar,"dis_libro.json");
}

#[test]
fn calcular_copias_recogidas_test_varias_veces(){
 //creamos el primero de los dos archivos
  let dis_libro = crear_vector_libros_disponibles("dis_libro.json");
  //creacion bibloteca
  let nombre = "bibloteca_unlp".to_string();
  let dir = "-".to_string();
  let prestamos:Vec<Prestamo> = Vec::new();
  let mut bibloteca = Bibloteca::new(nombre,dir,dis_libro,prestamos);
  //creamos el segundo
  bibloteca.crear_vector_prestamos("dis_libro.json","prestamos.json");
  //libro a buscar
  let titulo = "harry potter".to_string();
  let autor = "Rowling".to_string();
  let num_pag = 200;
  let genero = Genero::Novela;
  let libro_buscar = Libro::new(titulo, autor, num_pag, genero);
  bibloteca.tomar_copia(libro_buscar.clone(),"dis_libro.json");
  bibloteca.tomar_copia(libro_buscar.clone(),"dis_libro.json");
  bibloteca.tomar_copia(libro_buscar.clone(),"dis_libro.json");
  bibloteca.tomar_copia(libro_buscar.clone(),"dis_libro.json");
  bibloteca.tomar_copia(libro_buscar.clone(),"dis_libro.json");
}

#[test]
fn calcular_copias_recogidas_test_error_libro(){
 //creamos el primero de los dos archivos
  let dis_libro = crear_vector_libros_disponibles("dis_libro.json");
  //creacion bibloteca
  let nombre = "bibloteca_unlp".to_string();
  let dir = "-".to_string();
  let prestamos:Vec<Prestamo> = Vec::new();
  let mut bibloteca = Bibloteca::new(nombre,dir,dis_libro,prestamos);
  //creamos el segundo
  bibloteca.crear_vector_prestamos("dis_libro.json","prestamos.json");
  //libro a buscar
  let titulo = "el lobo estepario".to_string();
  let autor = "Herman Hesse".to_string();
  let num_pag = 100;
  let genero = Genero::Otros;
  let libro_buscar = Libro::new(titulo, autor, num_pag, genero);
  bibloteca.tomar_copia(libro_buscar,"dis_libro.json");
}

#[test]
fn calcular_copias_recogidas_test_error_bibloteca(){
 //creamos el primero de los dos archivos
  let dis_libro = crear_vector_libros_disponibles("dis_libro.json");
  //creacion bibloteca
  let nombre = "bibloteca_unlp".to_string();
  let dir = "-".to_string();
  let prestamos:Vec<Prestamo> = Vec::new();
  let mut bibloteca = Bibloteca::new(nombre,dir,dis_libro,prestamos);
  //creamos el segundo
  bibloteca.crear_vector_prestamos("dis_libro.json","prestamos.json");
  //libro a buscar
  let titulo = "harry potter".to_string();
  let autor = "Rowling".to_string();
  let num_pag = 200;
  let genero = Genero::Novela;
  let libro_buscar = Libro::new(titulo, autor, num_pag, genero);
  bibloteca.tomar_copia(libro_buscar,"error");
}

#[test]
fn calcular_copias_devueltas_test(){
 //creamos el primero de los dos archivos
  let dis_libro = crear_vector_libros_disponibles("dis_libro.json");
  //creacion bibloteca
  let nombre = "bibloteca_unlp".to_string();
  let dir = "-".to_string();
  let prestamos:Vec<Prestamo> = Vec::new();
  let mut bibloteca = Bibloteca::new(nombre,dir,dis_libro,prestamos);
  //creamos el segundo
  bibloteca.crear_vector_prestamos("dis_libro.json","prestamos.json");
  //libro a buscar
  let titulo = "el lobo estepario".to_string();
  let autor = "Herman Hesse".to_string();
  let num_pag = 100;
  let genero = Genero::Otros;
  let libro_buscar = Libro::new(titulo, autor, num_pag, genero);
  bibloteca.devolver_copia(libro_buscar,"dis_libro.json");
}

#[test]
fn calcular_copias_devueltas_test_libro_error(){
 //creamos el primero de los dos archivos
  let dis_libro = crear_vector_libros_disponibles("dis_libro.json");
  //creacion bibloteca
  let nombre = "bibloteca_unlp".to_string();
  let dir = "-".to_string();
  let prestamos:Vec<Prestamo> = Vec::new();
  let mut bibloteca = Bibloteca::new(nombre,dir,dis_libro,prestamos);
  //creamos el segundo
  bibloteca.crear_vector_prestamos("dis_libro.json","prestamos.json");
  //libro a buscar
  let titulo = "harry potter".to_string();
  let autor = "Rowling".to_string();
  let num_pag = 200;
  let genero = Genero::Novela;
  let libro_buscar = Libro::new(titulo, autor, num_pag, genero);
  bibloteca.devolver_copia(libro_buscar,"dis_libro.json");
}

#[test]
fn calcular_copias_devueltas_test_error_path(){
 //creamos el primero de los dos archivos
  let dis_libro = crear_vector_libros_disponibles("dis_libro.json");
  //creacion bibloteca
  let nombre = "bibloteca_unlp".to_string();
  let dir = "-".to_string();
  let prestamos:Vec<Prestamo> = Vec::new();
  let mut bibloteca = Bibloteca::new(nombre,dir,dis_libro,prestamos);
  //creamos el segundo
  bibloteca.crear_vector_prestamos("dis_libro.json","prestamos.json");
  //libro a buscar
  let titulo = "harry potter".to_string();
  let autor = "Rowling".to_string();
  let num_pag = 200;
  let genero = Genero::Novela;
  let libro_buscar = Libro::new(titulo, autor, num_pag, genero);
  bibloteca.devolver_copia(libro_buscar,"error");
}

#[test]
fn guardar_datos_prestamo_error_path(){
 //creamos el primero de los dos archivos
  let dis_libro = crear_vector_libros_disponibles("dis_libro.json");
  //creacion bibloteca
  let nombre = "bibloteca_unlp".to_string();
  let dir = "-".to_string();
  let prestamos:Vec<Prestamo> = Vec::new();
  let mut bibloteca = Bibloteca::new(nombre,dir,dis_libro,prestamos);
  //creamos el segundo
  bibloteca.crear_vector_prestamos("dis_libro.json","prestamos.json");
  bibloteca.guardar_datos_prestamos("prueba");
}


//creo que esta mal
#[test]
fn cantidad_de_ejemplares_en_prestamo_test() {
   //creamos el primero de los dos archivos
  let dis_libro = crear_vector_libros_disponibles("dis_libro.json");
  //creacion bibloteca
  let nombre = "bibloteca_unlp".to_string();
  let dir = "-".to_string();
  let prestamos:Vec<Prestamo> = Vec::new();
  let mut bibloteca = Bibloteca::new(nombre,dir,dis_libro,prestamos);
  //creamos el segundo
  bibloteca.crear_vector_prestamos("dis_libro.json","prestamos.json");
  //creacion de fecha y cliente 
  let libro_prestar = Libro::new("harry potter".to_string(), "Rowling".to_string(), 200, Genero::Novela);
  let cliente_prestar = Cliente::new("Juan".to_string(), "10".to_string(), "a@.com".to_string());
  let fecha_de_hoy = Fecha::new(14, 6, 2023);
  bibloteca.realizar_prestamo(fecha_de_hoy.clone(), cliente_prestar.clone(), libro_prestar.clone(),"dis_libro.json","prestamos.json");
  bibloteca.realizar_prestamo(fecha_de_hoy.clone(), cliente_prestar.clone(), libro_prestar.clone(),"dis_libro.json","prestamos.json");
  bibloteca.cantidad_de_ejemplares_en_prestamo(fecha_de_hoy, cliente_prestar,"prestamos.json");
}

#[test]
fn cantidad_de_ejemplares_en_prestamo_error_path_test() {
   //creamos el primero de los dos archivos
  let dis_libro = crear_vector_libros_disponibles("dis_libro.json");
  //creacion bibloteca
  let nombre = "bibloteca_unlp".to_string();
  let dir = "-".to_string();
  let prestamos:Vec<Prestamo> = Vec::new();
  let mut bibloteca = Bibloteca::new(nombre,dir,dis_libro,prestamos);
  //creamos el segundo
  bibloteca.crear_vector_prestamos("dis_libro.json","prestamos.json");
  //creacion de fecha y cliente 
  let libro_prestar = Libro::new("harry potter".to_string(), "Rowling".to_string(), 200, Genero::Novela);
  let cliente_prestar = Cliente::new("Juan".to_string(), "10".to_string(), "a@.com".to_string());
  let fecha_de_hoy = Fecha::new(14, 6, 2023);
  bibloteca.cantidad_de_ejemplares_en_prestamo(fecha_de_hoy, cliente_prestar,"error");
}

#[test]
fn realizar_prestamo_test() {
  //creamos el primero de los dos archivos
  let dis_libro = crear_vector_libros_disponibles("dis_libro.json");
  //creacion bibloteca
  let nombre = "bibloteca_unlp".to_string();
  let dir = "-".to_string();
  let prestamos:Vec<Prestamo> = Vec::new();
  let mut bibloteca = Bibloteca::new(nombre,dir,dis_libro,prestamos);
  //creamos el segundo
  bibloteca.crear_vector_prestamos("dis_libro.json","prestamos.json");
  //Creacion de libro, cliente y fecha
  let libro_prestar = Libro::new("harry potter".to_string(), "Rowling".to_string(), 200, Genero::Novela);
  let cliente_prestar = Cliente::new("Juan".to_string(), "10".to_string(), "a@.com".to_string());
  let fecha_de_hoy = Fecha::new(14, 6, 2023);
  bibloteca.realizar_prestamo(fecha_de_hoy, cliente_prestar, libro_prestar,"dis_libro.json","prestamos.json");
}

#[test]
fn realizar_prestamo_mas_de_cinco_veces_test() {
  //creamos el primero de los dos archivos
  let dis_libro = crear_vector_libros_disponibles("dis_libro.json");
  //creacion bibloteca
  let nombre = "bibloteca_unlp".to_string();
  let dir = "-".to_string();
  let prestamos:Vec<Prestamo> = Vec::new();
  let mut bibloteca = Bibloteca::new(nombre,dir,dis_libro,prestamos);
  //creamos el segundo
  bibloteca.crear_vector_prestamos("dis_libro.json","prestamos.json");
  //Creacion de libro, cliente y fecha
  let libro_prestar = Libro::new("harry potter".to_string(), "Rowling".to_string(), 200, Genero::Novela);
  let cliente_prestar = Cliente::new("Juan".to_string(), "10".to_string(), "a@.com".to_string());
  let fecha_de_hoy = Fecha::new(14, 6, 2023);
  bibloteca.realizar_prestamo(fecha_de_hoy.clone(), cliente_prestar.clone(), libro_prestar.clone(),"dis_libro.json","prestamos.json");
 bibloteca.realizar_prestamo(fecha_de_hoy.clone(), cliente_prestar.clone(), libro_prestar.clone(),"dis_libro.json","prestamos.json");
  bibloteca.realizar_prestamo(fecha_de_hoy.clone(), cliente_prestar.clone(), libro_prestar.clone(),"dis_libro.json","prestamos.json");
  bibloteca.realizar_prestamo(fecha_de_hoy.clone(), cliente_prestar.clone(), libro_prestar.clone(),"dis_libro.json","prestamos.json");
  bibloteca.realizar_prestamo(fecha_de_hoy.clone(), cliente_prestar.clone(), libro_prestar.clone(),"dis_libro.json","prestamos.json");
  bibloteca.realizar_prestamo(fecha_de_hoy.clone(), cliente_prestar.clone(), libro_prestar.clone(),"dis_libro.json","prestamos.json");
}

#[test]
fn devolver_prestamo_test() {
  //creamos el primero de los dos archivos
  let dis_libro = crear_vector_libros_disponibles("dis_libro.json");
  //creacion bibloteca
  let nombre = "bibloteca_unlp".to_string();
  let dir = "-".to_string();
  let prestamos:Vec<Prestamo> = Vec::new();
  let mut bibloteca = Bibloteca::new(nombre,dir,dis_libro,prestamos);
  //creamos el segundo
  bibloteca.crear_vector_prestamos("dis_libro.json","prestamos.json");
  //Creacion de libro y cliente
  let libro_prestar = Libro::new("harry potter".to_string(), "Rowling".to_string(), 200, Genero::Novela);
  let cliente_prestar = Cliente::new("Juan".to_string(), "10".to_string(), "a@.com".to_string());
  let fecha_de_hoy = Fecha::new(14, 6, 2023);
  bibloteca.realizar_prestamo(fecha_de_hoy, cliente_prestar.clone(), libro_prestar.clone(),"dis_libro.json","prestamos.json");
  bibloteca.devolver_prestamo( libro_prestar, cliente_prestar,"prestamos.json");
}

#[test]
fn devolver_prestamo_error_path_test() {
  //creamos el primero de los dos archivos
  let dis_libro = crear_vector_libros_disponibles("dis_libro.json");
  //creacion bibloteca
  let nombre = "bibloteca_unlp".to_string();
  let dir = "-".to_string();
  let prestamos:Vec<Prestamo> = Vec::new();
  let mut bibloteca = Bibloteca::new(nombre,dir,dis_libro,prestamos);
  //creamos el segundo
  bibloteca.crear_vector_prestamos("dis_libro.json","prestamos.json");
  //Creacion de libro y cliente
  let libro_prestar = Libro::new("harry potter".to_string(), "Rowling".to_string(), 200, Genero::Novela);
  let cliente_prestar = Cliente::new("Juan".to_string(), "10".to_string(), "a@.com".to_string());
  let fecha_de_hoy = Fecha::new(14, 6, 2023);
  bibloteca.realizar_prestamo(fecha_de_hoy, cliente_prestar.clone(), libro_prestar.clone(),"dis_libro.json","prestamos.json");
  bibloteca.devolver_prestamo( libro_prestar, cliente_prestar,"error");
}

#[test]
fn prestamos_por_vencer_test() {
  //creacion bibloteca
  let nombre = "bibloteca_unlp".to_string();
  let dir = "-".to_string();
  let dis_libro = Vec::new();
  let prestamos:Vec<Prestamo> = Vec::new();
  let bibloteca = Bibloteca::new(nombre,dir,dis_libro,prestamos);
  //Creacion de fecha
  let fecha_de_hoy = Fecha::new(14, 6, 2023);
  bibloteca.prestamos_por_vencer(fecha_de_hoy);
}


#[test]
fn prestamos_vencido_test() {
  //creacion bibloteca
  let nombre = "bibloteca_unlp".to_string();
  let dir = "-".to_string();
  let dis_libro = Vec::new();
  let prestamos:Vec<Prestamo> = Vec::new();
  let bibloteca = Bibloteca::new(nombre,dir,dis_libro,prestamos);
  //Creacion de fecha
  let fecha_de_hoy = Fecha::new(14, 6, 2023);
  bibloteca.prestamos_vencido(fecha_de_hoy);
}

#[test]
fn devolver_libro_test() {
   //creamos el primero de los dos archivos
  let dis_libro = crear_vector_libros_disponibles("dis_libro.json");
  //creacion bibloteca
  let nombre = "bibloteca_unlp".to_string();
  let dir = "-".to_string();
  let prestamos:Vec<Prestamo> = Vec::new();
  let mut bibloteca = Bibloteca::new(nombre,dir,dis_libro,prestamos);
  //creamos el segundo
  bibloteca.crear_vector_prestamos("dis_libro.json","prestamos.json");
  //Creacion de libro y cliente
  let libro_prestar = Libro::new("harry potter".to_string(), "Rowling".to_string(), 200, Genero::Novela);
  let cliente_prestar = Cliente::new("Juan".to_string(), "10".to_string(), "a@.com".to_string());
  let fecha_de_hoy = Fecha::new(14, 6, 2023);
  bibloteca.realizar_prestamo(fecha_de_hoy.clone(), cliente_prestar.clone(), libro_prestar.clone(),"dis_libro.json","prestamos.json");
  bibloteca.devolver_libro( libro_prestar, cliente_prestar,fecha_de_hoy,"prestamos.json");
}

#[test]
fn devolver_libro_no_existe_test() {
   //creamos el primero de los dos archivos
  let dis_libro = crear_vector_libros_disponibles("dis_libro.json");
  //creacion bibloteca
  let nombre = "bibloteca_unlp".to_string();
  let dir = "-".to_string();
  let prestamos:Vec<Prestamo> = Vec::new();
  let mut bibloteca = Bibloteca::new(nombre,dir,dis_libro,prestamos);
  //creamos el segundo
  bibloteca.crear_vector_prestamos("dis_libro.json","prestamos.json");
  //Creacion de libro y cliente
  let libro_prestar = Libro::new("harry potter".to_string(), "Rowling".to_string(), 200, Genero::Novela);
  let cliente_prestar = Cliente::new("Juan".to_string(), "10".to_string(), "a@.com".to_string());
  let fecha_de_hoy = Fecha::new(14, 6, 2023);
  bibloteca.realizar_prestamo(fecha_de_hoy.clone(), cliente_prestar.clone(), libro_prestar.clone(),"dis_libro.json","prestamos.json");
  bibloteca.devolver_libro( libro_prestar, cliente_prestar,fecha_de_hoy,"no_existe");
}


#[test]
fn fecha_valida_test() {
   let fecha = Fecha::new(10,10,2010);
   assert_eq!(fecha.es_fecha_valida(),true);
}

#[test]
fn fecha_valida_test_bisiesto() {
   let fecha = Fecha::new(29,2,2020);
   assert_eq!(fecha.es_fecha_valida(),true);
}

#[test]
fn fecha_valida_test_febrero_no_bisiesto() {
   let fecha = Fecha::new(25,2,2020);
   assert_eq!(fecha.es_fecha_valida(),true);
}

#[test]
fn fecha_valida_test_invalida() {
   let fecha = Fecha::new(10,15,2010);
   assert_eq!(fecha.es_fecha_valida(),false);
}

#[test]
fn fecha_valida_test_30_dias_antes_de_julio() {
   let fecha = Fecha::new(10,6,2010);
   assert_eq!(fecha.es_fecha_valida(),true);
}

#[test]
fn fecha_valida_test_30_dias_despues_de_julio() {
   let fecha = Fecha::new(10,9,2010);
   assert_eq!(fecha.es_fecha_valida(),true);
}

#[test]
fn fecha_valida_test_otra_fecha() {
   let fecha = Fecha::new(1,1,2000);
   assert_eq!(fecha.es_fecha_valida(),true);
}

#[test]
fn sumar_dias_test() {
   let mut fecha = Fecha::new(1,1,2000);
   fecha.sumar_dias(2);
   let fecha_aux = Fecha::new(3,1,2000);
   assert_eq!(fecha,fecha_aux);
}

#[test]
fn sumar_dias_test_paso_mes() {
   let mut fecha = Fecha::new(30,1,2000);
   fecha.sumar_dias(10);
   let fecha_aux = Fecha::new(9,2,2000);
   assert_eq!(fecha,fecha_aux);
}

#[test]
fn sumar_dias_test_paso_anio() {
   let mut fecha = Fecha::new(31,12,2000);
   fecha.sumar_dias(15);
   let fecha_aux = Fecha::new(15,1,2001);
   assert_eq!(fecha,fecha_aux);
}

#[test]
fn restar_dias_test() {
   let mut fecha = Fecha::new(10,1,2000);
   fecha.restar_dias(2);
   let fecha_aux = Fecha::new(8,1,2000);
   assert_eq!(fecha,fecha_aux);
}

#[test]
fn restar_dias_test_paso_mes() {
   let mut fecha = Fecha::new(5,2,2000);
   fecha.restar_dias(10);
   let fecha_aux = Fecha::new(26,1,2000);
   assert_eq!(fecha.dia,fecha_aux.dia);
   assert_eq!(fecha.mes,fecha_aux.mes);
   assert_eq!(fecha.anio,fecha_aux.anio);
}

#[test]
fn restar_dias_test_paso_mes_30_dias() {
   let mut fecha = Fecha::new(5,7,2000);
   fecha.restar_dias(10);
   let fecha_aux = Fecha::new(25,6,2000);
   assert_eq!(fecha.dia,fecha_aux.dia);
   assert_eq!(fecha.mes,fecha_aux.mes);
   assert_eq!(fecha.anio,fecha_aux.anio);
}

#[test]
fn restar_dias_test_paso_mes_febrero_no_bisiesto() {
   let mut fecha = Fecha::new(5,3,2003);
   fecha.restar_dias(10);
   let fecha_aux = Fecha::new(23,2,2003);
   assert_eq!(fecha.dia,fecha_aux.dia);
   assert_eq!(fecha.mes,fecha_aux.mes);
   assert_eq!(fecha.anio,fecha_aux.anio);
}

#[test]
fn restar_dias_test_paso_mes_febrero_bisiesto() {
   let mut fecha = Fecha::new(5,3,2020);
   fecha.restar_dias(10);
   let fecha_aux = Fecha::new(24,2,2020);
   assert_eq!(fecha.dia,fecha_aux.dia);
   assert_eq!(fecha.mes,fecha_aux.mes);
   assert_eq!(fecha.anio,fecha_aux.anio);
}

#[test]
fn restar_dias_test_paso_anio() {
   let mut fecha = Fecha::new(1,1,2000);
   fecha.restar_dias(15);
   let fecha_aux = Fecha::new(17,12,1999);
   assert_eq!(fecha,fecha_aux);
}

#[test]
fn es_mayor_test_anio() {
   let mut fecha = Fecha::new(1,1,2000);
   let fecha_aux = Fecha::new(17,12,1999);
   assert_eq!(fecha.es_mayor(fecha_aux),true);
}

#[test]
fn no_es_mayor_test_anio() {
   let mut fecha = Fecha::new(1,1,2000);
   let fecha_aux = Fecha::new(17,12,2001);
   assert_eq!(fecha.es_mayor(fecha_aux),false);
}

#[test]
fn es_mayor_test_mes() {
   let mut fecha = Fecha::new(1,4,2000);
   let fecha_aux = Fecha::new(17,1,2000);
   assert_eq!(fecha.es_mayor(fecha_aux),true);
}

#[test]
fn no_es_mayor_test_mes() {
   let mut fecha = Fecha::new(1,1,2000);
   let fecha_aux = Fecha::new(17,12,2000);
   assert_eq!(fecha.es_mayor(fecha_aux),false);
}

#[test]
fn es_mayor_test_dia() {
   let mut fecha = Fecha::new(18,1,2000);
   let fecha_aux = Fecha::new(17,1,2000);
   assert_eq!(fecha.es_mayor(fecha_aux),true);
}




    
  