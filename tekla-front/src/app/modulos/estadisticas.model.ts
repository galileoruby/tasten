import { Errores } from "./errores.model";

export class Estadisticas {

    constructor(
        public teclaActual: string,
        public posicionActual: number,
        public totalErrores: number,
        public errorPorTecla: Errores[]
    ) {
    }
}