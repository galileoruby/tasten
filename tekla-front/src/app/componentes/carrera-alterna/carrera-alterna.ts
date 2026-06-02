
import {
  Component, OnInit, OnDestroy, signal, computed
} from '@angular/core';
import { Subscription } from 'rxjs';
import { CarreraWsService } from '../../services/carrera-ws.service';
import { EventoProgreso, EventoJugadorTermino } from '../../models/carrera.models';

// PrimeNG — cada componente se importa por separado
import { CardModule } from 'primeng/card';
import { InputTextModule } from 'primeng/inputtext';
import { ProgressBarModule } from 'primeng/progressbar';
import { TableModule } from 'primeng/table';
import { TagModule } from 'primeng/tag';
import { CommonModule, DecimalPipe } from '@angular/common';
import { ServicioTexto } from '../../services/servicio-texto';

interface EstadoJugador {
  usuario: string;
  posicion: number;
  errores: number;
  precision: number;
  wpm: number;
  terminado: boolean;
  tiempo_segundos?: number;
}


@Component({
  selector: 'app-carrera-alterna',
  imports: [
    CommonModule,
    DecimalPipe,
    CardModule,
    InputTextModule,
    ProgressBarModule,
    TableModule,
    TagModule,
  ],
  templateUrl: './carrera-alterna.html',
  styleUrl: './carrera-alterna.less',
})
export class CarreraAlterna implements OnInit, OnDestroy {

  // ─── Estado de la carrera (signals — Angular 21) ───
  textoCarrera = signal<string>('');
  totalCaracteres = signal<number>(0);
  jugadores = signal<Map<string, EstadoJugador>>(new Map());
  conectado = signal<boolean>(false);
  carreraIniciada = signal<boolean>(false);

  // Genera: "sala-4821", "sala-7392", etc.
   

  // Estado del jugador local
  private usuario = 'ana';         // vendrá de tu auth
  private salaId = `sala-${Math.floor(Math.random() * 50) + 1}`;
  private tiempoInicio: number | null = null;
  posicionLocal = signal<number>(0);
  erroresLocales = signal<number>(0);
  inputUsuario = signal<string>('');


  // Computed — porcentaje de progreso del jugador local
  porcentajeProgreso = computed(() =>
    this.totalCaracteres() > 0
      ? Math.round((this.posicionLocal() / this.totalCaracteres()) * 100)
      : 0
  );

  // Lista de jugadores ordenada por posición (para la tabla)
  jugadoresOrdenados = computed(() =>
    [...this.jugadores().values()]
      .sort((a, b) => b.posicion - a.posicion)
  );

  private subs = new Subscription();

  constructor(
    private ws: CarreraWsService
  ) { }

  ngOnInit(): void {
    this.ws.conectar(this.salaId, this.usuario);
    this.conectado.set(true);
    this.suscribirse();
  }

  private suscribirse(): void {

    // Recibir texto de la carrera
    this.subs.add(
      this.ws.textoCarrera$().subscribe(evento => {
        this.textoCarrera.set(evento.texto);
        this.totalCaracteres.set(evento.caracteres);
      })
    );

    // Recibir progreso de otros jugadores
    this.subs.add(
      this.ws.progreso$().subscribe(evento => {
        // No procesar nuestro propio progreso (ya lo tenemos local)
        //if (evento.usuario === this.usuario) return;
        this.actualizarJugador(evento);
      })
    );

    // Un jugador terminó
    this.subs.add(
      this.ws.jugadorTermino$().subscribe(evento => {
        this.marcarTerminado(evento);
      })
    );

    // Jugador se unió
    this.subs.add(
      this.ws.jugadorUnido$().subscribe(evento => {
        console.log(`${evento.usuario} se unió. Total: ${evento.total_jugadores}`);
      })
    );

    // Jugador se fue
    this.subs.add(
      this.ws.jugadorSalio$().subscribe(evento => {
        const mapa = new Map(this.jugadores());
        mapa.delete(evento.usuario);
        this.jugadores.set(mapa);
      })
    );
  }

  // ─── Llamado en cada keyup del input ──────
  onTecla(event: KeyboardEvent): void {
    const input = (event.target as HTMLInputElement).value;
    const texto = this.textoCarrera();

    if (!this.carreraIniciada() && input.length === 1) {
      // Primera tecla — arrancar el cronómetro
      this.tiempoInicio = Date.now();
      this.carreraIniciada.set(true);
    }

    // Calcular posición y errores comparando con el texto esperado
    let correctos = 0;
    let errores = 0;
    for (let i = 0; i < input.length; i++) {
      if (input[i] === texto[i]) correctos++;
      else errores++;
    }

    this.posicionLocal.set(input.length);
    this.erroresLocales.set(errores);
    this.inputUsuario.set(input);

    // Enviar progreso al servidor cada tecla
    this.ws.enviar({
      tipo: 'progreso',
      posicion: input.length,
      errores,
      caracteres_correctos: correctos,
      tiempo_inicio_ms: this.tiempoInicio ?? Date.now(),
    });

    // Verificar si terminó
    if (input === texto) {
      this.terminar(correctos);
    }
  }

  private terminar(correctos: number): void {
    const tiempoSegundos = this.tiempoInicio
      ? Math.round((Date.now() - this.tiempoInicio) / 1000)
      : 0;

    this.ws.enviar({
      tipo: 'termino',
      tiempo_segundos: tiempoSegundos,
      errores: this.erroresLocales(),
      caracteres_correctos: correctos,
      total_caracteres: this.totalCaracteres(),
    });
  }

  private actualizarJugador(evento: EventoProgreso): void {
    const mapa = new Map(this.jugadores());
    mapa.set(evento.usuario, {
      usuario: evento.usuario,
      posicion: evento.posicion,
      errores: evento.errores,
      precision: evento.precision,
      wpm: evento.wpm,
      terminado: false,
    });
    this.jugadores.set(mapa);
  }

  private marcarTerminado(evento: EventoJugadorTermino): void {
    const mapa = new Map(this.jugadores());
    const jugador = mapa.get(evento.usuario) ?? {
      usuario: evento.usuario,
      posicion: this.totalCaracteres(),
      errores: 0,
      precision: evento.precision,
      wpm: evento.wpm,
      terminado: false,
    };
    mapa.set(evento.usuario, {
      ...jugador,
      terminado: true,
      tiempo_segundos: evento.tiempo_segundos,
      wpm: evento.wpm,
      precision: evento.precision,
    });
    this.jugadores.set(mapa);
  }

  ngOnDestroy(): void {
    this.subs.unsubscribe();
    this.ws.desconectar();
  }

  // Colorear caracteres: verde=correcto, rojo=error, gris=sin escribir
  // ⚠️ Este método debe existir en tu clase CarreraAlterna
  getClaseCaracter(index: number): string {
    const input = this.inputUsuario();
    if (index >= input.length) return 'char-pending';
    return input[index] === this.textoCarrera()[index]
      ? 'char-correcto'
      : 'char-error';
  }

}
