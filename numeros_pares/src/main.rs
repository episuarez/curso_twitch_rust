use std::io::Write;
use std::cmp::Ordering;

fn main() {

    let mut entrada1 = String::new();
    let mut entrada2 = String::new();

    print!("Dime un numero: ");
    std::io::stdout().flush().expect("Error: Al vaciar el buffer");
    std::io::stdin().read_line(&mut entrada1).expect("Error: No puedo leer bien el dato.");

    print!("Dime un numero: ");
    std::io::stdout().flush().expect("Error: Al vaciar el buffer");
    std::io::stdin().read_line(&mut entrada2).expect("Error: No puedo leer bien el dato.");

    entrada1 = entrada1.replace("\r\n", "");
    entrada2 = entrada2.replace("\r\n", "");

    let mut numero1: u16 = entrada1.parse::<u16>().unwrap();
    let mut numero2: u16 = entrada2.parse::<u16>().unwrap();

    let mut contador = 0;

    if numero1 > numero2 {
        let numero_temporal = numero1;

        numero1 = numero2;
        numero2 = numero_temporal;
    }
    
    for numero in numero1..=numero2 {
        match (numero % 2).cmp(&0) {
            Ordering::Equal => contador += 1, _ => ()
        }
    }

    println!("{}", contador);
    
    let pares = (numero1..=numero2).filter(|numero| numero % 2 != 0).count();
    print!("{}", pares);
}
