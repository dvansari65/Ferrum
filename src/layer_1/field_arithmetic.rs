use primitive_types::U256;

const P: U256 = U256([
    0x3c208c16d87cfd47,
    0x97816a916871ca8d,
    0xb85045b68181585d,
    0x30644e72e131a029,
]);
pub struct Fp {
    pub value: U256,
}

impl Fp {
    pub fn add(&self, other: &Fp) -> Self {
        return Self {
            value: (self.value + other.value) % P,
        };
    }
    pub fn new(value: U256) -> Self {
        Self { value: value % P }
    }
    pub fn mult(&self, other: &Fp) -> Self {
        Self {
            value: (self.value * other.value) % P,
        }
    }
}
