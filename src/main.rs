use manejo_inputs;
fn main() {
    manejo_inputs::mostrar_menu();
    //leyendo la operacion que el usuario quiere ejecutar 
    let opcion: i32 = manejo_inputs::obtener_input("Escribe el numero de la operacion que quieres realizar");

    //leyendo los digitos a utilizar. 
    let a: i32 = manejo_inputs::obtener_input("Primer digito");
    let b: i32 = manejo_inputs::obtener_input("Segundo digito");

    let resultado = manejo_inputs::obtener_resultado(opcion, a, b);
        
    //imprimiendo el resutado en pantalla 
    println!("Resultado: {}" , resultado);
}
