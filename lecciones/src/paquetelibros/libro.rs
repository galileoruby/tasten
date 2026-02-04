use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Libro {
    pub titulo: String,
    pub autor: String,
    pub year_publicacion: u32,
    pub disponible: bool,
}

fn module_path_to_dir(module_path: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Remover "crate::" si existe
    let path = if module_path.starts_with("crate::") {
        &module_path[7..]
    } else {
        module_path
    };

    // Reemplazar "::" con separador de directorio
    let dir_path = path.replace("::", "/");

    // Comenzar desde la raíz del proyecto
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());

    let full_path = PathBuf::from(manifest_dir).join("src").join(dir_path);

    // Si quieres subir un nivel (de libro/ a paquetelibros/)
    let parent_dir = full_path
        .parent()
        .ok_or("No se pudo obtener directorio padre")?
        .to_path_buf();

    Ok(parent_dir)
}

impl Libro {
    pub fn new(titulo: String, autor: String, año_publicacion: u32) -> Self {
        Self {
            titulo,
            autor,
            year_publicacion: año_publicacion,
            disponible: true,
        }
    }

   pub fn leer_archivo() -> Result<(), Box<dyn std::error::Error>> {
    let current_file = file!();
    println!("📄 Archivo fuente actual: {}", current_file);

    let current_path = Path::new(current_file);
    let source_dir = current_path
        .parent()
        .ok_or("No se pudo obtener directorio del archivo")?;

    // Buscar recursivamente el archivo
    if let Some(archivo_path) = Self::find_file_recursive(source_dir, "librero.txt") {
        println!("✅ Archivo encontrado en: {:?}", archivo_path);
        
        let contenido = fs::read_to_string(&archivo_path)?;
        println!("📝 Contenido:\n{}", contenido);
        Ok(())
    } else {
        // Buscar en directorio padre si no está en el actual
        let target_dir = source_dir
            .parent()
            .ok_or("No se pudo obtener directorio padre")?;
            
        if let Some(archivo_path) = Self::find_file_in_dir(target_dir, "librero.txt") {
            println!("✅ Archivo encontrado en directorio padre: {:?}", archivo_path);
            
            let contenido = fs::read_to_string(&archivo_path)?;
            println!("📝 Contenido:\n{}", contenido);
            Ok(())
        } else {
            eprintln!("❌ Archivo 'librero.txt' no encontrado");
            Err("Archivo no encontrado".into())
        }
    }
}

/// Busca un archivo en un directorio (sin recursión)
fn find_file_in_dir(dir: &Path, filename: &str) -> Option<PathBuf> {
    std::fs::read_dir(dir)
        .ok()?
        .filter_map(Result::ok)
        .find(|entry| {
            entry.file_name() == filename
        })
        .map(|entry| entry.path())
}

/// Busca recursivamente un archivo
fn find_file_recursive(dir: &Path, filename: &str) -> Option<PathBuf> {

    for entry in std::fs::read_dir(dir).ok()?.filter_map(Result::ok) {
        let path = entry.path();
        println!("iterando::{:#?}", &path.file_name());
        
        if path.is_file() && path.file_name()?.to_string_lossy() == filename {
            return Some(path);
        } else if path.is_dir() {
            if let Some(found) = Self::find_file_recursive(&path, filename) {
                return Some(found);
            }
        }
    }
    None
}

    // Getters
    pub fn titulo(&self) -> &str {
        &self.titulo
    }

    pub fn disponible(&self) -> bool {
        self.disponible
    }

    pub fn prestar(&mut self) -> Result<(), String> {
        if !self.disponible {
            return Err(format!("'{}' ya está prestado", self.titulo));
        }

        self.disponible = false;
        Ok(())
    }

    pub fn devolver(&mut self) {
        self.disponible = true;
    }
}
