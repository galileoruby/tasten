#![allow(unused_imports, dead_code)]
mod karen;
mod michi;

mod especie;

use std::arch::x86_64::_SIDD_MOST_SIGNIFICANT;
use std::ops::Add;

use crate::especie::Especie;
use crate::karen::Persona;
use crate::michi::Mascota;
use chrono::{TimeZone, Utc};

use chrono::prelude::*;

fn main() {
    let fecha = Utc.with_ymd_and_hms(2025, 02, 8, 13, 12, 50);

    match fecha {
        chrono::LocalResult::Single(x_men) => {
            println!("Fecha valida:{}", x_men);
        }
        chrono::LocalResult::Ambiguous(dt1, dt2) => {
            println!("Fecha ambigua:{} o {}", dt1, dt2);
        }
        chrono::offset::LocalResult::None => {
            println!("Fecha perramente invalida");
        }
    }

    let local: DateTime<Local> = Local::now();
    let _michi: Especie = Especie::Gato;
    let _michi2: Especie = Especie::Hamster;

    let _kira = Mascota::new(
        String::from("enojona"),
        local,
        String::from("calico"),
        String::from("pollito asado"),
        _michi,
    );

    let _naranjoso: Mascota = Mascota::new(
        String::from("el naranjas"),
        local,
        String::from("ginger"),
        String::from("aguacate"),
        _michi2,
    );

    let mut persona: Persona = Persona::nueva(String::from("Karen martinez"), 15);

    persona.agregar_mascota(_kira);
    persona.agregar_mascota(_naranjoso);

    // let segundos=local.nanosecond();

    // let mascota: Mascota =-. Mascota::new(String::from("puto"), fecha);
    // println!("Hola  and secondos: {} ", segundos);
    println!("{:#?}", persona.mascotas);

    persona.actualizar_comidafavorita_mascota(0, String::from("pollito-con-papas"));
    println!("{:#?}", persona.mascotas);

    let _myplaceholder: i32 = 34;
    let myplaceholder: i32 = 34;
    // println!(
    //     "{}",
    //     _kira.fecha_nacimiento.format("%Y-%m-%d %H:%M").to_string()
    // );
    // println!("{}", _kira.fecha_nacimiento.format("%m -%d").to_string());
}
