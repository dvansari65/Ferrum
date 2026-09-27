use std::ops::{Add, Mul, Neg, Sub};

use bls12_381::Scalar;

#[derive(Clone, Debug, PartialEq, Eq,Copy)]
pub struct FieldElement {
    pub value: Scalar,
}
impl FieldElement {
    pub fn new(num: Scalar) -> Self {
        Self { value: num }
    }
    pub fn zero() -> Self {
        Self { value: 0.into() }
    }
    pub fn inverse(&self) -> Option<FieldElement> {
        self.value
            .invert()
            .into_option()
            .map(|value| FieldElement { value })
    }
    pub fn pow(&self, expo: u64) -> Self {
        // concern: if expo becomes too large then this operation will become expensive
        let mut result = Scalar::one();
        for _ in 0..expo {
            result += self.value;
        }
        Self { value: result }
    }
    pub fn equality(&self, rhs: &Self) -> bool {
        self.value == rhs.value
    }
    pub fn checked_div(&self, rhs: &Self) -> Option<FieldElement> {
        rhs.value
            .invert()
            .into_option()
            .map(|inverse| FieldElement {
                value: self.value * inverse,
            })
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;
    fn mul(self, rhs: Self) -> FieldElement {
        FieldElement {
            value: self.value * rhs.value,
        }
    }
}
impl Add for FieldElement {
    type Output = FieldElement;
    fn add(self, rhs: Self) -> FieldElement {
        FieldElement {
            value: self.value + rhs.value,
        }
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;
    fn sub(self, rhs: Self) -> FieldElement {
        FieldElement {
            value: self.value - rhs.value,
        }
    }
}

impl Neg for FieldElement {
    type Output = FieldElement;
    fn neg(self) -> FieldElement {
        FieldElement { value: -self.value }
    }
}


macro_rules! forward_ref_binop {
    ($trait:ident,$method:ident,$t:ty) => {
        impl<'a,'b> $trait<&'b $t> for &'a $t {
            type Output = $t;
            fn $method(self , rhs:&'b $t) -> $t{
                $trait::$method(*self, *rhs)
            }
        }
        impl<'a> $trait<$t> for &'a $t {
            type Output = $t;
            fn $method(self,rhs:$t) -> $t {
                $trait::$method(*self , rhs)
            }
        }
        impl<'a> $trait<&'a $t> for $t {
            type Output = $t;
            fn $method(self,rhs:&'a $t) -> $t {
                $trait::$method(self , *rhs)
            }
        }
    };
}

forward_ref_binop!(Add,add, FieldElement);
forward_ref_binop!(Mul,mul, FieldElement);
forward_ref_binop!(Sub,sub, FieldElement);