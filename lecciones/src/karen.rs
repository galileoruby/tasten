use crate::especie::Especie;

use crate::michi::Mascota;
#[derive(Debug)]
pub struct Persona {
    pub nombre: String,
    pub edad: u32,
    pub mascotas: Vec<Mascota>,
}

impl Persona {
    pub fn nueva(nombre: String, edad: u32) -> Self {
        Self {
            nombre,
            edad,
            mascotas: Vec::new(),
        }
    }

    pub fn agregar_mascota(&mut self, mascota: Mascota) {
        self.mascotas.push(mascota);
    }

    pub fn tiene_mascotas(&self) -> bool {
        !self.mascotas.is_empty()
    }

    pub fn cantidad_mascotas(&self) -> usize {
        self.mascotas.len()
    }

    pub fn actualizar_comidafavorita_mascota(&mut self, indice: usize, nueva_comida: String) {
        if let Some(mascota) = self.mascotas.get_mut(indice) {
            mascota.actualizar_comida_favorita(nueva_comida);
        }else{
            println!("No hay registros para actualizar.")
        }
    }
}
