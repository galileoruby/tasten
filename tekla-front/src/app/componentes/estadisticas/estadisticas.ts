import { Component, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Card, CardModule } from 'primeng/card';
import { Divider, DividerModule } from 'primeng/divider';
import { BadgeModule } from 'primeng/badge';
import { TooltipModule } from 'primeng/tooltip';
import { ProgressBarModule } from 'primeng/progressbar';
import { CarreraStateService } from '../../services/carrera-state.service';

@Component({
  selector: 'app-estadisticas',
  standalone: true,
  imports: [
    CommonModule, Card, Divider,
    BadgeModule, TooltipModule, ProgressBarModule,
  ],
  templateUrl: './estadisticas.html',
  styleUrl: './estadisticas.less',
})
export class Estadisticas {

  // Un solo inject — sin constructor, sin Subscription
  cs = inject(CarreraStateService);

  // Computed local para el template de errores por tecla
  get errorKeys(): { key: string; count: number }[] {
    return this.cs.errorPorTecla()
      .map(e => ({ key: e.tecla, count: e.cantidad }));
  }
}