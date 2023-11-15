import { TestBed } from '@angular/core/testing';

import { MessageManagementService } from './message-management.service';

describe('MessageManagementService', () => {
  let service: MessageManagementService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(MessageManagementService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
