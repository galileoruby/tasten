use super::libro::Libro;

#[derive(Debug, Clone)]
pub struct Biblioteca {
    libros: Vec<Libro>,
}

impl Biblioteca {
    pub fn new() -> Self {
        Self { libros: Vec::new() }
    }
    pub fn agregar_libro(&mut self, libro: Libro) {
        self.libros.push(libro);
    }

    pub fn buscar_por_titulo(&self, titulo: &str) -> Option<&Libro> {
        self.libros.iter().find(|libro| libro.titulo == titulo)
    }

    // Listar libros disponibles
    pub fn libros_disponibles(&self) -> Vec<&Libro> {
        self.libros
            .iter()
            .filter(|libro| libro.disponible)
            .collect()
    }

    // Prestar libro
    pub fn prestar_libro(&mut self, titulo: &str) -> Result<(), String> {
        if let Some(libro) = self.libros.iter_mut().find(|l| l.titulo == titulo) {
            libro.prestar()
        } else {
            Err(format!("Libro '{}' no encontrado", titulo))
        }
    }

    // Estadísticas
    pub fn estadisticas(&self) -> (usize, usize) {
        let total = self.libros.len();
        let disponibles = self.libros.iter().filter(|l| l.disponible).count();
        (total, disponibles)
    }
}
