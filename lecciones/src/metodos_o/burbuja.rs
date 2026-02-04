use rand::{Rng, rng, seq::SliceRandom};

pub fn orden_burbuja() {
    let numeros = generar_numeros(7, 500, 1000);
    println!("todos numeros: {:#?}", numeros);
}

pub fn generar_numeros(cantidad: usize, min: i32, max: i32) -> Vec<i32> {
    println!("generar_numeros");
    //let mut rng = rand::thread_rng();
    let mut rng = rng();
    let mut numeros = Vec::with_capacity(cantidad);

    for _ in 0..cantidad {
        let numero = rng.random_range(min..=max);
        numeros.push(numero);
    }
    numeros
}