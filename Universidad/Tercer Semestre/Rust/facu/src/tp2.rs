pub fn ej1() {
  fn es_par(x:i32) -> bool {
    return if x%2 == 0 {true} else {false};
  }

  let num:i32 = 1;
  println!("{}",es_par(num));
}

pub fn ej2() {
  fn es_primo(x:i32) -> bool {
    //apartado por si se introducen numeros negativos o el 2
    if x <= 1 {
        return false;
    } else if x == 2 {
      return true;
    }
    //lectura generica para numeros no negativos ni el 2
    for i in 2..x {
      if x%i == 0 {
        return false;
      }
    }
   true
  }

  let num:i32 = 3;
  println!("{} --- true es primo",es_primo(num));
}

pub fn ej3() {
  fn suma_pares(v: &mut [i32]) -> i32 {
    let mut total:i32 = 0;
    for i in 0..v.len() {
      if v[i] % 2 == 0 {
        total += v[i];
      }
    }
    return total
  }
  let mut v:[i32;5] = [6,2,3,4,5];
  println!("la suma de los numeros pares es de: {}",suma_pares(&mut v));
}

pub fn ej4() {
  fn cantidad_impares(v: &mut [i32]) -> i32 {
    let mut total:i32 = 0;
    for i in 0..v.len() {
      if !(v[i] % 2 == 0) {
        total += 1;
      }
    }
    return total
  }
  let mut v:[i32;5] = [6,2,3,4,5];
  println!("la cantidad de numeros impares, fue de: {}",cantidad_impares(&mut v));
}

pub fn ej5() {
  fn duplicar_valores(v: &mut [f64;5]) -> [f64;5] {
    let mut aux:[f64;5] = [0.0,0.0,0.0,0.0,0.0];
    for i in 0..v.len() {
      aux[i] = v[i] * 2.0;
    }
    return aux;
  }
  let mut v:[f64;5] = [6.01,2.22,3.15,4.0,5.0];
  println!("la cantidad de numeros impares, fue de: {:?}",duplicar_valores(&mut v));
}

pub fn ej6() {
  fn longitud_de_cadenas(v: [String;5]) -> [i32;5] {
    let mut aux:[i32;5] = [0,0,0,0,0];
    for i in 0..v.len() {
      aux[i] = v[i].len() as i32;
    }
    return aux;
  }
   let arreglo_str:[String;5] = ["Frutas".to_string(), "autos".to_string(),"Argentina".to_string(),"licencia".to_string(),"Facultad".to_string()];
    let cantidad_en_arreglo:[i32;5] = longitud_de_cadenas(arreglo_str);
    for i in 0..5{
        println!("En la posicion {} hay una cadena de {} caracteres", i, cantidad_en_arreglo[i]);
    }
}

pub fn ej7() {
  fn mayores(v: [i32;5], l: i32) -> i32 {
    let mut aux:i32 = 0;
    for i in 0..5 {
      if v[i] >= l {
        aux += 1;
      }
    }
    aux
  }
  let arreglo:[i32;5] = [1,2,3,4,5];
  const limite:i32 = 3;
  let cant: i32 = mayores(arreglo,limite);
  println!("La cantidad de numeros que superaron el limite fueron de {}",cant);
}

pub fn ej8() {
  fn sumar_arreglos(v1: [f64;5], v2:[f64;5]) -> [f64;5] {
    let mut aux:[f64;5] = [0.0,0.0,0.0,0.0,0.0];
    for i in 0..v1.len() {
      aux[i] = v1[i] + v2[i];
    }
    return aux;
  }
  let v1:[f64;5] = [6.01,2.22,3.15,4.0,5.0];
  let v2:[f64;5] = [1.01,2.22,3.15,4.0,5.0];
  println!("la cantidad de numeros impares, fue de: {:?}",sumar_arreglos(v1,v2));
}

pub fn ej9() {
  fn cantidad_en_rango(v: [i32;5], mut i: i32, mut s: i32) -> i32 {
    let mut aux:i32 = 0;
    if i < 0 {
      println!("El numero {} esta por fuera del rango inferior del arreglo, lo remplazamos por 0",i);
      i = 0;
    } if s > v.len() as i32 {
      println!("El numero {} esta por fuera del rango superior del arreglo, lo remplazamos por {}",s,v.len());
      s = v.len() as i32;
    }
    for i in i..s+1 {
      aux +=1;
    }
    return aux;
  }
  let v:[i32;5] = [6,2,3,4,5];
  let inf:i32 = -1;
  let sup:i32 = 3;
  println!("la cantidad de numeros en ese rango es de: {}",cantidad_en_rango(v,inf,sup));
  }

  pub fn ej10() {
    fn cantidad_de_cadenas_mayor_a( v: [String;5], l: i32) ->i32 {
      let mut aux:i32 =0;
      for i in 0..v.len() {
        if v[i].len() as i32 >= l {
          aux += 1;
        }
      }
      aux
    }

    let v:[String;5] =  ["a".to_string(),"b".to_string(),"hola".to_string(),"casa".to_string(),"ta".to_string()];
    let lim:i32 = 2;
    println!("La cantidad de cadenas que superaron o  igualaron {} caracteres fueron de {} ",lim,cantidad_de_cadenas_mayor_a(v,lim));
  }

  pub fn ej11() {
    fn multiplicar_valores(v:[i32;5],f:i32) -> [i32;5] {
      let mut aux:[i32;5] = [0,0,0,0,0];
      for i in 0..v.len() {
        aux[i] = v[i] * f;
      }
      aux
    }
    let v:[i32;5] = [1,2,3,4,5];  
    let lim:i32 = 2;
    println!(" {:?} ",multiplicar_valores(v,lim));
  }

  pub fn ej12() {
    fn reemplazar_pares(v:[i32;5]) -> [i32;5] {
      let mut aux:[i32;5] = [0,0,0,0,0];
      for i in 0..v.len() {
        if v[i] % 2 == 0{
          aux[i] = -1;
        } else {
          aux[i] = v[i];
        }
      }
      aux
    }
    let v:[i32;5] = [1,2,3,4,5];  
    println!(" {:?} ",reemplazar_pares(v));
  }

  pub fn ej13() {
    fn ordenar_nombres( v: [String;5]) ->[String;5] {
      let mut aux:[String;5] = v.clone();
      return aux;
      }
    let v:[String;5] =  ["a".to_string(),"b".to_string(),"hola".to_string(),"casa".to_string(),"ta".to_string()];
    println!("{:?}",ordenar_nombres(v));
  
  }

  pub fn ej14() {
    
  }