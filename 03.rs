fn main() {
    let edad = 18;

    // IF / ELSE
    let mensaje = if edad >= 18 {
        "Eres mayor de edad, puedes votar."
    } else {
        "Eres menor de edad, no puedes votar."
    };

    println!("{}", mensaje);

    // MATCH
    match edad {
        0..=12 => println!("Eres un niño"),
        13..=17 => println!("Eres adolescente"),
        18..=64 => println!("Eres adulto"),
        _ => println!("Eres adulto mayor"),
    }

    // FOR
    for i in 1..=5 {
        println!("Iteración número: {}", i);
    }
}
