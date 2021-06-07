use std::io::Write;

fn main() {

    print!("Dime tu nombre: ");
    std::io::stdout().flush().expect("error");
    let mut nombre = String::new();
    std::io::stdin().read_line(&mut nombre).expect("Error: Al leer de consola.");

    print!("Dime tu edad: ");
    std::io::stdout().flush().expect("error");
    let mut edad = String::new();
    std::io::stdin().read_line(&mut edad).expect("Error: Al leer de consola.");
    
    nombre = nombre.replace("\r\n", "");
    edad = edad.replace("\r\n", "");

    print!("Hola {}, tienes {} años.", nombre, edad);

}