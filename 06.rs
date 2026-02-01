fn mostrar(texto: &String) {
    println!("Texto: {}", texto);
}

fn agregar_exclamacion(texto: &mut String) {
    texto.push('!');
}

fn main() {
    let nombre = String::from("Iver");
    mostrar(&nombre);
    println!("Después de mostrar: {}", nombre);

    let mut saludo = String::from("Hola");
    agregar_exclamacion(&mut saludo);
    println!("Saludo modificado: {}", saludo);
}

/*basicamente
string = egoista por que no te presta y si lo das pierdes
&string = amigable porque presta
 */