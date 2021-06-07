use std::io::Write;

fn main() {
    print!("Dime un año de nacimiento: ");
    std::io::stdout().flush().expect("Error: Al vaciar el buffer.");
    let mut anno = String::new();
    std::io::stdin().read_line(&mut anno).expect("Error: Al no poder leer la linea");

    anno = anno.replace("\r\n", "");
    let anno: u16 = anno.parse::<u16>().unwrap();

    if 2021 - anno > 17 {
        print!("Eres mayor de edad");
    } else {
        print!("No eres mayor de edad");
    }
}