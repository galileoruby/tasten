import { Component, OnDestroy, OnInit, ViewEncapsulation } from '@angular/core';
import { ServicioTexto } from '../../services/servicio-texto';
import { Observable, Subscription } from 'rxjs';
import { DomSanitizer, SafeHtml } from '@angular/platform-browser';
import { FluidModule } from 'primeng/fluid';
import { DialogModule } from 'primeng/dialog';
import { ButtonModule } from 'primeng/button';
import { TableModule } from "primeng/table";
import { CommonModule } from '@angular/common';
@Component({
  selector: 'app-estadisticas',
  imports: [DialogModule, ButtonModule, CommonModule, TableModule],
  templateUrl: './estadisticas.html',
  styleUrl: './estadisticas.less',
})
export class Estadisticas {
  constructor(public servicioCarrera: ServicioTexto) {
    this.servicioCarrera = servicioCarrera;
  }

  // Propiedades para el Dialog
  mostrarEstadisticas: boolean = false;

    cerrarEstadisticas(): void {
    this.mostrarEstadisticas = false;
  }

    getEstadisticas() {
    return {
      precision: this.servicioCarrera.calcularPrecision(),
      errores: this.servicioCarrera.ContadorErrores,
      totalCaracteres: this.servicioCarrera.estadisticas.totalCaracteresLeccion,
      posicionActual: this.servicioCarrera.estadisticas.posicionActual,
      progreso: ((this.servicioCarrera.estadisticas.posicionActual / this.servicioCarrera.estadisticas.totalCaracteresLeccion) * 100).toFixed(1),
      erroresPorTecla: this.servicioCarrera.estadisticas.errorPorTecla
    };
  }
}
