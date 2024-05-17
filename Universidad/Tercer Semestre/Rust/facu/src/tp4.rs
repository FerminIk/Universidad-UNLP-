pub fn ej1(){

  impl PrimI32 for i32{
    fn primo(&self) -> bool{
      let mut result = false;
      let mitad;
      if self % 2 == 0 {
        mitad= self / 2;
      } else {
        mitad= self+1 / 2;
      }
      if mitad > 2 {
        let  mut aux = false;
        for i in 2..mitad {
          if (self % i as i32 == 0){
            aux = true;
          }
        }
        if aux == false {
          result = true;
        }
      } else {
        if (self == &1) || (self == &2) {
          result = true;
        } else  {
          result = false;
        }
      }
      return result;
    }
  }

  pub trait PrimI32{
    fn primo(&self) -> bool;
  }

  fn cantidad_primos(vector :Vec<i32>) ->i32 {
    let iterador = vector.iter();
    let mut cant = 0;
    for num in iterador{
      if (num.primo()) {
        cant += 1;
      }
    }
    return cant;
  }

}

pub fn ej2() {
  struct Persona<'a>{
  nombre:&'a str,
  apellido:&'a str,
  direccion:&'a str,
  ciudad:&'a str,
  salario:f64,
  edad:u8,
  }
}