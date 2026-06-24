CREATE TABLE IF NOT EXISTS lecciones (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    texto TEXT NOT NULL,
    caracteres INTEGER NOT NULL,
    palabras INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS progreso (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    usuario TEXT NOT NULL,
    leccion_id INTEGER NOT NULL,
    posicion INTEGER NOT NULL,
    errores INTEGER NOT NULL,
    caracteres_correctos INTEGER NOT NULL,
    tiempo_inicio_ms INTEGER NOT NULL,
    FOREIGN KEY (leccion_id) REFERENCES lecciones(id)
);

CREATE TABLE IF NOT EXISTS errores (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    usuario TEXT NOT NULL,
    leccion_id INTEGER NOT NULL,
    tecla TEXT NOT NULL,
    cantidad INTEGER NOT NULL,
    FOREIGN KEY (leccion_id) REFERENCES lecciones(id)
);

INSERT OR IGNORE INTO lecciones (texto, caracteres, palabras) VALUES
('La programación es el arte de decirle a otra persona lo que quieres que haga la computadora.', 97, 13),
('Rust es un lenguaje de programación enfocado en seguridad, velocidad y concurrencia sin necesidad de un recolector de basura.', 125, 15),
('El código limpio siempre parece que fue escrito por alguien a quien le importa lo que hace.', 103, 15),
('La simplicidad es la sofisticación máxima en el diseño de software moderno.', 84, 11),
('Aprender a programar es aprender a pensar de una manera completamente nueva y estructurada.', 96, 13),
('Todo programa tiene al menos un error y puede ser reducido en una línea. Por lo tanto, todo programa puede ser reducido a un error.', 140, 23),
('El mejor código es el que no necesita comentarios para ser entendido por cualquier desarrollador.', 109, 14),
('La experiencia es el nombre que la gente le da a sus errores cuando programa por primera vez.', 102, 15);
