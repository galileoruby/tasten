import { ComponentFixture, TestBed } from '@angular/core/testing';

import { CarreraAlterna } from './carrera-alterna';

describe('CarreraAlterna', () => {
  let component: CarreraAlterna;
  let fixture: ComponentFixture<CarreraAlterna>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [CarreraAlterna]
    })
    .compileComponents();

    fixture = TestBed.createComponent(CarreraAlterna);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
