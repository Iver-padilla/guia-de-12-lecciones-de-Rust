// Importamos el struct Usuario
use crate::models::Usuario;

// Función pública
pub fn crear_usuario(id: u32, nombre: String) -> Usuario {
    Usuario { id, nombre }
}
