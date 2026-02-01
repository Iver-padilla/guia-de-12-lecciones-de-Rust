// Función que intenta buscar un usuario por id
// Puede DEVOLVER un nombre o NADA
fn buscar_usuario(id: u32) -> Option<String> {
    // Si el id es 1, el usuario existe
    if id == 1 {
        // Some significa: "sí hay algo"
        Some(String::from("Iver"))
    } else {
        // None significa: "no hay nada"
        None
    }
}

fn main() {
    // Llamamos a la función
    let resultado = buscar_usuario(1);

    // match obliga a manejar TODOS los casos
    match resultado {
        // Si hay un valor, Rust lo saca de Some(...)
        Some(nombre) => {
            println!("Usuario encontrado: {}", nombre);
        }

        // Si no hay nada
        None => {
            println!("Usuario no existe");
        }
    }
}
