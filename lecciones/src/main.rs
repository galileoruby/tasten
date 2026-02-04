#![allow(unused_imports, dead_code)]
mod karen;
mod michi;
mod paquetelibros;

mod metodos_o;

mod vectores;

mod especie;
mod llamada_karen;
use console::{Term, style};
use std::arch::x86_64::_SIDD_MOST_SIGNIFICANT;
use std::io::{self, Read};
use std::ops::Add;

use paquetelibros::main_1;
use vectores::dinamicos;

fn main() {
    loop {
        println!("{}", style("Pruebas en rustaceo").cyan().bold());
        println!("{}", style("1. Leer archivo").green());
        println!("{}", style("2. Generar numeros aleatorios").green()) ;
        println!("{}", style("S. Salir").red() );
        print!("{}", style("Seleccione una opción:").yellow());
        println!();

        let mut opcion = String::new();
        io::stdin().read_line(&mut opcion).unwrap();

        print!("Opcion seleccionada::{}.-", opcion);

        match opcion.trim() {
            "1" => _ = paquetelibros::Libro::leer_archivo(),
            "2" => _ = metodos_o::burbuja::orden_burbuja(),
            op if matches!(op, "S" | "s") => {
                println!("Adios");
                break;
            }
            _ => println!("Opcion no valida"),
        }
    }
}
