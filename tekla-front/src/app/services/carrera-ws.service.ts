import { Injectable, OnDestroy } from '@angular/core';
import { webSocket, WebSocketSubject } from 'rxjs/webSocket';
import { Observable, Subject, EMPTY } from 'rxjs';
import { catchError, filter, tap } from 'rxjs/operators';
import {
  EventoSala,
  EventoTextoCarrera,
  EventoProgreso,
  EventoJugadorTermino,
  EventoJugadorUnido,
  EventoJugadorSalio,
  MensajeCliente,
} from '../models/carrera.models';

@Injectable({ providedIn: 'root' })
export class CarreraWsService implements OnDestroy {

  // Dos tipos: OUT = lo que recibimos, IN = lo que enviamos
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  private socket$!: WebSocketSubject<any>;
  private readonly URL_BASE = 'ws://localhost:3000/ws';

  // ─── Conectar a una sala ───────────────────
  conectar(salaId: string, usuario: string): void {
    // Si ya hay una conexión abierta, cerrarla primero
    this.desconectar();

    this.socket$ = webSocket<any>({
      url: `${this.URL_BASE}/${salaId}?usuario=${usuario}`,

      // webSocket de RxJS serializa/deserializa JSON automáticamente
      // No necesitas JSON.parse ni JSON.stringify manualmente

      // Callback cuando la conexión abre
      openObserver: {
        next: () => console.log(`[WS] Conectado: sala=${salaId} usuario=${usuario}`)
      },
      // Callback cuando la conexión cierra
      closeObserver: {
        next: () => console.log('[WS] Desconectado')
      }
    });
  }

  // ─── Stream general — todos los eventos ───
  // Los componentes pueden suscribirse a esto
  // y filtrar por tipo lo que les interesa
  mensajes$(): Observable<EventoSala> {
    return this.socket$.pipe(
      catchError(err => {
        console.error('[WS] Error:', err);
        return EMPTY;
      })
    );
  }

  // ─── Streams filtrados por tipo de evento ─
  // Más cómodo en los componentes que hacer el filter manual

  textoCarrera$(): Observable<EventoTextoCarrera> {
    return this.mensajes$().pipe(
      filter((e): e is EventoTextoCarrera => e.tipo === 'texto_carrera')
    );
  }

  progreso$(): Observable<EventoProgreso> {
    return this.mensajes$().pipe( 
      filter((e): e is EventoProgreso => e.tipo === 'progreso')
    );
  }

  jugadorTermino$(): Observable<EventoJugadorTermino> {
    return this.mensajes$().pipe(
      filter((e): e is EventoJugadorTermino => e.tipo === 'jugador_termino')
    );
  }

  jugadorUnido$(): Observable<EventoJugadorUnido> {
    return this.mensajes$().pipe(
      filter((e): e is EventoJugadorUnido => e.tipo === 'jugador_unido')
    );
  }

  jugadorSalio$(): Observable<EventoJugadorSalio> {
    return this.mensajes$().pipe(
      filter((e): e is EventoJugadorSalio => e.tipo === 'jugador_salio')
    );
  }

  // ─── Enviar mensaje al servidor ───────────
  enviar(mensaje: MensajeCliente): void {
    if (!this.socket$) {
      console.warn('[WS] No hay conexión activa');
      return;
    }
    // .next() serializa a JSON automáticamente y lo envía
    this.socket$.next(mensaje);
  }

  // ─── Cerrar conexión ──────────────────────
  desconectar(): void {
    this.socket$?.complete();
  }

  // Si el servicio se destruye, cerrar la conexión
  ngOnDestroy(): void {
    this.desconectar();
  }
}