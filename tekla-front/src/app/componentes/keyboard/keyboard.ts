import { Component, afterNextRender, ElementRef, viewChild, OnInit } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { InputTextModule } from 'primeng/inputtext';
import { FluidModule } from 'primeng/fluid';
import { ServicioTexto } from '../../services/servicio-texto';

@Component({
  selector: 'app-keyboard',
  imports: [
    InputTextModule,
    FormsModule,
    FluidModule],
  templateUrl: './keyboard.html',
  styleUrl: './keyboard.less',
})
export class Keyboard implements OnInit {

  _servicio: ServicioTexto;
  value = "";
  lastKeyPressed = '';
  esValido: boolean | null = null;
  keyHistory: string[] = [];

  // Contadores de teclas especiales
  enterCount = 0;
  escapeCount = 0;
  tabCount = 0;


  // Historial de tiempos entre teclas
  private _tiemposEntreTeclas: number[] = [];
  private _ultimoTimeStamp: number = 0;

  // Propiedades para mostrar
  tiempoUltimaTecla: number = 0;
  tiempoPromedio: number = 0;
  velocidadActual: number = 0;

  inputRef = viewChild<ElementRef<HTMLInputElement>>('inputFocus');

  private _patronActual: number[] = [];


  constructor(servicioTexto: ServicioTexto) {
    this._servicio = servicioTexto;
    afterNextRender(() => {
      // Este código solo corre en el navegador después del primer renderizado
      this.inputRef()?.nativeElement.focus();
    });
  }


  ngOnInit(): void {
    // Opción 1: Suscribirse directamente
    this._servicio.cargarLeccion().subscribe({
      next: (texto) => {         
      },
      error: (error) => {
        console.error('Error:', error);
         
      }
    });


    this.cargarLeccionAsync();
  }

  async cargarLeccionAsync(): Promise<void> {
    try {
      const texto = this._servicio.cargarLeccion();
    } catch (error) {
      console.error('Error async:', error);
    }
  }

  // 1. Método básico para keydown
  onKeyDown(event: KeyboardEvent) {
    // console.log('KeyDown:', {
    //   key: event.key,
    //   code: event.code,
    //   ctrlKey: event.ctrlKey,
    //   shiftKey: event.shiftKey,
    //   altKey: event.altKey
    // });

    const ahora = performance.now();

    this.lastKeyPressed = event.key;
    this.keyHistory.push(event.key);

    // console.log("lastKeyPressed:",this.lastKeyPressed);

    // Limitar historial a 10 teclas
    if (this.keyHistory.length > 10) {
      this.keyHistory.shift();
      this.value = "";

    }

    // Combinaciones de teclas comunes
    if (event.ctrlKey && event.key === 'c') {
      // this.showMessage('success', 'Copiado al portapapeles');
      // console.log('kopiado');
    }

    if (event.ctrlKey && event.key === 'v') {
      // this.showMessage('info', 'Pegado desde portapapeles');
      // console.log('pegado');
    }

    // Prevenir comportamiento por defecto para algunas teclas
    if (event.key === 'Tab') {
      this.tabCount++;
      // event.preventDefault(); // Descomenta si quieres evitar el tab
    }


    this.esValido = this._servicio.esCaracterValido(event.key);


    if (this._ultimoTimeStamp > 0) {
      // Calcular tiempo entre teclas
      const tiempoEntre = ahora - this._ultimoTimeStamp;

      // Guardar tiempos
      this.tiempoUltimaTecla = tiempoEntre;
      this._tiemposEntreTeclas.push(tiempoEntre);

      // Mantener solo los últimos 50 tiempos
      if (this._tiemposEntreTeclas.length > 50) {
        this._tiemposEntreTeclas.shift();
      }

      // Calcular promedio
      this.calcularPromedio();

      // Calcular velocidad actual (media móvil de últimos 5)
      this.calcularVelocidadActual();

      // Detectar patrones de escritura
      this.detectarPatron(tiempoEntre);

      // Mostrar feedback visual basado en velocidad
      this.mostrarFeedbackVelocidad(tiempoEntre);
    }

    this._ultimoTimeStamp = ahora;

    // Tu lógica existente...
    this.lastKeyPressed = event.key;
    this.keyHistory.push(event.key);

    if (this.keyHistory.length > 10) {
      this.keyHistory.shift();
    }
  }




  // 2. Método para keyup
  onKeyUp(event: KeyboardEvent) {
    // console.log('KeyUp:', event.key);

    // Contar teclas especiales
    switch (event.key) {
      case 'Enter':
        this.enterCount++;
        this.onEnterKey();
        break;
      case 'Escape':
        this.escapeCount++;
        this.onEscapeKey();
        break;
    }
  }

  // 3. Método para keypress (deprecated en algunos casos, pero aún usable)
  onKeyPress(event: KeyboardEvent) {
    // keypress solo para teclas que producen caracteres
    // console.log('KeyPress - Carácter:', event.key);

    // Validar entrada
    // if (!this.isValidCharacter(event.key)) {
    //   event.preventDefault();       
    // }
  }

  // 4. Métodos específicos para teclas
  onEnterKey() {
    // console.log('Enter presionado', this.enterCount, 'veces');
    // this.showMessage('success', `Formulario enviado (Enter #${this.enterCount})`);

    // Lógica cuando se presiona Enter
    // this.submitForm();
  }

  onEscapeKey() {
    // console.log('Escape presionado', this.escapeCount, 'veces');
    this.value = ''; // Limpiar input
    // this.showMessage('info', 'Campo limpiado (Escape)');
  }

  // 5. Métodos de utilidad
  private isValidCharacter(char: string): boolean {
    // Solo letras, números y espacios
    return /^[a-zA-Z0-9\s]$/.test(char);
  }

  clearHistory() {
    this.keyHistory = [];
    this.enterCount = 0;
    this.escapeCount = 0;
    this.tabCount = 0;
  }


  getInputClasses(): string {
    if (this.esValido === null) {
      return 'text-mono';
    }
    return this.esValido ? 'text-green-mono bordered' : 'text-red-mono bordered';
  }


  private calcularPromedio(): void {
    if (this._tiemposEntreTeclas.length === 0) {
      this.tiempoPromedio = 0;
      return;
    }

    const suma = this._tiemposEntreTeclas.reduce((a, b) => a + b, 0);
    this.tiempoPromedio = suma / this._tiemposEntreTeclas.length;
  }

  private calcularVelocidadActual(): void {
    if (this._tiemposEntreTeclas.length < 5) {
      this.velocidadActual = this.tiempoUltimaTecla;
      return;
    }

    const ultimos5 = this._tiemposEntreTeclas.slice(-5);
    this.velocidadActual = ultimos5.reduce((a, b) => a + b, 0) / 5;
  }

  private detectarPatron(tiempo: number): void {
    this._patronActual.push(tiempo);

    if (this._patronActual.length > 10) {
      this._patronActual.shift();

      // Aquí podrías añadir lógica para detectar patrones específicos
      // Ejemplo: detectar si el usuario está escribiendo con ritmo constante
      const variacion = this.calcularVariacion(this._patronActual);

      if (variacion < 0.3) { // Menos del 30% de variación
        console.log('Patrón estable detectado');
      }
    }
  }

  private calcularVariacion(tiempos: number[]): number {
    const promedio = tiempos.reduce((a, b) => a + b) / tiempos.length;
    const varianza = tiempos.reduce((a, b) => a + Math.pow(b - promedio, 2), 0) / tiempos.length;
    return Math.sqrt(varianza) / promedio;
  }

  private mostrarFeedbackVelocidad(tiempo: number): void {
    // Clasificar la velocidad
    if (tiempo < 100) {
      // Muy rápido (menos de 100ms)
      this.aplicarEstilo('rapido');
    } else if (tiempo < 300) {
      // Normal (100-300ms)
      this.aplicarEstilo('normal');
    } else {
      // Lento (más de 300ms)
      this.aplicarEstilo('lento');
    }
  }

  private aplicarEstilo(tipo: 'rapido' | 'normal' | 'lento'): void {
    // Puedes implementar cambios visuales aquí
    console.log(`Velocidad: ${tipo}`);
  }

  // Getter para estadísticas
  get estadisticas(): any {
    return {
      tiempoActual: this.tiempoUltimaTecla,
      promedio: this.tiempoPromedio,
      velocidadActual: this.velocidadActual,
      totalMuestras: this._tiemposEntreTeclas.length,
      esRapido: this.tiempoUltimaTecla < 150,
      esLento: this.tiempoUltimaTecla > 500
    };
  }

  // Reiniciar mediciones
  reiniciarTiempos(): void {
    this._tiemposEntreTeclas = [];
    this._ultimoTimeStamp = 0;
    this.tiempoUltimaTecla = 0;
    this.tiempoPromedio = 0;
    this.velocidadActual = 0;
  }



  getColorForTime(tiempo: number): string {
    if (tiempo < 100) return '#10b981'; // Verde para rápido
    if (tiempo < 300) return '#f59e0b'; // Amarillo para normal
    return '#ef4444'; // Rojo para lento
  }

  getHeightForTime(tiempo: number): number {
    // Normalizar altura entre 20% y 100%
    const minTime = 50;  // ms
    const maxTime = 1000; // ms
    const normalized = Math.min(Math.max(tiempo, minTime), maxTime);
    return 20 + (80 * (normalized - minTime) / (maxTime - minTime));
  }

  // Para calcular palabras por minuto (WPM)
  calcularWPM(): number {
    if (this._tiemposEntreTeclas.length < 10) return 0;

    const ultimoMinuto = this._tiemposEntreTeclas.filter(t => t < 1000);
    if (ultimoMinuto.length === 0) return 0;

    const tiempoTotal = ultimoMinuto.reduce((a, b) => a + b, 0);
    const caracteresPorMinuto = (ultimoMinuto.length / tiempoTotal) * 60000;

    // Asumir 5 caracteres por palabra
    return caracteresPorMinuto / 5;
  }
}