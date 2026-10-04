#[cfg(test)]
mod tests {
    use bls12_381::Scalar;

    use crate::{polynomial::polynomial::Polynomial, state::FieldElement};

    fn fe(n: u64) -> FieldElement {
        FieldElement::new(Scalar::from(n))
    }
    #[test]
    fn mul_two_linear_polynomial() {
        /*
        array 1 =  [2,3];
        array 2 = [1,5];
        mult = 2(1,5) + 3(1,5)
        ans = 2 + 10x + 3x + 15x^2
        ans = 2 + 13x + 15x^2
        new array = [2,13,15]
        */
        let a = Polynomial::new(vec![fe(2), fe(3)]);
        let b = Polynomial::new(vec![fe(1), fe(5)]);
        let product = a.mul(&b);
        
        let expexted = vec![fe(2),fe(13),fe(15)];
        assert_eq!(product.coefficients , expexted)
    }
    #[test]
    fn mul_two_linear_poly_for_three_degree(){
        let a = Polynomial::new(vec![fe(2),fe(3),fe(10)]);
        let b = Polynomial::new(vec![fe(2),fe(3),fe(10)]);
        /*
            polynomials = (2,3,10)(2,3,10);
            = 2(2 + 3x + 10x^2) + 3x(2 + 3x + 10x^2) + 10x^2 (2 + 3x + 10x^2);
            = 4 + 6x + 20x^2 + 6x + 9x^2 + 30x^3 + 20x^2 + 30x^3 + 100x^4;
            = 4 + 12x + 49x^2 + 60x^3 + 100x^4
            = [4,12,49,60,100];
         */
        let product = a.mul(&b);
        let expected = vec![fe(4),fe(12),fe(49),fe(60),fe(100)];
        assert_eq!(product.coefficients,expected);
    }
    #[test]
    fn add_two_polynomial(){
        let a = Polynomial::new(vec![fe(2),fe(3),fe(10)]);
        let b =  Polynomial::new(vec![fe(1), fe(5)]);
        /*
            (2 + 3x + 10x^2) + (1 + 5x)
            = 3 + 8x + 10x^2
            ans = [3,8,10]
        */
        let addition = a.add(&b);
        let expected = vec![fe(3),fe(8),fe(10)];
        assert_eq!(addition.coefficients,expected)
    }
    #[test]
    fn sub_two_polynomial(){
        let a = Polynomial::new(vec![fe(12),fe(7),fe(2)]);
        let b = Polynomial::new(vec![fe(2),fe(13)]);

        /*
            (12 + 7x + 2x^2) - (2 + 13x)
            = 10 - 6x + 2x^2
            ans = [10 , -6 , 2]
        */
        let subtraction = a.sub(&b);
        let expexted = vec![fe(10),-fe(6),fe(2)];
        assert_eq!(subtraction.coefficients,expexted)
    }
    #[test]
    fn position_of_non_zero_coefficients(){
        let a = Polynomial::new(vec![fe(1),fe(5),fe(10),fe(0),fe(0),fe(0)]);
        let expected = a.degree().unwrap();
        assert_eq!(expected,2)
    }
}
