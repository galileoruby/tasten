use chrono::{DateTime, Utc};
// use serde::{Deserialize, Serialize};
use chrono::prelude::*;

use crate::especie::Especie;

#[derive(Debug)]
pub struct Mascota {
    pub nombre: String,
    pub fecha_nacimiento: DateTime<Local>,
    pub color: String,
    pub comida_favorita: String,
    pub especie: Especie,
}

impl Mascota {
    pub fn new(
        nombre: String,
        fecha: DateTime<Local>,
        color: String,
        comida_favorita: String,
        especie: Especie
    ) -> Self {
        Self {
            nombre,
            fecha_nacimiento: fecha,
            color,
            comida_favorita: comida_favorita,
            especie
        }
    }

    pub fn tarjeta(&self) {
        println!("Hola soy {}", self.nombre);
    }
}
