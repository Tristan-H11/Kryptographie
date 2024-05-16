import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MenezesVanstoneSingleComponent } from './menezes-vanstone-single.component';

xdescribe('MenezesVanstoneSingleComponent', () => {
  let component: MenezesVanstoneSingleComponent;
  let fixture: ComponentFixture<MenezesVanstoneSingleComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MenezesVanstoneSingleComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(MenezesVanstoneSingleComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  xit('should create', () => {
    expect(component).toBeTruthy();
  });
});
