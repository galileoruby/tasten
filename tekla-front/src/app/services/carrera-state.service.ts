import { Injectable, signal, computed, OnDestroy } from '@angular/core';
import { Subscription } from 'rxjs';
import { CarreraWsService } from './carrera-ws.service';

export interface ErrorTecla { tecla: string; cantidad: number; }
export interface JugadorSala {
  usuario: string; posicion: number; errores: number;
  precision: number; wpm: number; terminado: boolean;
  abandonado: boolean;
  tiempo_segundos?: number;
}

@Injectable({ providedIn: 'root' })
export class CarreraStateService implements OnDestroy {

  // ── Configuración (en el futuro vendrá del auth/lobby) ──
  readonly usuario = 'ana';
  readonly salaId  = `sala-${Math.floor(Math.random() * 50) + 1}`;

  private subs = new Subscription();
  private tiempoInicio: number | null = null;

  constructor(private ws: CarreraWsService) {}

  // ════════════════════════════════════════════
  // SIGNALS — leídos por Leccion y Estadisticas
  // ════════════════════════════════════════════

  textoCarrera       = signal<string>('');
  totalCaracteres    = signal<number>(0);
  inputActual        = signal<string>('');
  posicionLocal      = signal<number>(0);
  erroresLocales     = signal<number>(0);
  caracteresCorrectos = signal<number>(0);
  carreraIniciada    = signal<boolean>(false);
  carreraTerminada   = signal<boolean>(false);
  errorPorTecla      = signal<ErrorTecla[]>([]);
  jugadores          = signal<Map<string, JugadorSala>>(new Map());

  // ── Computed ──
  porcentajeProgreso = computed(() => {
    const total = this.totalCaracteres();
    if (total <= 0) return 0;

    const progreso = this.carreraTerminada()
      ? 100
      : this.posicionLocal() / total;

    return Math.min(100, Math.round(progreso * 100));
  });

  precisionLocal = computed(() => {
    const totalIntentos = this.caracteresCorrectos() + this.erroresLocales();
    if (totalIntentos === 0) return 100;
    return Math.round((this.caracteresCorrectos() / totalIntentos) * 100);
  });

  wpmLocal = computed(() => {
    if (!this.tiempoInicio || this.caracteresCorrectos() === 0) return 0;
    const min = (Date.now() - this.tiempoInicio) / 60000;
    return Math.round((this.caracteresCorrectos() / 5) / min);
  });

  jugadoresOrdenados = computed(() =>
    [...this.jugadores().values()].sort((a, b) => b.posicion - a.posicion)
  );

  // ════════════════════════════════════════════
  // CONECTAR — llamar desde el componente página
  // ════════════════════════════════════════════

  conectar(): void {
    this.ws.conectar(this.salaId, this.usuario);

    // Texto de la carrera
    this.subs.add(
      this.ws.textoCarrera$().subscribe(e => {
        this.textoCarrera.set(e.texto);
        this.totalCaracteres.set(e.caracteres);
      })
    );

    // Progreso de otros jugadores
    this.subs.add(
      this.ws.progreso$().subscribe(e => {
        if (e.usuario === this.usuario) return;
        const m = new Map(this.jugadores());
        m.set(e.usuario, {
          usuario: e.usuario, posicion: e.posicion,
          errores: e.errores, precision: e.precision,
          wpm: e.wpm, terminado: false, abandonado: false,
        });
        this.jugadores.set(m);
      })
    );

    // Jugador se unió
    this.subs.add(
      this.ws.jugadorUnido$().subscribe(e => {
        const m = new Map(this.jugadores());
        const jugadorExistente = m.get(e.usuario);
        m.set(e.usuario, {
          usuario: e.usuario,
          posicion: jugadorExistente?.posicion ?? 0,
          errores: jugadorExistente?.errores ?? 0,
          precision: jugadorExistente?.precision ?? 0,
          wpm: jugadorExistente?.wpm ?? 0,
          terminado: jugadorExistente?.terminado ?? false,
          abandonado: jugadorExistente?.abandonado ?? false,
        });
        this.jugadores.set(m);
      })
    );

    // Jugador terminó
    this.subs.add(
      this.ws.jugadorTermino$().subscribe(e => {
        const m = new Map(this.jugadores());
        const j = m.get(e.usuario) ?? {
          usuario: e.usuario, posicion: this.totalCaracteres(),
          errores: 0, precision: e.precision, wpm: e.wpm, terminado: false, abandonado: false,
        };
        m.set(e.usuario, { ...j, terminado: true, abandonado: false,
          tiempo_segundos: e.tiempo_segundos, wpm: e.wpm, precision: e.precision });
        this.jugadores.set(m);
      })
    );

    // Jugador salió
    this.subs.add(
      this.ws.jugadorSalio$().subscribe(e => {
        const m = new Map(this.jugadores());
        const jugador = m.get(e.usuario);
        if (jugador) {
          m.set(e.usuario, { ...jugador, abandonado: true });
        }
        this.jugadores.set(m);
      })
    );
  }

  desconectar(): void {
    this.subs.unsubscribe();
    this.ws.desconectar();
  }

  // ════════════════════════════════════════════
  // ACCIÓN — llamada desde TecladoComponent/Keyboard
  // ════════════════════════════════════════════

  procesarTecla(tecla: string): void {
    const texto = this.textoCarrera();
    if (!texto || this.carreraTerminada()) return;

    // Arrancar cronómetro en primera tecla
    if (!this.carreraIniciada()) {
      this.tiempoInicio = Date.now();
      this.carreraIniciada.set(true);
    }

    // Validar carácter comparando contra el texto
    const pos = this.posicionLocal();
    const esCorrecta = tecla === texto[pos];

    if (esCorrecta) {
      this.caracteresCorrectos.update(v => v + 1);
      this.posicionLocal.update(v => v + 1);
    } else {
      this.erroresLocales.update(v => v + 1);
      this.registrarErrorTecla(tecla);
    }

    // Enviar progreso al servidor
    this.ws.enviar({
      tipo: 'progreso',
      posicion: this.posicionLocal(),
      errores: this.erroresLocales(),
      caracteres_correctos: this.caracteresCorrectos(),
      tiempo_inicio_ms: this.tiempoInicio ?? Date.now(),
    });

    // Verificar si terminó
    if (this.posicionLocal() >= texto.length) {
      this.terminar();
    }
  }

  private terminar(): void {
    if (this.carreraTerminada()) return;
    this.carreraTerminada.set(true);

    const tiempoSegundos = this.tiempoInicio
      ? Math.round((Date.now() - this.tiempoInicio) / 1000)
      : 0;

    this.ws.enviar({
      tipo: 'termino',
      tiempo_segundos: tiempoSegundos,
      errores: this.erroresLocales(),
      caracteres_correctos: this.caracteresCorrectos(),
      total_caracteres: this.totalCaracteres(),
    });
  }

  private registrarErrorTecla(tecla: string): void {
    const lista = [...this.errorPorTecla()];
    const ex = lista.find(e => e.tecla === tecla);
    if (ex) ex.cantidad++;
    else lista.push({ tecla, cantidad: 1 });
    this.errorPorTecla.set(lista);
  }

  ngOnDestroy(): void { this.desconectar(); }
}