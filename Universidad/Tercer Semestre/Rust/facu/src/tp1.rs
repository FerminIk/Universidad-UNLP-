pub fn ej1(){
   let flotante = 3.0;
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Error");
    let nro_leido:i32 = buf.trim().parse().expect("no es i32");
    println!("la multiplicacion de los numeros es de {}",flotante*f64::from(nro_leido));
    println!("la division de los numeros es de {}",flotante/f64::from(nro_leido));
    println!("la suma de los numeros es de {}",flotante+f64::from(nro_leido));
    println!("la resta de los numeros es de {}",flotante-f64::from(nro_leido));
    }

pub fn ej2() {
   let mut buf: String = String::new();
   println!("introduzca un numero por favor");
   std::io::stdin().read_line(&mut buf).expect("error");
   let num:u8 = buf.trim().parse().expect("error");

   println!("numero  introducido {}",num);

   println!("Hexadecimal number is: {:#02X}", num);
}

pub fn ej3() {
   let mut buf:String=String::new();
   println!("introduce un valor booleano. El que se dispone es true");
   std::io::stdin().read_line(&mut buf).expect("error");
   let input:bool=buf.trim().parse().expect("error");
   let mut valor:bool=true;
   if valor == input {
      println!("Valor sigue siendo verdadero");
   } else {
       println!("Valor ya  no vale nada :c");
   }
}

pub fn ej4() {
   let mut tupla:(String,i8,bool) = ("hola".to_string(),10,true);
   println!("Tupla 1 {}. Tupla 2 {}. Tupla 3 {}.  ",tupla.0,tupla.1,tupla.2);
   println!("{:?}",tupla);
}

pub fn ej5() {
   let mut buf:String = String::new();
   std::io::stdin().read_line(&mut buf).expect("error");
   let mut mensaje:String = "hola ".to_string().to_owned().to_ascii_uppercase();
   mensaje.push_str(&buf.to_ascii_uppercase());
   println!("{}",mensaje);
}

pub fn ej6() {
   let num:u8 = 10;
   let mut buf:String = String::new();
   std::io::stdin().read_line(&mut buf).expect("Error");
   let input:i8 = buf.trim().parse().expect("error");
   let potencia:u8 = num + input as u8;
   println!("la suma de 10 y {} es igual a : {}. Y su potencia es {}",input,potencia,potencia.pow(2));
}

pub fn ej7(){
   //constantes
   const dimF:usize = 6;
   const num:i16 = 10;
   //variables
   let mut arreglo:[i16;dimF] = [0;dimF];
   let mut buf:String = String::new();
   for i in 0..dimF {
      arreglo[i] = i as i16 + 1;
      arreglo[i] = arreglo[i]*num;
   }
   println!("{:?}",arreglo);
}

pub fn ej8(){
   let buf:String = "holo".to_string();
   let caracter:char = 'o';
   let cadena:Vec<char> = buf.chars().collect();
   let mut contador:i32 = 0;
   for i in 0..cadena.len() {
      if cadena[i] == caracter {
         contador += 1;
      }
   }
   println!("la cantidad de veces que se leyo {}, fue de {}",caracter,contador);
}

pub fn ej9(){
   let mut arreglo:[i16;5] = [1,2,3,4,5];
   let mut total:i16 = 0;
   for i in 0..5 {
      total = total + arreglo[i];
   }
   println!("El total de la suma del arreglo es de {}",total);
}

pub fn ej10(){
   let mut arreglo1:[i16;5] = [1,2,3,4,5];
   let mut arreglo2:[i16;5] = [10,9,8,7,6];
   let mut total:[i16;5] = [0;5];
   for i in 0..5 {
      total[i] = arreglo1[i] + arreglo2[i];
   }
   println!("El total de la suma del arreglo es de {:?}",total);
}

pub fn ej11(){
    let mut arreglo:[&str;5] = ["Fermin","Gonzalo","Lautaro","Monica","David"];
    let mut buf:String=String::new();
    let mut seEncuentra:bool=false;
    std::io::stdin().read_line(&mut buf).expect("error");
    let limpio:String=buf.trim().to_string();
    for i in 0..arreglo.len() {
      if arreglo[i] == limpio {
         seEncuentra = true;
      }
    }
    print!("{}",seEncuentra);
}

pub fn ej12(){
   let tupla:(String,[i32;5]) = ("hola".to_string(),[1,2,3,4,5]);
   println!("Este es el string de la tupla es {}, y esta es la suma {} ",tupla.0,tupla.1[0]+tupla.1[1]+tupla.1[2]+tupla.1[3]+tupla.1[4]);
}

