use std::io;

enum Conversion {
    MetersToFeet,
    FeetToMeters,
    CelsiusToFahrenheit,
    FahrenheitToCelsius,
}

impl Conversion {
    fn execute(&self, value: f64) -> (f64, &'static str) {
        match self {
            Conversion::MetersToFeet => (value * 3.28084, "pies"),
            Conversion::FeetToMeters => (value / 3.28084, "metros"),
            Conversion::CelsiusToFahrenheit => ((value * 9.0 / 5.0) + 32.0, "°F"),
            Conversion::FahrenheitToCelsius => ((value - 32.0) * 5.0 / 9.0, "°C"),
        }
    }
}

fn main() {
    loop {
        println!("\nConversor de unidades");
        println!("Opciones disponibles:");
        println!("1. Metros a pies");
        println!("2. Pies a metros");
        println!("3. Celsius a Fahrenheit");
        println!("4. Fahrenheit a Celsius");
        println!("5. Salir");

        // Leer la opción del usuario
        let mut opcion = String::new();
        println!("Ingresa la opcion:");
        io::stdin()
            .read_line(&mut opcion)
            .expect("Error al leer la entrada.");
        let opcion = match opcion.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, ingresa un número válido.");
                continue;
            }
        };

        // Verificar si el usuario desea salir
        if opcion == 5 {
            println!("Gracias por usar el conversor. ¡Hasta pronto!");
            break;
        }

        // Seleccionar la conversión
        let conversion = match opcion {
            1 => Conversion::MetersToFeet,
            2 => Conversion::FeetToMeters,
            3 => Conversion::CelsiusToFahrenheit,
            4 => Conversion::FahrenheitToCelsius,
            _ => {
                println!("Opción no válida. Intenta nuevamente.");
                continue;
            }
        };

        // Leer el valor a convertir
        let mut valor = String::new();
        println!("Ingresa el valor que deseas convertir:");
        io::stdin()
            .read_line(&mut valor)
            .expect("Error al leer la entrada.");
        let valor = match valor.trim().parse::<f64>() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, ingresa un número válido.");
                continue;
            }
        };

        // Realizar la conversión
        let (resultado, unidad) = conversion.execute(valor);
        println!("El resultado de la conversión es: {:.2} {}", resultado, unidad);
    }
}
