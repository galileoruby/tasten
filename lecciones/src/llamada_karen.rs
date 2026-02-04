use crate::especie::Especie;
use crate::karen::Persona;
use crate::michi::Mascota;
use chrono::{DateTime, Local, TimeZone, Utc};

pub fn ejecutar() {
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
    let michi: Especie = Especie::Gato;
    let michi2: Especie = Especie::Hamster;

    let kira = Mascota::new(
        String::from("enojona"),
        local,
        String::from("calico"),
        String::from("pollito asado"),
        michi,
    );

    let naranjoso = Mascota::new(
        String::from("el naranjas"),
        local,
        String::from("ginger"),
        String::from("aguacate"),
        michi2,
    );

    let mut persona = Persona::nueva(String::from("Karen martinez"), 15);

    persona.agregar_mascota(kira);
    persona.agregar_mascota(naranjoso);

    println!("{:#?}", persona.mascotas);

    persona.actualizar_comidafavorita_mascota(0, String::from("pollito-con-papas"));
    println!("{:#?}", persona.mascotas);
}
