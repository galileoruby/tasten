import { EMPTY, Subject } from 'rxjs';
import { CarreraStateService } from './carrera-state.service';
import { CarreraWsService } from './carrera-ws.service';

describe('CarreraStateService', () => {
  let service: CarreraStateService;
  let jugadorUnidoSubject: Subject<{ usuario: string; total_jugadores: number }>;
  let jugadorTerminoSubject: Subject<any>;

  beforeEach(() => {
    jugadorUnidoSubject = new Subject<{ usuario: string; total_jugadores: number }>();
    jugadorTerminoSubject = new Subject<any>();

    const wsMock = {
      conectar: jasmine.createSpy('conectar'),
      desconectar: jasmine.createSpy('desconectar'),
      enviar: jasmine.createSpy('enviar'),
      textoCarrera$: () => EMPTY,
      progreso$: () => EMPTY,
      jugadorTermino$: () => jugadorTerminoSubject.asObservable(),
      jugadorUnido$: () => jugadorUnidoSubject.asObservable(),
      jugadorSalio$: () => EMPTY,
    };

    service = new CarreraStateService(wsMock as unknown as CarreraWsService);
    service.conectar();
  });

  it('should add a joining user to the players list before any typing progress is sent', () => {
    jugadorUnidoSubject.next({ usuario: 'bob', total_jugadores: 2 });

    expect(service.jugadores().get('bob')?.usuario).toBe('bob');
    expect(service.jugadores().get('bob')?.posicion).toBe(0);
    expect(service.jugadores().get('bob')?.terminado).toBeFalse();
  });

  it('should consume the new finish payload with per-key errors', () => {
    jugadorTerminoSubject.next({
      usuario: 'alex',
      tiempo_segundos: 12,
      precision: 88,
      wpm: 42,
      posicion_ranking: 1,
      errores: [
        { tecla: 'a', cantidad: 2 },
        { tecla: 's', cantidad: 1 },
      ],
    });

    expect(service.errorPorTecla()).toEqual([
      { tecla: 'a', cantidad: 2 },
      { tecla: 's', cantidad: 1 },
    ]);
    expect(service.jugadores().get('alex')?.errores).toBe(3);
  });
});
