#!/usr/bin/env bash
# Instalar websocat una sola vez:
#   cargo install websocat

# ─────────────────────────────────────────────
# TERMINAL 1 — Jugador "ana" se une a sala1
# ─────────────────────────────────────────────
websocat "ws://127.0.0.1:3000/ws/sala1?usuario=ana"
# Recibirás de inmediato el TextoCarrera:
# {"tipo":"texto_carrera","id":3,"texto":"Rust es...","caracteres":90,"palabras":15}

# Luego envía progreso (pega esto y presiona Enter):
{"tipo":"progreso","posicion":10,"errores":1,"caracteres_correctos":9,"tiempo_inicio_ms":1700000000000}

# Al terminar:
{"tipo":"termino","tiempo_segundos":42,"errores":2,"caracteres_correctos":88,"total_caracteres":90}


# ─────────────────────────────────────────────
# TERMINAL 2 — Jugador "bob" en la MISMA sala
# ─────────────────────────────────────────────
websocat "ws://127.0.0.1:3000/ws/sala1?usuario=bob"
# Verás que recibe el mismo texto Y los eventos de "ana" en tiempo real

{"tipo":"progreso","posicion":15,"errores":0,"caracteres_correctos":15,"tiempo_inicio_ms":1700000000000}


# ─────────────────────────────────────────────
# TERMINAL 3 — Jugador "carlos" en sala DISTINTA
# ─────────────────────────────────────────────
websocat "ws://127.0.0.1:3000/ws/sala2?usuario=carlos"
# Recibe un texto diferente, aislado de sala1


# ─────────────────────────────────────────────
# ALTERNATIVA: probar con wscat (npm)
# npm install -g wscat
# wscat -c "ws://127.0.0.1:3000/ws/sala1?usuario=ana"
# ─────────────────────────────────────────────


#progreso actual
{
  "tipo": "progreso",
  "posicion": 90,
  "errores": 2,
  "caracteres_correctos": 100,
  "tiempo_inicio_ms": 1710000000000
}


# termino
{
  "tipo": "jugador_termino",
  "usuario": "eminem",
  "tiempo_segundos": 42,
  "precision": 100.0,
  "wpm": 35.5,
  "posicion_ranking": 3
}