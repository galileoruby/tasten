import { Component, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { CardModule } from 'primeng/card';
import { TableModule } from 'primeng/table';
import { BadgeModule } from 'primeng/badge';
import { ProgressBarModule } from 'primeng/progressbar';
import { CarreraStateService, JugadorSala } from '../../services/carrera-state.service';

@Component({
  selector: 'app-usuarios-estadisticas',
  standalone: true,
  imports: [
    CommonModule,
    CardModule,
    TableModule,
    BadgeModule,
    ProgressBarModule
  ],
  templateUrl: './usuarios-estadisticas.html',
  styleUrl: './usuarios-estadisticas.less'
})
export class UsuariosEstadisticas {
  private readonly cs = inject(CarreraStateService);

  get otrosUsuarios() {
    return [...this.cs.jugadores().values()]
      .filter((jugador) => jugador.usuario !== this.cs.usuario)
      .sort((a, b) => b.posicion - a.posicion);
  }

  progresoPercent(posicion: number): number {
    const total = this.cs.totalCaracteres();
    if (total <= 0) return 0;

    return Math.min(100, Math.round((posicion / total) * 100));
  }

  estadoJugador(jugador: JugadorSala): string {
    if (jugador.abandonado) return 'Abandon';
    return jugador.terminado ? 'Terminado' : 'En curso';
  }

  severidadJugador(jugador: JugadorSala): 'info' | 'success' | 'warn' | 'danger' | 'secondary' | 'contrast' | null | undefined {
    if (jugador.abandonado) return 'warn';
    return jugador.terminado ? 'success' : 'info';
  }
}