fn saludar(nombre: &str) {
    println!("Hola, {}", nombre);
}

fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

fn clasificar_edad(edad: i32) -> &'static str {
    match edad {
        0..=12 => "Niño",
        13..=17 => "Adolescente",
        18..=64 => "Adulto",
        _ => "Adulto mayor",
    }
}

fn es_par(numero: i32) -> bool {
    numero % 2 == 0
}

fn main() {
    saludar("Iver");

    let resultado = sumar(10, 5);
    println!("Suma: {}", resultado);

    let edad = 17;
    let categoria = clasificar_edad(edad);
    println!("Edad: {}, categoría: {}", edad, categoria);

    let numero = 10;
    println!("{} es par? {}", numero, es_par(numero));
}
