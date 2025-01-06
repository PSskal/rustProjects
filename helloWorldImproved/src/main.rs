use std::env;

fn main() {
    // Obtén los argumentos pasados al programa
    let args: Vec<String> = env::args().collect();

    // Verifica si se proporcionó un argumento
    if args.len() > 1 {
        // Toma el primer argumento después del nombre del programa
        let name = &args[1];
        println!("hello, {}!", name);
    } else {
        println!("please, what is your name.");
    }
}
