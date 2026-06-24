// ─────────────────────────────────────────────
// Mensajes que el SERVIDOR envía al cliente
// Espejo exacto de EventoSala en state.rs
// ─────────────────────────────────────────────

export interface EventoTextoCarrera {
  tipo: 'texto_carrera';
  id: number;
  texto: string;
  caracteres: number;
  palabras: number;
}

export interface EventoProgreso {
  tipo: 'progreso';
  usuario: string;
  posicion: number;
  errores: number;
  precision: number;
  wpm: number;
}

export interface EventoJugadorTermino {
  tipo: 'jugador_termino';
  usuario: string;
  tiempo_segundos: number;
  precision: number;
  wpm: number;
  posicion_ranking: number;
  errores?: ErrorTecla[];
}

export interface ErrorTecla {
  tecla: string;
  cantidad: number;
}

export interface EventoJugadorUnido {
  tipo: 'jugador_unido';
  usuario: string;
  total_jugadores: number;
}

export interface EventoJugadorSalio {
  tipo: 'jugador_salio';
  usuario: string;
}

export interface EventoError {
  tipo: 'error';
  mensaje: string;
}

// Union type — cualquier evento posible del servidor
export type EventoSala =
  | EventoTextoCarrera
  | EventoProgreso
  | EventoJugadorTermino
  | EventoJugadorUnido
  | EventoJugadorSalio
  | EventoError;

// ─────────────────────────────────────────────
// Mensajes que el CLIENTE envía al servidor
// Espejo exacto de MensajeCliente en ws.rs
// ─────────────────────────────────────────────

export interface MensajeProgreso {
  tipo: 'progreso';
  posicion: number;
  errores: number;
  caracteres_correctos: number;
  tiempo_inicio_ms: number; // Date.now() al empezar
}

export interface MensajeTermino {
  tipo: 'termino';
  tiempo_segundos: number;
  errores: ErrorTecla[];
  caracteres_correctos: number;
  total_caracteres: number;
}

export type MensajeCliente = MensajeProgreso | MensajeTermino;