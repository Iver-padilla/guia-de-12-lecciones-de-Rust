// Definimos un struct: qué ES un usuario
struct Usuario {
    nombre: String, // nombre del usuario
    edad: u8,        // edad (0–255)
    activo: bool,    // si el usuario está activo o no
}

// impl = lo que el Usuario PUEDE HACER
impl Usuario {
    // Método que SOLO LEE datos
    // &self = préstamo (no modifica nada)
    fn saludar(&self) {
        println!("Hola, soy {}", self.nombre);
    }

    // Método que devuelve un valor
    fn es_mayor(&self) -> bool {
        self.edad >= 18
    }

    // Método que MODIFICA al usuario
    // &mut self = permiso para cambiar cosas
    fn desactivar(&mut self) {
        self.activo = false;
    }
}

fn main() {
    // Creamos un usuario (necesita mut porque luego lo modificamos)
    let mut usuario = Usuario {
        nombre: String::from("Iver"),
        edad: 20,
        activo: true,
    };

    // Llamamos a métodos como si fuera un objeto
    usuario.saludar();

    if usuario.es_mayor() {
        println!("Es mayor de edad");
    } else {
        println!("Es menor de edad");
    }

    // Cambiamos el estado del usuario
    usuario.desactivar();

    println!("Usuario activo? {}", usuario.activo);
}
