use rand::Rng;

#[derive(Debug)]
pub struct Carrera {
    pub texto_generado: String,
}

impl Carrera {
    // pub fn carrera_aleatoria(textoGenerado: String) -> Self {
    //     Self { textoGenerado }
    // }
    pub fn leccion_aleatoria() -> String {
        const LECCIONES_ESP: [&str; 8] = [
            "La tecnología avanza a pasos agigantados, transformando cada aspecto de nuestra vida cotidiana.",
            "La inteligencia artificial y el aprendizaje automático están revolucionando industrias enteras.",
            "Ofreciendo soluciones antes consideradas imposibles. Estas herramientas no solo optimizan tareas repetitivas",
            ", sino que también abren puertas a descubrimientos científicos.",
            "El desarrollo sostenible se ha convertido en una prioridad global ante los crecientes desafíos ambientales.",
            " La colaboración internacional y la innovación tecnológica son clave para encontrar soluciones.",
            "La educación del siglo XXI requiere una transformación profunda para preparar a las nuevas generaciones en un mundo en constante cambio.",
            "Las habilidades digitales y el pensamiento crítico son esenciales.",
        ];

        let mut rng = rand::rng();
        let indice = rng.random_range(0..LECCIONES_ESP.len());

        LECCIONES_ESP[indice].to_string()
    }
}