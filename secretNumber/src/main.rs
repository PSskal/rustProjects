use std::io; // Para manejar entradas del usuario
use rand::Rng; // Para generar números aleatorios

fn main() {
    println!("¡Bienvenido al juego de adivinanzas!");
    println!("Estoy pensando en un número entre 1 y 100.");
    println!("¡Intenta adivinarlo!");

    // Genera un número aleatorio entre 1 y 100
    let numero_secreto = rand::thread_rng().gen_range(1..=100);

    loop {
        // Solicita un número al usuario
        println!("Por favor, ingresa tu número:");
        let mut intento = String::new();

        io::stdin()
            .read_line(&mut intento)
            .expect("Error al leer la entrada.");

        // Convierte la entrada en un número
        let intento: u32 = match intento.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, ingresa un número válido.");
                continue; // Reinicia el bucle si no es un número
            }
        };

        // Compara el intento con el número secreto
        if intento < numero_secreto {
            println!("Demasiado bajo.");
        } else if intento > numero_secreto {
            println!("Demasiado alto.");
        } else {
            println!("¡Felicidades! Adivinaste el número: {}", numero_secreto);
            break; // Sale del bucle si el usuario acierta
        }
    }
}
