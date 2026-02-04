use super::Biblioteca;
use super::Libro;



pub fn main_1() {
    let mut mi_biblioteca = Biblioteca::new();

    // Agregar libros
    mi_biblioteca.agregar_libro(Libro::new(
        "El Quijote".to_string(),
        "Miguel de Cervantes".to_string(),
        1605,
    ));

    mi_biblioteca.agregar_libro(Libro::new(
        "Cien años de soledad".to_string(),
        "Gabriel García Márquez".to_string(),
        1967,
    ));

    mi_biblioteca.agregar_libro(Libro::new(
        "1984".to_string(),
        "George Orwell".to_string(),
        1949,
    ));

    // Mostrar disponibles
    println!("📚 Libros disponibles:");
    for libro in mi_biblioteca.libros_disponibles() {
        println!("  - {}", libro.titulo());
    }

    // Prestar un libro
    match mi_biblioteca.prestar_libro("1984") {
        Ok(_) => println!("✅ '1984' prestado exitosamente"),
        Err(e) => println!("❌ Error: {}", e),
    }

    // Intentar prestar mismo libro otra vez
    match mi_biblioteca.prestar_libro("1984") {
        Ok(_) => println!("✅ Prestado"),
        Err(e) => println!("❌ Error: {}", e),
    };

    // Estadísticas
    let (total, disponibles) = mi_biblioteca.estadisticas();
    println!(
        "📊 Estadísticas: {}/{} libros disponibles",
        disponibles, total
    );
}
