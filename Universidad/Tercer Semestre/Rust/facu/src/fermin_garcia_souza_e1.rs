use random_number::random;

//funciones
//la funcion funciona en todo el rango u16 (0..65535), pude haberlo hecho con la variable u128, pero me parecia una exagueracion.
fn get_par_random(v:&[u16]) -> i64{
    let pos_random:usize = random!(..=v.len()-1);
    let mut aux:i64 = v[pos_random] as i64;
    if aux % 2 != 0 {
        aux = -1
    }
    return aux;
} 

//programa principal de ejemplo
fn main() {
  //Este arreglo generico que puede ser modificado agregandole o quitandole las celdas que vea necesario (Respetando en este caso el rango u16).
  let mut v:[u16;15] = [0;15];
  for i in 0..v.len() {
    v[i] = i as u16;
  }
  println!("el numero elegido al azar fue {} ¡felicidades!",get_par_random(&mut v));
}