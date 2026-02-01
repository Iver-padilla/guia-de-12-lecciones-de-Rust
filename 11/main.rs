mod models;
mod services;

use services::crear_usuario;

fn main() {
    let user = crear_usuario(1, String::from("Iver"));
    println!("Usuario: {} (id {})", user.nombre, user.id);
}
