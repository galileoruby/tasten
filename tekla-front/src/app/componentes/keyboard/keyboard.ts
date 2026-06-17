import { Component, afterNextRender, ElementRef, viewChild, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputTextModule } from 'primeng/inputtext';
import { FluidModule } from 'primeng/fluid';
import { CarreraStateService } from '../../services/carrera-state.service';

@Component({
  selector: 'app-keyboard',
  imports: [InputTextModule, FormsModule, FluidModule],
  templateUrl: './keyboard.html',
  styleUrl: './keyboard.less',
})
export class Keyboard {

  // Reemplaza ServicioTexto — ahora lee del estado WS
  cs = inject(CarreraStateService);

  // Mantener value para el input (no se usa para lógica, solo para limpiar)
  value = '';

  inputRef = viewChild<ElementRef<HTMLInputElement>>('inputFocus');

  constructor() {
    afterNextRender(() => {
      this.inputRef()?.nativeElement.focus();
    });
  }

  onKeyDown(event: KeyboardEvent): void {
    // Ignorar teclas especiales
    if (event.key === 'Escape') {
      this.value = '';
      return;
    }
    if (event.key === 'Tab') {
      event.preventDefault();
      return;
    }
    if (event.key.length > 1) return; // Shift, Control, etc.

    // Toda la lógica va al servicio — él actualiza signals y envía al WS
    this.cs.procesarTecla(event.key);
  }

  onInput(e: Event): void {
    const valor = (e.target as HTMLInputElement).value;
    this.value = valor;
  }

  getInputClasses(): string {
    const pos = this.cs.posicionLocal();
    const texto = this.cs.textoCarrera();
    if (pos === 0 || !texto) return 'text-mono';

    // Verificar si el último carácter fue correcto
    const ultimoChar = this.value[this.value.length - 1];
    const esperado = texto[pos - 1];
    return ultimoChar === esperado
      ? 'text-green-mono bordered'
      : 'text-red-mono bordered';
  }
}