enum Estado {
    Nino,
    Adolescente,
    Adulto,
}

struct Persona {
    nombre: String,
    edad: u8,
    estado: Estado,
}

fn main() {
    let persona = Persona {
        nombre: String::from("Iver"),
        edad: 17,
        estado: Estado::Adolescente,
    };

    match &persona.estado {
        Estado::Nino => {
            println!("{} tiene {} años y es un niño", persona.nombre, persona.edad);
        }
        Estado::Adolescente => {
            println!("{} tiene {} años y es adolescente", persona.nombre, persona.edad);
        }
        Estado::Adulto => {
            println!("{} tiene {} años y es adulto", persona.nombre, persona.edad);
        }
    }
}
