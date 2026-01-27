import { Component } from '@angular/core';
import { TabsModule } from 'primeng/tabs';

import { Keyboard } from '../keyboard/keyboard';
import { BreadcrumbModule } from 'primeng/breadcrumb';
import { MenuItem } from 'primeng/api';
import { Carrera } from '../carrera/carrera'; 

@Component({
  selector: 'app-header',
  imports: [
    
    TabsModule,
    Keyboard,
    BreadcrumbModule,
    Carrera],
  templateUrl: './header.html',
  styleUrl: './header.less',
})
export class Header {

  items: MenuItem[] = [
    { label: 'Productos', routerLink: '/productos' },
    { label: 'Electrónica', routerLink: '/productos/electronica' },
    { label: 'Computadoras', routerLink: '/productos/electronica/computadoras' },
    { label: 'Laptop HP', routerLink: '/productos/electronica/computadoras/123' }
  ];
  homeWithIcon: MenuItem = {
    icon: 'pi pi-home',
    routerLink: '/',
    title: 'Inicio'
  };
}
