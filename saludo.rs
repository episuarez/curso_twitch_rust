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
    
    nombre = nombre.trim().to_string();
    edad = edad.trim().to_string();

    print!("Hola {}, tienes {} años.", nombre, edad);
}