fn saludar(nombre: &str) {
    println!("Hola, {}", nombre);
}

fn main() {
    // &str
    let nombre1: &str = "Iver";

    // String
    let mut nombre2 = String::from("Padilla");
    nombre2.push_str(" Quesada");

    saludar(nombre1);
    saludar(&nombre2);

    // Vec
    let mut numeros = vec![1, 2, 3];

    for n in &numeros {
        println!("Número: {}", n);
    }

    for n in &mut numeros {
        *n += 1;
    }

    println!("Vector modificado: {:?}", numeros);
}
