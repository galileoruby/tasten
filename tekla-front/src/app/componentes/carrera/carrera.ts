import { Component, ViewEncapsulation, inject, effect } from '@angular/core';
import { DomSanitizer, SafeHtml } from '@angular/platform-browser';
import { FluidModule } from 'primeng/fluid';
import { CarreraStateService } from '../../services/carrera-state.service';
import { Badge } from "primeng/badge";

@Component({
  selector: 'app-carrera',
  imports: [FluidModule],
  templateUrl: './carrera.html',
  styleUrl: './carrera.less',
  encapsulation: ViewEncapsulation.None,
})
export class Carrera {
  private readonly cs = inject(CarreraStateService);
  private readonly sanitizer = inject(DomSanitizer);

  textoDesplegado: SafeHtml = this.sanitizer.bypassSecurityTrustHtml(
    '<em>Cargando lección...</em>'
  );

  constructor() {
    effect(() => {
      this.actualizarTextoConEstilos();
    });
  }

  actualizarTextoConEstilos(): void {
    const texto = this.cs.textoCarrera();
    const posicion = this.cs.posicionLocal();

    if (!texto || texto.trim() === '') {
      this.textoDesplegado = this.sanitizer.bypassSecurityTrustHtml(
        '<em>Esperando texto...</em>'
      );
      return;
    }

    const html = Array.from(texto).map((char, i) => {
      if (i < posicion) {
        return `<span class="caracter-correcto">${this.escapeHtml(char)}</span>`;
      }
      if (i === posicion) {
        return `<span class="caracter-actual">${this.escapeHtml(char)}</span>`;
      }
      return this.escapeHtml(char);
    }).join('');

    this.textoDesplegado = this.sanitizer.bypassSecurityTrustHtml(html);
  }

  private escapeHtml(value: string): string {
    return value
      .replaceAll('&', '&amp;')
      .replaceAll('<', '&lt;')
      .replaceAll('>', '&gt;')
      .replaceAll('"', '&quot;')
      .replaceAll("'", '&#39;');
  }
}