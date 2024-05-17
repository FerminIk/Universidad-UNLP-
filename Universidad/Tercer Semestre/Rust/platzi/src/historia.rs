//nos traemos las librerias
use csv::{ReaderBuilder,StringRecord};
use std::collections::{HashMap};
use std::fs;

//el path
const FILENAME: &str = "history.csv";

#[derive(Debug)]
// forma TIPO,TAG,TEXTO,VIDA
struct DatoHistoria {
  //Creamos una estructura de datos (como un registro)
  tipo_dato:String,
  tag:String,
  texto:String,
  vida:i32,
  opciones: Vec<DatoHistoria>,
}

impl DatoHistoria {
  fn new(row: StringRecord) -> DatoHistoria {
     let vida = row.get(3).unwrap().parse::<i32>().unwrap_or(0);

    DatoHistoria{
      tipo_dato:row.get(0).expect("error").trim().to_string(),
      tag:row.get(1).expect("error").trim().to_string(),
      texto: row.get(2).expect("error").trim().to_string(),
      vida:vida,
      opciones:vec![],
    }
  }
}

pub fn historia(){
  let mut vida = 100;
  let mut tag_actual= "INICIO".to_string();
  let mut last_record:String = String::new();
  //creamos un vector de registros
  let mut datos_historia:HashMap<String, DatoHistoria> = HashMap::new();
  //cargamos el archivo
  let content = fs::read_to_string(FILENAME).expect("error");
  //creamos un  reader de filas
  let  mut rdr = ReaderBuilder::new().delimiter(b';').has_headers(false).from_reader(content.as_bytes());


  //for each
  for result in rdr.records() {
    let result = result.expect("0");
    //cargamos el registro 
    let dato = DatoHistoria::new(result);

    if dato.tipo_dato ==  "SITUACION" {
      let record_tag = dato.tag.clone();

      datos_historia.insert(record_tag.clone(),dato);
      last_record = record_tag;
    } else if dato.tipo_dato == "OPCION" {
      if let Some(data) = datos_historia.get_mut(&last_record){
        (*data).opciones.push(dato);
      }
    }
  }
  
  //game loop
  loop {
    println!("Tienes {} de vida", vida);
    
    if let Some(data) = datos_historia.get(&tag_actual){
      println!("{}",data.texto);

      for (indice, option) in data.opciones.iter().enumerate() {
        println!("[{}] {}",indice,option.texto);
      }

      let mut seleccion = String::new();
      std::io::stdin().read_line(&mut seleccion).expect("error");
      let seleccion = seleccion.trim().parse::<usize>().expect("error");

      if seleccion < data.opciones.len(){
        tag_actual = data.opciones[seleccion].tag.clone();
      } else {
        println!("Comando no valido");
      }
      vida += data.vida;
      println!("");
    } else {
      break;
    }
    if vida <= 0 {
      println!("Has perdido");
      break;
    }
  }

}