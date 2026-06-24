Constitución del Proyecto: SpeedTyper (Carreras de Teclado)

1. Principios Tecnológicos Inmutables
   Frontend: Angular 20+ (en este repositorio se usa Angular 21.x). El desarrollo debe favorecer Angular Signals y ChangeDetectionStrategy.OnPush. El uso de ChangeDetectionStrategy.Default solo se permite cuando exista una justificación explícita y no sea reemplazable por una solución basada en signals.
   Componentes UI: El framework oficial es PrimeNG. No se permite reemplazarlo por alternativas como Angular Material o Bootstrap.
   Backend: Rust robusto y asíncrono usando Tokio. El manejo de estado de salas debe ser thread-safe y debe evitar condiciones de carrera mediante Arc<RwLock<...>> o paso de mensajes con canales (tokio::sync::mpsc), según convenga.
2. Reglas de Negocio Globales (Seguridad)
   Validación Estricta: El backend es la fuente de verdad. El frontend solo refleja el estado y nunca debe considerarse fiable para validar progreso, resultados o reglas de juego.
   Idempotencia en WebSockets: Todos los mensajes del protocolo WS deben manejar identificadores únicos o mecanismos equivalentes para evitar procesamiento duplicado.
3. Restricciones de Estilo y Accesibilidad
   La aplicación debe evocar una estética de terminal de desarrollo o máquina de escribir. Por ende, la tipografía base para bloques de texto de carrera y estadísticas será siempre Courier o variantes monospaciadas.
   Los layouts principales de formularios, tableros y paneles deben usar los contenedores fluidos de PrimeNG cuando aplique, priorizando la consistencia visual y la accesibilidad.
