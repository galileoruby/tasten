import { Injectable } from '@angular/core';
import { HttpClient, HttpErrorResponse } from '@angular/common/http';
import { BehaviorSubject, Observable, of, throwError } from 'rxjs';
import { catchError, retry, shareReplay, tap, timeout } from 'rxjs/operators';
import { Estadisticas } from '../modulos/estadisticas.model';
import { Errores } from '../modulos/errores.model';
import { date } from '@primeng/themes/aura/datepicker';
import { mapToCanActivate } from '@angular/router';
import { LeccionRequest } from '../modulos/leccion.model';

@Injectable({
  providedIn: 'root',
})
export class ServicioTexto {
  private readonly API_URL = 'http://localhost:3000'; // URL de tu API Rust


  /**
   * x.contar errores y relacionarlos a la tecla
   * a.contar milisegundos entre tecla a-b
   * b.clasificar en velocidad bajo, medio , alto,super alto
   * 
   */


  private _carreraActual: string = "";

  // Agrega esta variable junto a _carreraActual
  private _leccionActual: LeccionRequest | null = null;
  private _carreraTerminada: boolean = false;
  public estadisticas: Estadisticas;

  constructor(
    private http: HttpClient
  ) {
    this.estadisticas = new Estadisticas('', 0, 0, [], 0, 0, null, null);
  }


  public iniciarCronometro(): void {
    this.estadisticas.tiempoInicio = new Date();
    this.estadisticas.tiempoFin = null;
  }

  public detenerCronometro(): void {
    this.estadisticas.tiempoFin = new Date();
  }

  private getTiempoTranscurridoSegundos(): number {
    if (!this.estadisticas.tiempoInicio) return 0;

    const fin = this.estadisticas.tiempoFin || new Date();
    const diferenciaMs = fin.getTime() - this.estadisticas.tiempoInicio.getTime();
    return diferenciaMs / 1000;
  }

  public calcularPPM(): number {
    const segundos = this.getTiempoTranscurridoSegundos();
    const minutos = segundos / 60;
    if (minutos <= 0) return 0;
    const palabras = this.estadisticas.caracteresCorrectos / 5;
    return Math.round((palabras / minutos) * 100) / 100

  }

  private calcularPPMNet(): number {
    const segundos = this.getTiempoTranscurridoSegundos();
    const minutos = segundos / 60;
    if (minutos <= 0) return 0;

    const palabras = this.estadisticas.caracteresCorrectos / 5;
    const ppmNet = (palabras - this.estadisticas.totalErrores) / minutos;
    return ppmNet > 0 ? Math.round(ppmNet * 100) / 100 : 0;
  }

  public calcularPrecision(): number {
    const precision = (this.estadisticas.caracteresCorrectos / this.estadisticas.totalCaracteresLeccion) * 100;
    console.log(`caracteresCorrectos: ${this.estadisticas.caracteresCorrectos}, totalCaracteresLeccion:${this.estadisticas.totalCaracteresLeccion}, precision: ${precision}`)
    if (precision < 0) return 0;
    return Math.round(precision * 100) / 100;
  }


  // Subject para notificar cuando se carga una nueva lección
  private leccionCargadaSubject = new BehaviorSubject<string>('');
  leccionCargada$ = this.leccionCargadaSubject.asObservable();


  // Subject para notificar cambios en la posición (cuando esTextoValido es llamado)
  private posicionCambiadaSubject = new BehaviorSubject<{
    posicion: number;
    caracterActual: string | null;
    textoCompleto: string;
    errores_tecleados: number;
    terminado: boolean;
  }>({
    posicion: 0,
    caracterActual: null,
    textoCompleto: '',
    errores_tecleados: 0,
    terminado: false
  });

  posicionCambiada$ = this.posicionCambiadaSubject.asObservable();


  // Subject para cachear la respuesta de la API
  private carreraCache$: Observable<LeccionRequest> | null = null;
  private isLoading = false;

  public registrarCaracter(caracter: string): boolean {
    if (caracter == "") {
      return false;
    }

    if (this.estadisticas.posicionActual == this.estadisticas.totalCaracteresLeccion - 1) {
      this.detenerCronometro();
      this._carreraTerminada = true;
      return true;
    }

    if (this.estadisticas.posicionActual > this._leccionActual?.caracteres!) {
      return false;
    }

    if (this._carreraActual[this.estadisticas.posicionActual] !== caracter) {
      this.registrarErrorPorTecla(caracter);
      return false;
    }

    this.estadisticas.teclaActual = caracter;
    this.estadisticas.posicionActual++;
    this.estadisticas.caracteresCorrectos++;

    this.notificarCambioPosicion();
    return true;
  }

  private registrarErrorPorTecla(tecla: string): void {
    const teclaNormalizada = tecla.trim();

    if (!teclaNormalizada) {
      console.warn("Tecla vacia recibida");
      return;
    }
    const errorActual = new Errores(tecla, 1);
    this.estadisticas.caracteresCorrectos--;
    this.estadisticas.totalErrores++;

    const existeError = this.estadisticas.errorPorTecla.find(ab => ab.tecla == tecla);
    if (existeError) {
      existeError.totalError++;
    } else {
      const nuevoError = new Errores(teclaNormalizada, 1);
      this.estadisticas.errorPorTecla.push(errorActual);
    }
  }


  private notificarCambioPosicion(): void {
    const caracterActual = this.estadisticas.posicionActual < this._carreraActual.length
      ? this._carreraActual[this.estadisticas.posicionActual]
      : null;
    this.posicionCambiadaSubject.next({
      posicion: this.estadisticas.posicionActual,
      caracterActual: caracterActual,
      textoCompleto: this._carreraActual,
      errores_tecleados: this.estadisticas.totalErrores,
      terminado: this._carreraTerminada
    });
  }



  // Método para obtener el texto con estilos HTML
  obtenerTextoConEstilos(): string {
    if (!this._carreraActual || this._carreraActual === '') {
      return '';
    }

    let resultado = '';

    for (let i = 0; i < this._carreraActual.length; i++) {
      const caracter = this._carreraActual[i];

      if (i === this.estadisticas.posicionActual) {
        // Caracter actual (el que debe escribirse)
        resultado += `<span class="caracter-actual">${caracter}</span>`;
        // } else if (i < this.siguientePosicion) {
        //   // Caracteres ya escritos

        //   resultado += `<span class="${'caracter-actual'}">${caracter}</span>`;
      }
      else {
        // Caracteres por escribir
        resultado += caracter;
      }
    }


    return resultado;
  }




  cargarLeccion(): Observable<LeccionRequest> {

    if (this._leccionActual) {
      return of(this._leccionActual);
    }

    if (this.carreraCache$) {
      return this.carreraCache$;
    }

    // Si no hay caché, hacemos la petición
    this.isLoading = true;

    this.carreraCache$ = this.http.get<LeccionRequest>(`${this.API_URL}/api/leccion`).pipe(
      retry(2),
      timeout(5000),
      tap(leccion => {
        this._carreraActual = leccion.texto as string;
        this.estadisticas.totalCaracteresLeccion = leccion.caracteres;
        
        this.leccionCargadaSubject.next(leccion.texto as string);
        this.isLoading = false;
        this.notificarCambioPosicion();
      }),
      catchError(error => {
        this.carreraCache$ = null;
        this.isLoading = false;
        return this.handleError(error);
      }),
      shareReplay(1)
    );

    this.isLoading = true;
    return this.carreraCache$;
  }


  get EsCarreraTerminada(): boolean {
    return this._carreraTerminada;
  }

  get ContadorErrores(): number {
    return this.estadisticas.totalErrores;
  }



  private handleError(error: HttpErrorResponse) {
    let errorMessage = 'Error desconocido';

    if (error.error instanceof ErrorEvent) {
      // Error del lado del cliente
      errorMessage = `Error: ${error.error.message}`;
    } else {
      // Error del lado del servidor
      switch (error.status) {
        case 0:
          errorMessage = 'No se pudo conectar con el servidor. Verifica que la API Rust esté ejecutándose.';
          break;
        case 404:
          errorMessage = 'Endpoint no encontrado. Verifica la URL.';
          break;
        case 500:
          errorMessage = 'Error interno del servidor Rust.';
          break;
        default:
          errorMessage = `Error ${error.status}: ${error.message}`;
      }
    }
    return throwError(() => new Error(errorMessage));
  }
}