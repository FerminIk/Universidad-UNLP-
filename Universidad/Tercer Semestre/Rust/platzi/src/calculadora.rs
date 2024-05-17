    
mod matrix;
use regex::Regex;

pub fn calculadora() {
// regex
let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
let re_res = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
let re_div = Regex::new(r"(\d+)\s?/\s?(\d+)").unwrap();
let re_mult = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();

// Datos del usuario
println!("Por favor introduce tu expresion: ");
let mut expression = String::new();
std::io::stdin().read_line(&mut expression).unwrap();

//multiplicacion
loop {
    // Operaciones
    let caps = re_mult.captures(expression.as_str());

    if caps.is_none(){
        break;
    }

    let caps = caps.unwrap();

    let caps_expression = caps.get(0).unwrap().as_str();
    let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();
    let mult = left_value * right_value;
    
    expression = expression.replace(caps_expression, &mult.to_string());
}
//division
loop {
    // Operaciones
    let caps = re_div.captures(expression.as_str());

    if caps.is_none(){
        break;
    }

    let caps = caps.unwrap();

    let caps_expression = caps.get(0).unwrap().as_str();
    let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();
    let div = left_value / right_value;
    
    expression = expression.replace(caps_expression, &div.to_string());
}
//resta
loop {
    // Operaciones
    let caps = re_res.captures(expression.as_str());

    if caps.is_none(){
        break;
    }

    let caps = caps.unwrap();

    let caps_expression = caps.get(0).unwrap().as_str();
    let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();
    let res = left_value - right_value;
    
    expression = expression.replace(caps_expression, &res.to_string());
}

//suma
loop {
    // Operaciones
    let caps = re_add.captures(expression.as_str());

    if caps.is_none(){
        break;
    }

    let caps = caps.unwrap();

    let caps_expression = caps.get(0).unwrap().as_str();
    let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();
    let addition = left_value + right_value;
    
    expression = expression.replace(caps_expression, &addition.to_string());
}
// Resultado
println!("resultado: {} ",expression);
}