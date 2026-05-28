use rand::Rng;
use crate::models::carrera::LeccionResponse;

/// Banco de párrafos para las lecciones de typing
const PARRAFOS: &[&str] = &[
    "La programación es el arte de decirle a otra persona lo que quieres que haga la computadora.",
    "Rust es un lenguaje de programación enfocado en seguridad, velocidad y concurrencia sin necesidad de un recolector de basura.",
    "El código limpio siempre parece que fue escrito por alguien a quien le importa lo que hace.",
    "La simplicidad es la sofisticación máxima en el diseño de software moderno.",
    "Aprender a programar es aprender a pensar de una manera completamente nueva y estructurada.",
    "Todo programa tiene al menos un error y puede ser reducido en una línea. Por lo tanto, todo programa puede ser reducido a un error.",
    "El mejor código es el que no necesita comentarios para ser entendido por cualquier desarrollador.",
    "La experiencia es el nombre que la gente le da a sus errores cuando programa por primera vez.",
];

/// Retorna un párrafo aleatorio del banco de lecciones
pub fn obtener_leccion_aleatoria() -> LeccionResponse {
    let mut rng = rand::rng();
    // let id = rng.gen_range(0..PARRAFOS.len());
    let id = rng.random_range(0..PARRAFOS.len());
    let texto = PARRAFOS[id].to_string();
    let caracteres = texto.len();
    let palabras = texto.split_whitespace().count();

    LeccionResponse {
        id,
        texto,
        caracteres,
        palabras,
    }
}