use std::io;

fn main() {
    println!("Calculadora Básica en Rust");
    println!("Selecciona una operación:");
    println!("1: Suma");
    println!("2: Resta");
    println!("3: Multiplicación");
    println!("4: División");

    // Lee la opción del usuario
    let mut opcion = String::new();
    io::stdin().read_line(&mut opcion).expect("Error al leer la entrada.");
    let opcion: u32 = match opcion.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor, ingresa un número válido.");
            return;
        }
    };

    // Pide el primer número
    println!("Ingresa el primer número:");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Error al leer la entrada.");
    let num1: f64 = match num1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor, ingresa un número válido.");
            return;
        }
    };

    // Pide el segundo número
    println!("Ingresa el segundo número:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Error al leer la entrada.");
    let num2: f64 = match num2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor, ingresa un número válido.");
            return;
        }
    };

    // Realiza la operación seleccionada
    match opcion {
        1 => println!("El resultado de la suma es: {}", num1 + num2),
        2 => println!("El resultado de la resta es: {}", num1 - num2),
        3 => println!("El resultado de la multiplicación es: {}", num1 * num2),
        4 => {
            if num2 != 0.0 {
                println!("El resultado de la división es: {}", num1 / num2);
            } else {
                println!("Error: No se puede dividir entre cero.");
            }
        }
        _ => println!("Opción no válida. Por favor, selecciona entre 1 y 4."),
    }
}
