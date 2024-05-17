pub fn clase8(){
  let mut buf  = String::new();
  println!("Elige una de las pildoras, pildora roja o pildora azul");
  std::io::stdin().read_line(&mut buf).expect("Error");
  buf = buf.trim().to_string();
  if buf == "roja" {
    println!("con la pildora {} sales de la matrix",buf);
  } else {
    if buf == "azul" {
      println!("con la pildora {} entras a la matrix",buf);
    }else{
      println!("que elegia xd");
    }
  }
}