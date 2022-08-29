//fn main() {
//   let nombre: &str = "Jose";
//  let edad: u8 = 24;
//  println!("Hola soy {} y tengo {} anos", nombre, edad);
//}

fn main() {
    println!("Porfavor introduce tu nombre: ");

    let mut nombre : String = String::new();
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();


    //Obtener la edad de la consola

    println!("Porfavor introduce tu edad: ");

    let mut edad : String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();

    //Convertir esa edad a un numero

    let edad_int : u8 = edad.trim().parse().unwrap();

    println!("Hola, bienvenido o bienvenida {} de {} anos", nombre, edad_int);


}