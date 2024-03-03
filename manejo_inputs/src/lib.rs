use std::io::stdin;
use operaciones;

//mostramos el menu de operaciones 
pub fn mostrar_menu() {
    println!("OPERACIONES DISPONIBLES");
    println!("-----------------------");
    println!("1. SUMA.");
    println!("2. RESTA.");
    println!("3. MULTIPLICACION.");
    println!("4. DIVISION.");
}

pub fn obtener_input(label: &str) -> i32 {
    println!("{}", label);

    // Obteniendo input 
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    // limpiando el input de los caracteres de retorno 
    input_string = input_string.replace("\n", "");
    input_string = input_string.replace("\r", "");

    // convirtiendo el input a un entero 
    let numero: i32 = input_string.parse::<i32>().unwrap();
    numero
}

pub fn obtener_resultado(opcion: i32, a: i32, b: i32) -> i32 {
     // calculando el resultad, por facilidad, si la oprecacion elegida no es ninguna de  las del menu, entonces vamos a colocar un 0
     match opcion {
        1 => operaciones::suma(a, b)
        ,2 => operaciones::resta(a, b)
        ,3 => operaciones::multiplicacion(a, b)
        ,4 => operaciones::division(a, b)
        ,_ => 0
    }
}