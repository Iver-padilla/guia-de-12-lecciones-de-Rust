// Función que puede FALLAR
// Ok(valor) si todo va bien
// Err(error) si algo sale mal
fn dividir(a: i32, b: i32) -> Result<i32, String> {
    // Validación
    if b == 0 {
        // Err contiene el error
        Err(String::from("No se puede dividir entre cero"))
    } else {
        // Ok contiene el resultado correcto
        Ok(a / b)
    }
}

fn main() {
    // Llamamos a la función
    let resultado = dividir(10, 2);

    // match obliga a manejar éxito y error
    match resultado {
        // Todo salió bien
        Ok(valor) => {
            println!("Resultado: {}", valor);
        }

        // Algo falló
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}
