use crate::state::FieldElement;

pub struct Polynomial {
    pub coefficients: Vec<FieldElement>,
}
// [2,1,2,4,5,6]
// 2*x^0 + 1*x^1 + 2*x^2 + 4*x^3 + 5*x^4 + 6*x^5
// 2 + x + 2x^2 + 4x^3 + 5x^4 + 6x^5

impl Polynomial {
    pub fn evaluate(&self, x: &FieldElement) -> FieldElement {
        let mut result = FieldElement::zero();

        for coefficient in self.coefficients.iter().rev() {
            result = result * x.clone() + coefficient.clone();
        }
        result
    }
    pub fn new(coefficients: Vec<FieldElement>) -> Self {
        Self { coefficients }
    }
    pub fn add(&self, rhs: &Self) -> Self {
        let max_len = self.coefficients.len().max(rhs.coefficients.len());
        let mut result = Vec::with_capacity(max_len);
        for i in 0..max_len {
            let a = self.coefficients.get(i);
            let b = rhs.coefficients.get(i);

            let value = match (a, b) {
                (Some(a), Some(b)) => a + b,
                (Some(a), None) => FieldElement { value: a.value },
                (None, Some(b)) => FieldElement { value: b.value },
                (None, None) => unreachable!(),
            };
            result.push(value);
        }
        Self {
            coefficients: result,
        }
    }
    pub fn sub(&self, rhs: &Self) -> Self {
        let max_len = self.coefficients.len().max(rhs.coefficients.len());
        let mut result = Vec::with_capacity(max_len);
        for i in 0..max_len {
            let a = self.coefficients.get(i);
            let b = rhs.coefficients.get(i);
            let value = match (a, b) {
                (Some(a), Some(b)) => a - b,
                (Some(a), None) => *a,
                (None, Some(b)) => *b,
                (None, None) => unreachable!(),
            };
            result.push(value);
        }
        Self {
            coefficients: result,
        }
    }
}
