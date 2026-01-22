import { TestBed } from '@angular/core/testing';

import { ServicioTexto } from './servicio-texto';

describe('ServicioTexto', () => {
  let service: ServicioTexto;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(ServicioTexto);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
