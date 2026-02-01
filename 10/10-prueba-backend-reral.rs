fn obtener_usuario(id: u32) -> Result<String, String> {
    if id == 0 {
        Err(String::from("ID inválido"))
    } else {
        Ok(String::from("Usuario encontrado"))
    }
}
