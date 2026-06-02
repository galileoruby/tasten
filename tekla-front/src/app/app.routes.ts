import { Routes } from '@angular/router';
export const routes: Routes = [
    {
        path: 'teclado',
        loadComponent: () => import('./componentes/keyboard/keyboard').then(m => m.Keyboard)
    },
    {
        path: 'carrera',
        loadComponent: () => import('./componentes/carrera-alterna/carrera-alterna').then(m => m.CarreraAlterna)
    },
    {
        path: '',
        loadComponent: () => import('./componentes/header/header').then(m => m.Header)
    }
];
