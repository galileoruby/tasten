import { Component, OnDestroy, OnInit, ViewEncapsulation } from '@angular/core';
import { ServicioTexto } from '../../services/servicio-texto';
import { Observable, Subscription } from 'rxjs';
import { DomSanitizer, SafeHtml } from '@angular/platform-browser';
import { FluidModule } from 'primeng/fluid';
import { TooltipModule } from 'primeng/tooltip';

@Component({
  selector: 'app-carrera',
  imports: [FluidModule, TooltipModule],
  templateUrl: './carrera.html',
  styleUrl: './carrera.less',
  encapsulation: ViewEncapsulation.None
})
export class Carrera implements OnInit, OnDestroy {

  public servicioCarrera: ServicioTexto;
  textoDesplegado: SafeHtml = '';
  private subscription: Subscription = new Subscription();
  private posicionCambiadaSubscription: Subscription | null = null;

  constructor(
    servicioCarrera: ServicioTexto,
    private sanitizer: DomSanitizer  // Inyecta DomSanitizer) 
  ) {
    this.servicioCarrera = servicioCarrera;
    this.textoDesplegado = this.sanitizer.bypassSecurityTrustHtml(
      "<em>Cargando lección...</em>"
    );
  }

  ngOnDestroy(): void {

    // Limpiar suscripciones para evitar memory leaks
    if (this.subscription) {
      this.subscription.unsubscribe();
    }
    if (this.posicionCambiadaSubscription) {
      this.posicionCambiadaSubscription.unsubscribe();
    }
  }

  ngOnInit(): void {
    // Cargar la lección inicialmente
    this.cargarLeccion();

    // Suscribirse a cambios en la posición
    this.suscribirACambiosPosicion();
  }

  getStatsTooltip(): string {

    const erroresPorTecla = this.servicioCarrera.estadisticas.errorPorTecla
      .map(error => `${error.tecla}: ${error.totalError} errores`)
      .join('\n      ');

    return `
      📊 Estadísticas en tiempo real:      
      • Precisión: ${this.servicioCarrera.calcularPrecision()}%
      • Errores: ${this.servicioCarrera.ContadorErrores}
      • Caracteres: ${this.servicioCarrera.estadisticas.totalCaracteresLeccion}
      • Progreso: ${((this.servicioCarrera.estadisticas.posicionActual / this.servicioCarrera.estadisticas.totalCaracteresLeccion) * 100).toFixed(1)}%             
      📈 Errores por tecla:
          ${erroresPorTecla ? '      ' + erroresPorTecla : '      Ningún error registrado'}
    `.trim();
  }

  suscribirACambiosPosicion(): void {
    this.posicionCambiadaSubscription = this.servicioCarrera.posicionCambiada$.subscribe({
      next: (estado) => {
        // Actualizar el texto con estilos cada vez que cambia la posición
        this.actualizarTextoConEstilos();

        // Puedes usar también la información del estado si la necesitas
        if (estado.terminado) {
          console.log('¡Carrera terminada!');
        }
      },
      error: (error) => {
        console.error('Error en suscripción a cambios de posición:', error);
      }
    });
  }

  cargarLeccion(): void {
    this.subscription = this.servicioCarrera.cargarLeccion().subscribe({
      next: (leccionActual) => {
        // Actualizar texto inicial
        this.actualizarTextoConEstilos();
      },
      error: (error) => {
        this.textoDesplegado = "Error al cargar la lección. Intenta nuevamente.";
        console.error(error);
      }
    });
  }

  actualizarTextoConEstilos(): void {
    // Obtener el texto formateado con estilos del servicio
    // Obtener el texto formateado y sanitizarlo
    const textoHtml = this.servicioCarrera.obtenerTextoConEstilos();
    // Si el texto está vacío, mostrar mensaje
    if (!textoHtml || textoHtml.trim() === '') {
      this.textoDesplegado = this.sanitizer.bypassSecurityTrustHtml(
        "<em>Esperando texto...</em>"
      );
      return;
    }
    this.textoDesplegado = this.sanitizer.bypassSecurityTrustHtml(textoHtml);
  }
}