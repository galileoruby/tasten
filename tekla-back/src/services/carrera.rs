use crate::models::carrera::LeccionResponse;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

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

#[derive(Debug, sqlx::FromRow)]
struct LeccionDb {
    id: i64,
    texto: String,
    caracteres: i64,
    palabras: i64,
}

/// Retorna una lección aleatoria desde la tabla lecciones
pub async fn obtener_leccion_aleatoria() -> Result<LeccionResponse, anyhow::Error> {
    let db_path = std::env::current_dir()?.join("db").join("tastendb.sqlite");
    let options = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;
    seed_lecciones(&pool).await?;

    let leccion: LeccionDb = sqlx::query_as(
        "SELECT id, texto, caracteres, palabras FROM lecciones ORDER BY RANDOM() LIMIT 1",
    )
    .fetch_one(&pool)
    .await?;

    Ok(LeccionResponse {
        id: leccion.id as usize,
        texto: leccion.texto,
        caracteres: leccion.caracteres as usize,
        palabras: leccion.palabras as usize,
    })
}

pub async fn guardar_progreso(
    pool: &SqlitePool,
    leccion_id: usize,
    usuario: &str,
    posicion: u16,
    errores: u16,
    caracteres_correctos: u16,
    tiempo_inicio_ms: i64,
) -> Result<(), anyhow::Error> {
    seed_lecciones(pool).await?;

    sqlx::query(
        "INSERT INTO progreso (usuario, leccion_id, posicion, errores, caracteres_correctos, tiempo_inicio_ms) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(usuario)
    .bind(leccion_id as i64)
    .bind(posicion as i64)
    .bind(errores as i64)
    .bind(caracteres_correctos as i64)
    .bind(tiempo_inicio_ms)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn guardar_error(
    pool: &SqlitePool,
    leccion_id: usize,
    usuario: &str,
    tecla: &str,
    cantidad: u16,
) -> Result<(), anyhow::Error> {
    seed_lecciones(pool).await?;

    sqlx::query(
        "INSERT INTO errores (usuario, leccion_id, tecla, cantidad) VALUES (?, ?, ?, ?)",
    )
    .bind(usuario)
    .bind(leccion_id as i64)
    .bind(tecla)
    .bind(cantidad as i64)
    .execute(pool)
    .await?;

    Ok(())
}

async fn seed_lecciones(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS lecciones (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            texto TEXT NOT NULL,
            caracteres INTEGER NOT NULL,
            palabras INTEGER NOT NULL
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS progreso (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            usuario TEXT NOT NULL,
            leccion_id INTEGER NOT NULL,
            posicion INTEGER NOT NULL,
            errores INTEGER NOT NULL,
            caracteres_correctos INTEGER NOT NULL,
            tiempo_inicio_ms INTEGER NOT NULL,
            FOREIGN KEY (leccion_id) REFERENCES lecciones(id)
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS errores (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            usuario TEXT NOT NULL,
            leccion_id INTEGER NOT NULL,
            tecla TEXT NOT NULL,
            cantidad INTEGER NOT NULL,
            FOREIGN KEY (leccion_id) REFERENCES lecciones(id)
        )",
    )
    .execute(pool)
    .await?;

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM lecciones")
        .fetch_one(pool)
        .await?;

    if count == 0 {
        let mut tx = pool.begin().await?;

        for texto in PARRAFOS {
            let caracteres = texto.chars().count() as i64;
            let palabras = texto.split_whitespace().count() as i64;

            sqlx::query("INSERT INTO lecciones (texto, caracteres, palabras) VALUES (?, ?, ?)")
                .bind(*texto)
                .bind(caracteres)
                .bind(palabras)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn guarda_progreso_en_tabla() {
        let temp_db = std::env::temp_dir().join(format!("tastendb-test-{}.sqlite", std::process::id()));
        let options = SqliteConnectOptions::new()
            .filename(&temp_db)
            .create_if_missing(true);
        let pool = SqlitePool::connect_with(options).await.unwrap();
        seed_lecciones(&pool).await.unwrap();

        guardar_progreso(&pool, 7, "ana", 12, 2, 34, 123456789).await.unwrap();

        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM progreso")
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(count, 1);

        let _ = std::fs::remove_file(temp_db);
    }
}