fn imprimir_numero(n: i32) {
    println!("Número: {}", n);
}

fn imprimir_texto(texto: String) {
    println!("Texto: {}", texto);
}

fn main() {
    // Ejemplo con tipos Copy (i32)
    let numero = 10;
    imprimir_numero(numero);
    println!("Número sigue vivo: {}", numero); // OK, i32 se copia automáticamente

    // Ejemplo con String (no Copy)
    let nombre = String::from("Iver");
    imprimir_texto(nombre);
    // println!("Nombre: {}", nombre); 
    // ❌ Esto NO compila porque imprimir_texto se llevó la propiedad de 'nombre'

    // Para evitarlo, podemos clonar:
    let apellido = String::from("Padilla");
    let copia = apellido.clone(); // Creamos una copia completa en memoria

    println!("Original: {}", apellido); // OK, el original sigue vivo
    println!("Copia: {}", copia);      // OK, la copia también funciona

    // Otro ejemplo más claro:
    let nombre2 = String::from("Iver");
    let copia_nombre = nombre2.clone();

    println!("nombre2: {}", nombre2);          // OK, sigue vivo
    println!("copia_nombre: {}", copia_nombre); // OK, la copia también existe
}
