use std::fs::File;
use std::io::copy;
use std::io::{self};
use reqwest::blocking::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Descargador de Archivos");

    // Solicitar URL al usuario
    println!("Ingresa la URL del archivo a descargar:");
    let mut url = String::new();
    io::stdin()
        .read_line(&mut url)
        .expect("Error al leer la URL");
    let url = url.trim();

    // Solicitar nombre de archivo local
    println!("Ingresa el nombre con el que deseas guardar el archivo:");
    let mut filename = String::new();
    io::stdin()
        .read_line(&mut filename)
        .expect("Error al leer el nombre del archivo");
    let filename = filename.trim();

    // Realizar la descarga
    println!("Descargando desde: {}", url);
    let response = Client::new().get(url).send()?.error_for_status()?;
    let mut file = File::create(filename)?;

    // Guardar el contenido en un archivo
    copy(&mut response.bytes()?.as_ref(), &mut file)?;

    println!("Archivo guardado como: {}", filename);
    Ok(())
}
