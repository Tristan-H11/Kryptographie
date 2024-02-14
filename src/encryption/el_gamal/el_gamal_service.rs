use crate::math_core::number_theory::number_theory_service::NumberTheoryService;

pub struct ElGamalService {
    number_theory_service: NumberTheoryService,
}

impl ElGamalService {
    pub fn new(number_theory_service: NumberTheoryService) -> ElGamalService {
        ElGamalService {
            number_theory_service,
        }
    }

    pub fn encrypt() {
        unimplemented!();
    }

    pub fn decrypt() {
        unimplemented!();
    }

    pub fn sign() {
        unimplemented!();
    }

    pub fn verify() {
        unimplemented!();
    }
}