fn main() {
    // Enteros
    let edad: i32 = 20;
    let edad_u8: u8 = edad as u8;

    println!("Edad i32: {}", edad);
    println!("Edad u8: {}", edad_u8);

    // Decimales
    let peso: f64 = 70.5;
    let peso_f32: f32 = peso as f32;

    println!("Peso f64: {}", peso);
    println!("Peso f32: {}", peso_f32);

    // Booleano
    let activo: bool = true;

    if activo {
        println!("Usuario activo");
    } else {
        println!("Usuario inactivo");
    }

    // char (Unicode)
    let letra: char = 'A';
    let emoji: char = '🔥';

    println!("Letra: {}", letra);
    println!("Emoji: {}", emoji);

    // &str vs String
    let nombre_str: &str = "Iver";
    let nombre_string: String = String::from("Iver Padilla");

    println!("Nombre (&str): {}", nombre_str);
    println!("Nombre (String): {}", nombre_string);

    // Casting explícito
    let puntos: i32 = 10;
    let bonus: f64 = 2.5;

    let total = puntos as f64 + bonus;
    println!("Total puntos: {}", total);
}
