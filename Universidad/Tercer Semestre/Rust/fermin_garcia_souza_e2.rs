/* biblio ejercicio 10:
implemente un método en la biblioteca con la siguiente firma: 
emails_de_clientes_que_tuvieron_mora()->Vec<String>
Este método retorna los 
emails de los clientes que devolvieron algún prestamo luego de la fecha 
de vencimiento.
No puede haber emails repetidos
11:07
https://play.rust-lang.org/?version=stable&mode=debug&edition=2021 */

//El metodo entregable esta abajo de todo

pub fn ej10() {
    #[derive(Clone)]
    struct Bibloteca{
      nombre:String,
      dir:String,
      dis_libro:Vec<Libros_Dis>,
      prestamos:Vec<Prestamo>
    }
  #[derive(Clone)]
    struct Prestamo{
      libro:Libro,
      cliente:Cliente,
      fecha_venc:Fecha,
      fecha_dev:Option<Fecha>,
      estado:String
    }
  #[derive(Clone)]
    struct Cliente{
      nombre:String,
      telefono:String,
      dir_electronica:String
    }
  #[derive(Clone)]
    struct Libros_Dis{
      ejemplares:u32,
      libro:Libro
    }
  
  #[derive(Clone)]
    struct Libro{
      titulo:String,
      autor:String,
      num_pag:i32,
      genero:Genero
    }
  
  #[derive(Copy, Clone)]
    enum Genero{
      novela,
      infantil,
      tecnico,
      otros
    }
  
    #[derive(Debug,Clone)]
    struct Fecha{
      dia: u8,
      mes:u8,
      anio:i32
    }
  
    impl Bibloteca{
      fn copias_disponibles(&self, libro_buscar:Libro)->Option<&u32> {
        if !self.dis_libro.is_empty() {
          let iterador = self.dis_libro.iter();
          for aux_libro in iterador {
            if (aux_libro.libro.titulo == libro_buscar.titulo) && (aux_libro.libro.autor == libro_buscar.autor){
              return Some(&aux_libro.ejemplares);
            }
          }
        }
        return None;
      }
  
      fn tomar_copia(&mut self, libro_buscar:Libro) {
        let aux = self.copias_disponibles(libro_buscar);
        if aux.is_some() {
          aux.unwrap() - 1;
        }
      }
  
      fn devolver_copia(&mut self, libro_buscar:Libro) {
        let aux = self.copias_disponibles(libro_buscar);
        if aux.is_some() {
          aux.unwrap() + 1;
        }
      }
  
      fn cliente_responsable(&mut self,hoy:Fecha,cliente:Cliente) {
        if !self.prestamos.is_empty() {
          for i in 0..self.prestamos.len() {
            if (self.prestamos[i].cliente.nombre == cliente.nombre) && (self.prestamos[i].cliente.telefono == cliente.telefono) && (self.prestamos[i].estado == "en préstamo") {
              self.prestamos[i].fecha_dev = Some(hoy.clone());
              self.devolver_copia(self.prestamos[i].libro.clone()); 
              self.prestamos[i].estado = "devuelto".to_string();
            }
          }
        }
      }
  
      fn crear_prestamo(&mut self,fecha_venc:Fecha,cliente:Cliente,libro:Libro){
        let estado = "en préstamo".to_string();
        let fecha_dev:Option<Fecha> = None;
        let aux = Prestamo { libro, cliente, fecha_venc, fecha_dev, estado };
        self.prestamos.push(aux);
      }
  
      fn realizar_prestamo(&mut self,mut hoy:Fecha,cliente:Cliente,libro:Libro) ->bool {
        if !self.prestamos.is_empty() {
          let mut aux:u8 = 0;
          for i in 0..self.prestamos.len() {
            if (self.prestamos[i].cliente.nombre == cliente.nombre) && (self.prestamos[i].cliente.telefono == cliente.telefono){
              aux += 1;
            }
          }
          if aux < 5 {
            let copias = self.copias_disponibles(libro.clone());
            if copias.is_some() {
              let copias = copias.unwrap();
              let cero:u32 = 0;
              if copias > &cero {
                copias - 1;
                hoy.sumar_dias(14);
                self.crear_prestamo(hoy,cliente,libro);
                return true;
              }
            }
          }
        }
        return false;
      }
  
      fn devolver_prestamo(&self,libro:Libro,cliente:Cliente,) -> Option<Prestamo> {
        if !self.prestamos.is_empty() {
          for i in 0..self.prestamos.len() {
            if (self.prestamos[i].cliente.nombre == cliente.nombre) && (self.prestamos[i].cliente.telefono == cliente.telefono) && (self.prestamos[i].libro.titulo == libro.titulo) {
              return Some(self.prestamos[i].clone());
            }
          }
        }
        return None;
      }
  
      fn prestamos_por_vencer(&self,mut hoy:Fecha,dias:u8) ->Vec<Prestamo> {
        let mut aux:Vec<Prestamo> = Vec::new();
        hoy.sumar_dias(dias);
        if !self.prestamos.is_empty() {
          for i in 0..self.prestamos.len() {
            if self.prestamos[i].fecha_venc.es_mayor(hoy.clone()) && (self.prestamos[i].estado == "en préstamo") {
              aux.push(self.prestamos[i].clone());
            }
          }
        }
        return aux;
      }
  
      fn prestamos_vencido(&self,hoy:Fecha) ->Vec<Prestamo> {
        let mut aux:Vec<Prestamo> = Vec::new();
        if !self.prestamos.is_empty() {
          for i in 0..self.prestamos.len() {
            if self.prestamos[i].fecha_venc.es_mayor(hoy.clone()) && (self.prestamos[i].estado == "en préstamo") {
              aux.push(self.prestamos[i].clone());
            }
          }
        }
        return aux;
      }
  
      fn devolver_libro(&mut self,libro:Libro,cliente:Cliente,hoy:Fecha) ->bool {
        if !self.prestamos.is_empty() {
          for i in 0..self.prestamos.len() {
            if (self.prestamos[i].cliente.nombre == cliente.nombre) && (self.prestamos[i].cliente.telefono == cliente.telefono) && (self.prestamos[i].libro.titulo == libro.titulo) {
              self.prestamos[i].fecha_dev = Some(hoy.clone());
              self.devolver_copia(self.prestamos[i].libro.clone()); 
              self.prestamos[i].estado = "devuelto".to_string();
              return true;
            }
          }
        }
        return false;
      }

      // ENTREGABLE!
      fn emails_de_clientes_que_tuvieron_mora(&self)->Vec<String> {
        let mut result:Vec<String> = Vec::new();
        //Si no tengo la fecha actual es imposible saber cuando vencio
        let hoy = Fecha::new(23,5,2023);
        if !self.prestamos.is_empty() {
            let aux = self.prestamos_vencido(hoy);
            let iterador = aux.iter();
            for prestamos_venc in iterador {
              if prestamos_venc.fecha_dev.is_some() {
                let fecha_de_devolucion = prestamos_venc.fecha_dev.clone().unwrap();
                if prestamos_venc.fecha_venc.es_mayor(fecha_de_devolucion){
                  let mut repetido:bool = false;
                    for i in 0..result.len() {
                      if prestamos_venc.cliente.dir_electronica == result[i] {
                        repetido = true;
                      }
                    }
                    if (!repetido) {
                     result.push(prestamos_venc.cliente.dir_electronica.clone());
                    }
                }
              }
            }
        }
        return result;
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
        self.dia = self.dia+dias;
        if !self.es_fecha_valida() {
          self.dia = self.dia-dias;
          for i in 0..dias {
            self.dia += 1;
            if !self.es_fecha_valida() {
              self.mes += 1;
              self.dia = 1;
              if !self.es_fecha_valida(){
                self.anio += 1;
                self.mes = 1;
              }
            }
          }
        }
      }
  
      fn restar_dias(&mut self, dias: u8) {
        self.dia = self.dia-dias;
        if !self.es_fecha_valida() {
          self.dia = self.dia+dias;
          for i in 0..dias {
            self.dia -= 1;
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