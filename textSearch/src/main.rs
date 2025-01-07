use std::fs; // Para leer archivos
use std::io; // Para manejar entradas del usuario
use regex::Regex; // Para usar expresiones regulares

fn main() {
    println!("Buscador de palabras clave en un archivo de texto.");

    // Solicitar al usuario el nombre del archivo
    println!("Ingresa el nombre del archivo (ejemplo: texto_ejemplo.txt):");
    let mut archivo = String::new();
    io::stdin()
        .read_line(&mut archivo)
        .expect("Error al leer la entrada.");
    let archivo = archivo.trim();

    // Leer el contenido del archivo
    let contenido = match fs::read_to_string(archivo) {
        Ok(texto) => texto,
        Err(_) => {
            println!("No se pudo leer el archivo. Verifica que el nombre es correcto.");
            return;
        }
    };

    // Solicitar la palabra clave al usuario
    println!("Ingresa la palabra clave que deseas buscar:");
    let mut palabra_clave = String::new();
    io::stdin()
        .read_line(&mut palabra_clave)
        .expect("Error al leer la entrada.");
    let palabra_clave = palabra_clave.trim();

    // Crear una expresión regular para buscar la palabra clave
    let re = Regex::new(&format!(r"(?i)\b{}\b", palabra_clave)).expect("Expresión regular inválida.");

    // Buscar y mostrar las líneas que contienen la palabra clave
    let mut encontrado = false;
    for (num_linea, linea) in contenido.lines().enumerate() {
        if re.is_match(linea) {
            println!("Línea {}: {}", num_linea + 1, linea);
            encontrado = true;
        }
    }

    if !encontrado {
        println!("No se encontró la palabra clave en el archivo.");
    }
}
