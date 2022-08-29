//VARIABLES

//fn main() {
//   let nombre: &str = "Jose";
//  let edad: u8 = 24;
//  println!("Hola soy {} y tengo {} anos", nombre, edad);
//}



//RECIBIR DATOS

// fn main() {
//     println!("Porfavor introduce tu nombre: ");

//     let mut nombre : String = String::new();
//     std::io::stdin().read_line(&mut nombre).unwrap();
//     nombre = nombre.trim().to_string();

//     //Obtener la edad de la consola

//     println!("Porfavor introduce tu edad: ");

//     let mut edad : String = String::new();
//     std::io::stdin().read_line(&mut edad).unwrap();

//     //Convertir esa edad a un numero
//     let edad_int : u8 = edad.trim().parse().unwrap();

//     println!("Hola, bienvenido o bienvenida {} de {} anos", nombre, edad_int);
// }


//CONDICIONALES

// fn main() {
//     // Obtener la edad de la consola
//     println!("Por favor introduce tu edad: ");
//     let mut edad : String = String::new();
//     std::io::stdin().read_line(&mut edad).unwrap();

//     // Convertir esa edad a un numero
//     let edad_int : u8 = edad.trim().parse().unwrap();

//     if edad_int >= 18 && edad_int != 30 {
//         println!("Puedes entrar a la discoteca");
//     } else if edad_int == 30 {
//         println!("No admitimos personas de exactamente 30 años");
//     }
//     else {
//         println!("Eres menor de edad todavia");
//     } 

//     println!("Tienes {} años", edad_int);
// }



// CICLO LOOP

// fn main() {
//     // Dos numeros que vamos a sumar
//     let numero_1 = 120;
//     let numero_2 = 321;

//     let suma = numero_1 + numero_2;

//     loop {
//         // Mostrar los dos numeros en pantalla
//         println!("Por favor escribir la suma de {} y {}: ", numero_1, numero_2);

//         // Obtener del usuario el numero que representa la suma
//         let mut suma_usuario = String::new();
//         std::io::stdin().read_line(&mut suma_usuario).unwrap();

//         let suma_usuario_int : i32 = suma_usuario.trim().parse().unwrap();

//         if suma_usuario_int == suma {
//             println!("Lo has hecho muy bien, el resultado {} es correcto", suma);
//             break;
//         } else {
//             println!("El resultado {} no es correcto por favor intentalo de nuevo", suma_usuario_int);
//         }
//     }
// }



// ARRAYS & FOR

// fn main() {

//     let mut nombres : Vec<String> = Vec::new();

//     for i in 0..3 {
//         println!("Por favor introduce un nombre: ");
//         let mut nombre = String::new();
//         std::io::stdin().read_line(&mut nombre).unwrap();
    
//         nombres.push(nombre);
//     } 

//     for nombre in nombres {
//         println!("El nombre es: {}", nombre);
//     }

//     let hola = ["H", "O", "L", "A"];

//     println!("{}", hola[0]);
//     println!("{}", hola[1]);
//     println!("{}", hola[2]);
//     println!("{}", hola[3]);

//     // println!("{}", hola[4]);

// }



//FUNCIONES

// fn sumar_uno(numero_a_sumar: i32) -> i32 {
//     let numero_final = numero_a_sumar + 1;
//     println!("{}", numero_final);


//     return numero_final;
// }



// fn main() {
//     println!("Hola Platzi");

//     let diez_mas_uno = sumar_uno(10);
//     sumar_uno(diez_mas_uno);
//     sumar_uno(12);
//     sumar_uno(13);
//     sumar_uno(14);


// }

