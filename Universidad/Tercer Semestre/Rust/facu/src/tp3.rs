use std::{vec, collections::{LinkedList, VecDeque}, clone};


pub fn ej1() {
  #[derive(Debug)]
  struct Persona {
    nombre: String,
    edad: u8,
    direccion: Option<String>
  }
  impl Persona{
    //constructor punto a
    fn new(nombre: String,edad:u8,direccion:Option<String>) ->Persona{
      Persona{nombre,edad,direccion}
    }
    //punto b
    fn imprimir(&self){
      if let Some(data) = &self.direccion {
        println!(" nombre: {} edad: {} dirreccion: {} ",self.nombre,self.edad,data);
      } else {
        println!(" nombre: {} edad: {} no tiene direccion ",self.nombre,self.edad);
      }
    }
    //punto c
    fn obtener_edad(&self) ->u8{
      self.edad
    }
    //punto d
    fn actualizar_direccion(&mut self,nueva_direccion:Option<String>){
      self.direccion = nueva_direccion;
    }
  }
  let mut p1 = Persona::new("Fermin".to_string(), 21, Some("lp".to_string()));
  p1.imprimir();
  println!(" {} ",p1.obtener_edad());
  p1.actualizar_direccion(None);
  p1.imprimir();
}

pub fn ej2() {
  #[derive(Debug)]
  struct Rectangulo{
    longitud: i32,
    ancho: i32
  }
  impl Rectangulo{
    fn new(longitud:i32,ancho:i32) -> Rectangulo {
      Rectangulo{longitud,ancho}
    }

    fn calcular_area(&self) -> i32 {
      self.longitud * self.ancho
    }

    fn calcular_perimetro(&self) ->i32 {
      self.ancho*2 + self.longitud*2
    }
    fn es_cuadrado(&self) ->bool {
      let mut aux: bool = false;
      if self.ancho == self.longitud {
        aux = true;
      }
      return aux;
    }
  }

  let rec = Rectangulo::new(10,10);
  println!(" {} ",rec.calcular_area());
  println!(" {} ",rec.calcular_perimetro());
  println!(" {} ",rec.es_cuadrado());
}

pub fn ej3() {
#[derive(Debug,Clone)]
  struct Fecha{
    dia: u8,
    mes:u8,
    anio:i32
  }

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
          let mut aux = self.clone();
          for i in 0..dias {
            aux.dia -= 1;
            if !self.es_fecha_valida() {
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
                  self.dia = 28;
                  if !self.es_fecha_valida(){
                    self.dia = 29;
                  }
                }
              }
            }
          }
        
      }
    }


    fn es_mayor(&self,f:Fecha) -> bool {
      let mut aux:bool = false;
      if f.anio > self.anio {
        aux = true;
      } else {
        if f.anio == self.anio {
          if f.mes > self.mes {
            aux = true;
          } else {
            if f.mes == self.mes {
              if f.dia > self.dia {
                aux = true;
              } 
            }
          }
        }
      }
      return aux;
    }  
  
  }

  let mut f = Fecha::new(30,4,4);
  let f2 = Fecha::new(29,4,4);
  println!("a {}",f.es_fecha_valida());
  println!("b {}",f.es_bisiesto());
  f.sumar_dias(2);
  println!("c {}",f.dia);
  f.restar_dias(2);
  println!("d {}",f.dia);
  println!("e {}",f.es_mayor(f2));
}

pub fn ej4() {
  #[derive(Debug)]
  struct Triangulo{
    lado1 : i32,
    lado2 : i32,
    lado3 : i32
  }
#[derive(Debug)]
  enum Tipo {
    equilatero,
    isoceles,
    escaleno
  }

  impl Triangulo{
    fn new(lado1:i32,lado2:i32,lado3:i32) ->Triangulo {
      Triangulo {lado1,lado2,lado3}
    }

    fn determinar_tipo(&self) -> Tipo{
      let mut aux:Tipo;
      if self.lado1 == self.lado2{
        if self.lado1 == self.lado3 {
          aux = Tipo::equilatero;
        } else {
          aux = Tipo::isoceles;
        }
      } else {
        if self.lado1 == self.lado3 {
          aux = Tipo::isoceles;
        } else {
          if self.lado2 == self.lado3 {
            aux = Tipo::isoceles;
          } else {
           aux = Tipo::escaleno; 
          }
        }
      }
      return aux;
    }
    fn calcular_area(&self) -> i32{
      let S:i32 = self.calcular_perimetro() / 2;
      (S*(S-self.lado1)*(S-self.lado1)*(S-self.lado1))
    }
    fn calcular_perimetro(&self) -> i32{
      self.lado1 + self.lado2 + self.lado3
    }
  }

  let t = Triangulo::new(1,2,3);
  let tipo = t.determinar_tipo();
  println!("{:?}",tipo);
  println!("{} raiz  cuadrada",t.calcular_area());
  println!("{}",t.calcular_perimetro());

}

pub fn ej5() {
  struct Producto{
    nombre: String,
    precio: f64,
    num: i32
  }

  impl  Producto{
      fn new(nombre: String, precio: f64, num:i32) -> Producto{
        Producto{nombre,precio,num}
      }

      fn calcular_impuestos(&self,porcentaje_de_impuestos:f64) ->f64 {
        let aux:f64 = porcentaje_de_impuestos/100 as f64;
        let porcentaje = (aux * self.precio) as f64;
        return porcentaje
      }

      fn aplicar_descuento(&self,precio_descuento:f64) ->f64 {
        let aux:f64 = precio_descuento/100 as f64;
        let porcentaje = (aux * self.precio) as f64;
        return  porcentaje 
      }

      fn calcular_precio_total(&self) ->f64 {
        let  imp = self.calcular_impuestos(30.0);
        let  des = self.aplicar_descuento(20.0);
        println!("{} impuesto del producto",imp);
        println!("{} descuento del producto",des);
        return self.precio + (imp-des);
      }
  }

  let mut p = Producto::new("a".to_string(),20.0,0);
  println!("{}",p.calcular_impuestos(30.0));
  println!("{}",p.aplicar_descuento(20.0));
  println!("{}",p.calcular_precio_total());
}

pub fn ej6() {
  #[derive(Debug)]
  struct Examen{
    materia: String,
    nota: i32,
  }
  #[derive(Debug)]
  struct Estudiante{
    nombre: String,
    num_id: i32,
    notas_examen:Vec<Examen>
  }

  impl Examen{
    fn new(materia:String, nota:i32) -> Examen {
      Examen{materia,nota}
    }
  }
  
  impl Estudiante{
    fn new(nombre:String, num_id:i32,notas_examen:Vec<Examen>) -> Estudiante {
      Estudiante{nombre,num_id,notas_examen}
    }

    fn obtener_promedio(&self)-> f64 {
      let mut total:f64 = 0.0;
      let largo = self.notas_examen.len();
      for i in 1..largo{
        let elemento = self.notas_examen[i].nota;
        total += elemento as f64;
      }
      let result:f64 = total /  (self.notas_examen.len() as f64);
      return  result as f64;
    }

    fn obtener_calificacion_mas_alta(&self)-> i32 {
      let mut max = -1;
      let largo = self.notas_examen.len();
      for i in 0..largo  {
          let nota_act = self.notas_examen[i].nota;
          if nota_act > max {
            max =  nota_act;
          }
      }
      return max;
    }

    fn obtener_calificacion_mas_baja(&self)-> i32 {
      let mut min = 11;
      let largo = self.notas_examen.len();
      for i in 0..largo  {
          let nota_act = self.notas_examen[i].nota;
          if nota_act < min {
            min =  nota_act;
          }
      }
      return min;
    }

    
  }
  let mut vector:Vec<Examen> = Vec::new();
  for i in 1..7 {
    let aux = Examen::new("a".to_string(),i);
    vector.push(aux);
  }
  let est = Estudiante::new("Fermin".to_string(), 12999, vector);
  println!("{:?}",est.obtener_promedio());
  println!("{:?}",est.obtener_calificacion_mas_alta());
  println!("{:?}",est.obtener_calificacion_mas_baja());

}


pub fn ej7() {
  /*Entiendo como primario los colores base de la luz (RGB)
  Y no los colores primarios de la tinta (Carmesi, cyan y amarrillo)*/ 
  #[derive(Debug)]
  #[derive(PartialEq)]
  enum Colores {
    Rojo,
    Verde,
    Azul,
    Amarrillo,
    Blanco,
    Negro
  }

  struct Cant_Auto{
    capacidad:i32,
    autos: Vec<Auto>
  }

  impl Cant_Auto {
    fn new(capacidad:i32,autos:Vec<Auto>) -> Cant_Auto {
      Cant_Auto{capacidad,autos}
    }
  }

  struct ConcesionarioAuto{
    nombre:String,
    dir:String,
    cant:i32,
    cant_auto:Cant_Auto
  }

#[derive(Debug)]
#[derive(PartialEq)]
  struct Auto{
    marca:String,
    modelo:String,
    anio:i32,
    precio:f64,
    color: Colores
  }

  impl Auto{
    fn new(marca:String,modelo:String,anio:i32,precio:f64,color:Colores) -> Auto{
      Auto{marca,modelo,anio,precio,color}
    }

    fn calcular_precio(&self) ->f64 {
    let mut aux = self.precio;
      if (self.color == Colores::Rojo) || (self.color == Colores::Verde) || (self.color == Colores::Azul)  {
        aux += self.precio * (25.0 / 100.0);
      } else {
        aux -= self.precio * (10.0 / 100.0);
      }
      if self.marca == "BMW" {
        aux += self.precio * (15.0 / 100.0);
      }

      if self.anio < 2000 {
        aux -= self.precio * (5.0 / 100.0);
      }
      return aux
    }
  }

  impl ConcesionarioAuto {
    fn new(nombre:String,dir:String,cant:i32,cant_auto:Cant_Auto) -> ConcesionarioAuto {
      ConcesionarioAuto{nombre,dir,cant,cant_auto}
    }

    fn agregar_auto(&mut self, auto:Auto)-> bool {
      if self.cant_auto.capacidad <= self.cant+1 {
        self.cant_auto.autos.push(auto);
        return true;
      } else {
        return false;
      }
    }

    fn eliminar_auto(&mut self,auto:Auto) -> bool {
      if self.cant_auto.autos.len() != 0 {
        for i in 0..self.cant_auto.autos.len() {
          if (self.cant_auto.autos[i] == auto) {
            self.cant_auto.autos.remove(i);
            return true;
          }
        }
      }
      return false;
    }

    fn buscar_auto(&mut self,auto:Auto) -> Option<Auto> {
      if self.cant_auto.autos.len() != 0 {
        for i in 0..self.cant_auto.autos.len() {
          if (self.cant_auto.autos[i] == auto) {
            //Como busca un auto con el mismo auto, retorno la misma estructura ya que son iguales
            return Some(auto);
          }
        }
      }
      return  None;
    }
  }

  let a1 = Auto::new("Renault".to_string(), "1".to_string(), 2002, 1000.0, Colores::Negro);
  let a2 = Auto::new("BMW".to_string(), "2".to_string(), 1800, 3000.0, Colores::Azul);
  let a3 = Auto::new("Volksvagen".to_string(), "3".to_string(), 1999, 1200.0, Colores::Verde);
  let tamanio = 4;
  let mut vector: Vec<Auto> =Vec::with_capacity(tamanio);
  vector.push(a1);
  vector.push(a2);
  vector.push(a3);
  let mut ca: Cant_Auto = Cant_Auto::new(tamanio as i32, vector);
  let mut cons = ConcesionarioAuto::new("Juanito autos".to_string(), "dir".to_string(), 3, ca);
  for i in 0..3 {
    println!("{}",cons.cant_auto.autos[i].calcular_precio());
  }

  let a4 = Auto::new("Volksvagen".to_string(), "4".to_string(), 1999, 1500.0, Colores::Amarrillo);
  println!("{}",cons.agregar_auto(a4));
  let a4 = Auto::new("Volksvagen".to_string(), "4".to_string(), 1999, 1500.0, Colores::Amarrillo);
  println!("{}",cons.eliminar_auto(a4));
  let a2 = Auto::new("BMW".to_string(), "2".to_string(), 1800, 3000.0, Colores::Azul);
  println!("{:?}",cons.buscar_auto(a2));
}

pub fn ej8() {
  #[derive(PartialEq,Clone,Debug)]
  enum Genero {
    Rock,
    Pop,
    Rap,
    Jazz,
    otro
  }

  #[derive(PartialEq,Clone,Debug)]
  struct Cancion {
    titulo:String,
    artista:String,
    genero:Genero
  }

  #[derive(PartialEq,Clone,Debug)]
  struct Playlist {
    lista: Vec<Cancion>,
    nombre: String
  }

  impl Cancion {
    fn new(titulo:String,artista:String,genero:Genero) -> Cancion {
      Cancion { titulo, artista, genero }
    }
  }

  impl  Playlist {
    fn new(nombre:String) -> Playlist{
      let mut lista:Vec<Cancion> = Vec::new();
      Playlist{lista,nombre}
    }

    fn newWithList(nombre:String,lista:Vec<Cancion>) -> Playlist{
      Playlist{lista,nombre}
    }

    fn Agregar_Cancion(&mut self,cancion: Cancion) {
      self.lista.push(cancion);
    }

    fn Eliminar_Cancion(&mut self,cancion:Cancion) {
      if !self.lista.is_empty() {
        self.lista.retain(|c|cancion != *c);
      }
    }

    fn mover_cancion(&mut self,pos:usize,pos_final:usize){
      if !self.lista.is_empty() {
        if self.lista.len() <= pos_final {
          self.lista.swap(pos as usize, pos_final as usize);
        }
      }
    }

    fn buscar_cancion_por_nombre(&mut self,titulo_input:String)  {
      self.lista.iter().find(|c|c.titulo == titulo_input);
    }  

    fn canciones_genero(&self, gen:Genero) -> Vec<Cancion> {
      self.lista.iter().filter(|c|c.genero == gen).cloned().collect()
    }

    fn canciones_artista(&self, art:String) -> Vec<Cancion> {
      self.lista.iter().filter(|c|c.artista == art).cloned().collect()
    }

    fn cambiar_titulo(&mut self,nuevo_nombre:String) {
      self.nombre = nuevo_nombre;
    }
    
    fn vaciar_playlist(&mut self) {
      self.lista = Vec::new();
    }
  }
}


pub fn ej9() {
  #[derive(Debug,Clone,PartialEq)]
  enum Animal {
      Perro,
      Gato,
      Caballo,
      Otro
  }

  #[derive(Debug,Clone,PartialEq)]
  struct Duenio {
    nombre:String,
    dir:String,
    nro:String
  }

  impl Duenio{
    fn new(nombre:String,
    dir:String,
    nro:String,) ->Duenio {
      Duenio { nombre, dir, nro }
    }
  }

  #[derive(Debug,Clone,PartialEq)]
  struct Mascota {
    nombre:String,
    edad:i32,
    animal:Animal,
    duenio:Duenio
  }

  impl Mascota {
    fn new(nombre:String,
    edad:i32,
    animal:Animal,
    duenio:Duenio) ->Mascota {
      Mascota { nombre, edad, animal, duenio }
    }
  }

  #[derive(Debug,Clone,PartialEq)]
  struct Atencion {
    datos_mascota:Mascota,
    diagnostico:String,
    tratamiento:String,
    fecha_siguiente:Fecha
  }

  #[derive(Debug,Clone,PartialEq)]
  struct Fecha{
    dia: u8,
    mes:u8,
    anio:i32
  }

  

   struct Veterinaria {
    nombre:String,
    direccion:String,
    id:i32,
    cola_atencion:VecDeque<Mascota>,
    reg_atencion:Vec<Atencion>
  }

  impl Veterinaria{
     fn new( nombre:String, direccion:String, id:i32, cola_atencion:VecDeque<Mascota>, reg_atencion:Vec<Atencion>) -> Veterinaria{
        Veterinaria {nombre, direccion, id, cola_atencion, reg_atencion}
    }

    fn agregar_mascota(&mut self,mascota:Mascota) {
      self.cola_atencion.push_back(mascota);
    }

    fn agregar_mascota_con_prioridad(&mut self,mascota:Mascota) {
      self.cola_atencion.push_front(mascota);
    }

    fn registrar_atencion(&mut self,diagnostico:String,tratamiento:String,fecha_siguiente:Fecha) {
      let datos_mascota: Mascota = self.cola_atencion.pop_front().expect("error"); 
      self.reg_atencion.push(Atencion { datos_mascota, diagnostico, tratamiento, fecha_siguiente});
    }

    fn atender(&mut self, diag:String,trat:String,fecha:Fecha) {
      self.registrar_atencion(diag,trat,fecha);
    }

    fn eliminar_por_nro(&mut self,nro_cola:usize) {
      self.cola_atencion.remove(nro_cola);
    }

    fn eliminar_por_mascota(&mut self,mascota:Mascota) {
      self.cola_atencion.retain(|m| *m != mascota);
    }

    fn buscar_atencion(&mut self,nom_mas:String,nom_due:String,nro:String) ->Option<&mut Atencion> {
      let iterador = self.reg_atencion.iter_mut();
      for reg_actual in iterador {
        if reg_actual.datos_mascota.nombre == nom_mas {
          if (reg_actual.datos_mascota.duenio.nombre == nom_due) && (reg_actual.datos_mascota.duenio.nro == nro) {
            return Some(reg_actual);
          }
        }
      }
      return None;
    }

    fn buscar_y_modificar_atencion(&mut self,nom_mas:String,nom_due:String,nro:String) {
      let aux = self.buscar_atencion(nom_mas, nom_due, nro);
      if aux.is_some() {
        let a = aux.unwrap();
        a.diagnostico = "modificar".to_string();
      }
    }

    fn buscar_y_modificar_atencion_fecha(&mut self,nom_mas:String,nom_due:String,nro:String,fec:Fecha) {
      let aux = self.buscar_atencion(nom_mas, nom_due, nro);
      if aux.is_some() {
        let a = aux.unwrap();
        a.fecha_siguiente = fec;
      }
    }

    fn buscar_y_eliminar(&mut self,atencion:Atencion)  {
      self.reg_atencion.retain(|a|*a != atencion);
    }
  }

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
          let mut aux = self.clone();
          for i in 0..dias {
            aux.dia -= 1;
            if !self.es_fecha_valida() {
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
                  self.dia = 28;
                  if !self.es_fecha_valida(){
                    self.dia = 29;
                  }
                }
              }
            }
          }
        
      }
    }


    fn es_mayor(&self,f:Fecha) -> bool {
      let mut aux:bool = false;
      if f.anio > self.anio {
        aux = true;
      } else {
        if f.anio == self.anio {
          if f.mes > self.mes {
            aux = true;
          } else {
            if f.mes == self.mes {
              if f.dia > self.dia {
                aux = true;
              } 
            }
          }
        }
      }
      return aux;
    }  
  
  }
}

pub fn ej10() {
#[derive(PartialEq,Clone,Debug)]
  struct Biblioteca{
    nombre:String,
    dir:String,
    libros_disponibles:Vec<Libros_Dis>,
    prestamos:Vec<Prestamo>
  }

  impl Biblioteca {
    fn new( nombre:String,dir:String,libros_disponibles:Vec<Libros_Dis>,prestamos:Vec<Prestamo>) -> Biblioteca {
      Biblioteca{nombre,dir,libros_disponibles,prestamos}
    }
  }


#[derive(PartialEq,Clone,Debug)]
  struct Prestamo{
    libro:Libro,
    cliente:Cliente,
    fecha_venc:Fecha,
    fecha_dev:Option<Fecha>,
    estado:String
  }

  impl Prestamo{
    fn new(libro:Libro,cliente:Cliente,fecha_venc:Fecha,fecha_dev:Option<Fecha>,estado:String) ->Prestamo {
      Prestamo { libro , cliente, fecha_venc, fecha_dev, estado}
    }
  }


#[derive(PartialEq,Clone,Debug)]
  struct Cliente{
    nombre:String,
    telefono:String,
    dir_electronica:String
  }

  impl Cliente{
    fn new( nombre:String,telefono:String,dir_electronica:String) ->Cliente {
      Cliente { nombre , telefono, dir_electronica}
    }
  }

#[derive(PartialEq,Clone,Debug)]
  struct Libros_Dis{
    ejemplares:u32,
    libro:Libro
  }

  impl Libros_Dis{
    fn new(ejemplares:u32,libro:Libro) ->Libros_Dis {
      Libros_Dis {ejemplares,libro}
    }
  }

#[derive(PartialEq,Clone,Debug)]
  struct Libro{
    titulo:String,
    autor:String,
    num_pag:i32,
    genero:Genero
  }

  impl Libro{
    fn new(titulo:String,autor:String,num_pag:i32,genero:Genero) ->Libro {
      Libro { titulo , autor, num_pag, genero}
    }
  }

#[derive(PartialEq,Clone,Debug)]
  enum Genero{
    novela,
    infantil,
    tecnico,
    otros
  }

  

  #[derive(PartialEq,Clone,Debug,Copy)]
  struct Fecha{
    dia: u8,
    mes:u8,
    anio:i32
  }

  
  impl Biblioteca{
    fn copias_disponibles(&self, libro:&Libro) -> u32{
        if let Some(copias) = self.libros_disponibles.iter().find(|l| l.libro == *libro){
            return copias.ejemplares
        }
        return 0
    }

    fn restar_un_ejemplar(&mut self, libro: &Libro) -> bool {
    if self.copias_disponibles(libro) >= 1 {
        if let Some(copias) = self.libros_disponibles.iter_mut().find(|aux| aux.libro == *libro) {
            copias.ejemplares -= 1;
            return true;
        }
    }
    false
  }

  
    fn sumar_un_ejemplar(&mut self, libro: &Libro) -> bool {
    if self.copias_disponibles(libro) >= 1 {
        if let Some(copias) = self.libros_disponibles.iter_mut().find(|aux| aux.libro == *libro) {
            copias.ejemplares += 1;
            return true;
        }
    }
    false
  }

    fn cantidad_de_ejemplares_en_prestamo(&self,cliente:&Cliente) ->u32{
      self.prestamos.iter().filter(|p|p.cliente == *cliente && p.estado == "En Prestamo").count() as u32
    }


    fn crear_prestamo(&mut self,fecha_venc:Fecha,cliente:Cliente,libro:Libro) -> bool{
      let estado = "En Prestamo".to_string();
      let fecha_dev:Option<Fecha> = None;
      let aux = Prestamo { libro, cliente, fecha_venc, fecha_dev, estado };
      self.prestamos.push(aux);
      false
    }

    fn realizar_prestamo(&mut self,mut hoy:Fecha,cliente:Cliente,libro:Libro) ->bool {
      if self.copias_disponibles(&libro) >= 1 && self.cantidad_de_ejemplares_en_prestamo(&cliente) < 5 {
        self.crear_prestamo(hoy, cliente, libro);
        return true;
      }
      false
    }


    fn prestamos_por_vencer(&self, dias: Fecha) -> Vec<Prestamo> {
    self.prestamos.iter().filter(|por_vencer| por_vencer.estado == "En Prestamo" && !por_vencer.fecha_venc.es_mayor(dias)).cloned().collect()
}


    fn prestamos_vencido(&self,hoy:Fecha) ->Vec<Prestamo> {
      self.prestamos.iter().filter(|vencido| vencido.fecha_venc.es_mayor(hoy)).cloned().collect()
    }

    fn buscar_prestamo(&self,cliente:Cliente,libro:Libro) -> bool {
      if let Some(encontre) = self.prestamos.iter().find(|buscar| buscar.cliente == cliente && buscar.libro == libro){
        return true;
      }
      false
    }



    pub fn devolver_libro(&mut self,libro:&Libro,cliente:&Cliente,fecha:Fecha){
        if let Some(encontre) = self.prestamos.iter_mut().find(|p| p.libro == *libro && p.cliente == *cliente){
            encontre.estado = "devuelto".to_string();
            encontre.fecha_dev = Some(fecha);
        }
    }

}


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
          let mut aux = self.clone();
          for i in 0..dias {
            aux.dia -= 1;
            if !self.es_fecha_valida() {
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
                  self.dia = 28;
                  if !self.es_fecha_valida(){
                    self.dia = 29;
                  }
                }
              }
            }
          }
        
      }
    }


    fn es_mayor(&self,f:Fecha) -> bool {
      let mut aux:bool = false;
      if f.anio > self.anio {
        aux = true;
      } else {
        if f.anio == self.anio {
          if f.mes > self.mes {
            aux = true;
          } else {
            if f.mes == self.mes {
              if f.dia > self.dia {
                aux = true;
              } 
            }
          }
        }
      }
      return aux;
    }  
  
  }
   
}