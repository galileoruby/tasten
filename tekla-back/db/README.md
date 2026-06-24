# Base de datos de Tekla Back

Esta carpeta contiene la base de datos SQLite para el proyecto.

## Archivo principal
- `tastendb.sql`: esquema SQL para crear la tabla `lecciones`.

## Crear la base de datos
Ejecuta:

```bash
sqlite3 db/tastendb.sqlite < db/tastendb.sql
```
