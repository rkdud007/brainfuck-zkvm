use std::ops::Mul;

/// FieldElement is a wrapper around a usize that represents a field element in a finite field.
/// Goldilocks prime is 2^64 - 2^32 + 1
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct FieldElement(pub u64);

impl FieldElement {
    pub const fn modulus() -> u64 {
        // p = 2^64 - 2^32 + 1
        //  = 1 + 3 * 5 * 17 * 257 * 65537 * 2^32
        //   = 1 + 4294967295 * 2^32
        18446744069414584321
    }

    pub fn generator(&self) -> Self {
        FieldElement(7)
    }

    pub fn one() -> Self {
        FieldElement(1)
    }

    pub fn zero() -> Self {
        FieldElement(0)
    }

    pub fn inverse(self) -> Self {
        let (gcd, x, _) = Self::extended_gcd(self.0 as i128, Self::modulus() as i128);
        if gcd != 1 {
            panic!("{} is not invertible", self.0);
        }
        FieldElement(
            ((x % Self::modulus() as i128 + Self::modulus() as i128) % Self::modulus() as i128)
                as u64,
        )
    }

    pub fn pow(self, rhs: FieldElement) -> FieldElement {
        let mut result = FieldElement::one();
        for _ in 0..rhs.to_usize() {
            result = result * self
        }
        result
    }

    /// Extended Euclidean Algorithm
    /// a * x + b * y = gcd(a, b)
    ///
    /// if b == 0, then a is the gcd, x = 1, y = 0
    /// if b != 0, then gcd(b, a % b), x = y - (a / b) * x, y = x
    fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
        if b == 0 {
            (a, 1, 0)
        } else {
            let (gcd, x, y) = Self::extended_gcd(b, a % b);
            (gcd, y, x - (a / b) * y)
        }
    }

    pub fn to_usize(self) -> usize {
        self.0 as usize
    }
}

impl From<u64> for FieldElement {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<usize> for FieldElement {
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}

impl From<i32> for FieldElement {
    fn from(value: i32) -> Self {
        if value < 0 {
            panic!("value should be positive")
        }
        Self(value as u64)
    }
}

impl std::ops::Neg for FieldElement {
    type Output = Self;
    fn neg(self) -> Self::Output {
        FieldElement((Self::modulus() - self.0) % Self::modulus())
    }
}

impl std::ops::Add for FieldElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(((self.0 as u128 + rhs.0 as u128) % (Self::modulus() as u128)) as u64)
    }
}

impl std::ops::Sub for FieldElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(
            ((self.0 as u128 + Self::modulus() as u128 - rhs.0 as u128) % (Self::modulus() as u128))
                as u64,
        )
    }
}

impl std::ops::Mul for FieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(((self.0 as u128 * rhs.0 as u128) % Self::modulus() as u128) as u64)
    }
}

impl std::ops::Div for FieldElement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        rhs.inverse().mul(self)
    }
}

impl std::ops::AddAssign for FieldElement {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::SubAssign for FieldElement {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::fmt::Display for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[test]
fn test_field_element_add() {
    let a = FieldElement(101);
    let b = FieldElement(18446744069414584320);
    // (101 + 18446744069414584320) % p
    let c = FieldElement(100);
    assert_eq!(a + b, c);
}

#[test]
fn test_field_element_sub() {
    let a = FieldElement(10);
    let b = FieldElement(11);
    // (10 + p - 11) % p
    let c = FieldElement(18446744069414584320);
    assert_eq!(a - b, c);
}

#[test]
fn test_field_element_mul() {
    let a = FieldElement(100000000000000);
    let b = FieldElement(200000000000000);
    // (100000000000000 * 200000000000000) % p
    let c = FieldElement(13612588910694654788);

    assert_eq!(a * b, c);
}

#[test]
fn test_field_element_div() {
    let a = FieldElement(10);
    let b = FieldElement(11);
    let c = FieldElement(10061860401498864176);

    assert_eq!(a / b, c);
}

#[test]
fn test_field_element_inverse() {
    let a = FieldElement(10);
    let b = FieldElement(16602069662473125889);

    assert_eq!(a.inverse(), b);
    assert_eq!(b.inverse(), a);
    assert_eq!(a.mul(b), FieldElement::one());
}
