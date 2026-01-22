import { Injectable } from '@angular/core';
import { HttpClient, HttpErrorResponse } from '@angular/common/http';
import { BehaviorSubject, Observable, of, throwError } from 'rxjs';
import { catchError, retry, shareReplay, tap, timeout } from 'rxjs/operators';

export interface TextoResponse {
  parrafo1: string;
  parrafo2: string;
}

export interface TextoArrayResponse {
  parrafos: string[];
  total_parrafos: number;
  total_caracteres: number;
  idioma: string;
}


@Injectable({
  providedIn: 'root',
})
export class ServicioTexto {
  private readonly API_URL = 'http://127.0.0.1:8080'; // URL de tu API Rust


  /**
   * a.contar errores y relacionarlos a la tecla
   * b.contar milisegundos entre tecla a-b
   * 
   */


  private _carreraActual: string = "";
  private _carreraTerminada: boolean = false;
  private siguientePosicion: number = 0;
  private errores_tecleados: number = 0;


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
  private carreraCache$: Observable<string> | null = null;
  private isLoading = false;

  constructor(private http: HttpClient) { }


  public esCaracterValido(caracter: string): boolean {

    if (caracter == "") {
      return false;
    }

    if (this.siguientePosicion == this._carreraActual.length) {
      this._carreraTerminada = true;
      return true;
    }

    if (this.siguientePosicion > this._carreraActual.length) {
      return false;
    }

    if (this._carreraActual[this.siguientePosicion] !== caracter) {
      this.errores_tecleados++;
      return false;
    }

    this.siguientePosicion++;

    this.notificarCambioPosicion();
    return true;
  }


  private notificarCambioPosicion(): void {
    const caracterActual = this.siguientePosicion < this._carreraActual.length
      ? this._carreraActual[this.siguientePosicion]
      : null;

    this.posicionCambiadaSubject.next({
      posicion: this.siguientePosicion,
      caracterActual: caracterActual,
      textoCompleto: this._carreraActual,
      errores_tecleados: this.errores_tecleados,
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

      if (i === this.siguientePosicion) {
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




  cargarLeccion(): Observable<string> {

    if (this._carreraActual && this._carreraActual !== '') {
      return of(this._carreraActual);
    }

    if (this.carreraCache$) {
      return this.carreraCache$;
    }

    // Si no hay caché, hacemos la petición
    this.carreraCache$ = this.http.get<string>(`${this.API_URL}/texto`).pipe(
      retry(2),
      timeout(5000),
      tap(texto => {
        this._carreraActual = texto;
        this.leccionCargadaSubject.next(texto);
        this.isLoading = false;
        this.notificarCambioPosicion();
      }),
      catchError(error => {
        this.carreraCache$ = null;
        this.isLoading = false;
        return this.handleError(error);
      }),
      shareReplay(1) // Comparte la respuesta con todos los suscriptores
    );

    this.isLoading = true;
    return this.carreraCache$;
  }


  get EsCarreraTerminada(): boolean {
    return this._carreraTerminada;
  }

  get ContadorErrores(): number {
    return this.errores_tecleados;
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
