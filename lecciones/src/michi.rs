use chrono::{DateTime, Utc};
use chrono::prelude::*;

use crate::especie::Especie;

#[derive(Debug)]
pub struct Mascota {
    pub nombre: String,
    pub fecha_nacimiento: DateTime<Local>,
    pub color: String,
    pub comida_favorita: String,
    pub especie: Especie,
     modificado: bool
}

impl Mascota {
    pub fn new(
        nombre: String,
        fecha: DateTime<Local>,
        color: String,
        comida_favorita: String,
        especie: Especie,
    ) -> Self {
        Self {
            nombre,
            fecha_nacimiento: fecha,
            color,
            comida_favorita: comida_favorita,
            especie,
            modificado: false
        }
    }

    pub fn tarjeta(&self) {
        println!("Hola soy {}", self.nombre);
    }

    pub fn actualizar_comida_favorita(&mut self, nueva_comida: String) {
        self.comida_favorita = nueva_comida;
        self.modificado = true;
    }
}
