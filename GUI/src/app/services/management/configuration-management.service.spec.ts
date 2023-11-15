import { TestBed } from '@angular/core/testing';

import { ConfigurationManagementService } from './configuration-management.service';

describe('ConfigurationManagementService', () => {
  let service: ConfigurationManagementService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(ConfigurationManagementService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
