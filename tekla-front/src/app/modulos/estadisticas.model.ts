import { Errores } from "./errores.model";

export class Estadisticas {

    constructor(
        public teclaActual: string,
        public posicionActual: number,
        public totalErrores: number,
        public errorPorTecla: Errores[],
        public totalCaracteresLeccion: number,
        public caracteresCorrectos: number,
        public tiempoInicio: Date| null,
        public tiempoFin: Date| null
    ) {
    }
}