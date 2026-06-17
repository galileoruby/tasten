
import { Component, OnInit, OnDestroy, inject } from '@angular/core';
import { Carrera } from '../carrera/carrera';           // tu componente leccion
import { Keyboard } from '../keyboard/keyboard';
import { Estadisticas } from '../estadisticas/estadisticas';
import { CarreraStateService } from '../../services/carrera-state.service';

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
  standalone: true,
  imports: [
    Carrera,
    Keyboard,
    Estadisticas
  ],
  templateUrl: './carrera-alterna.html',
  styleUrl: './carrera-alterna.less',
})
export class CarreraAlterna implements OnInit, OnDestroy {
  private cs = inject(CarreraStateService);

  // Solo esta página maneja conectar/desconectar
  // Los componentes hijos solo leen signals
  ngOnInit(): void { this.cs.conectar(); }
  ngOnDestroy(): void { this.cs.desconectar(); }

}
