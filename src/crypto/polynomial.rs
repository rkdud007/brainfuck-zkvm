use std::error::Error;

use super::field::FieldElement;

/// Polynomial can be represent as coefficients
pub struct Polynomial(pub Vec<FieldElement>);

impl Polynomial {
    pub fn new(coefficient: Vec<FieldElement>) -> Self {
        Self(coefficient)
    }

    /// Given a set of k+1 nodes {x₀, x₁, ..., xₖ}, we can retrieve the Lagrange basis polynomial
    /// lⱼ(x) = ∏ (x - xᵢ) / (xⱼ - xᵢ), for i = 0 to k, i ≠ j
    ///        i≠j
    fn calculate_lagrange_basis_polynomial(
        nodes: Vec<FieldElement>,
        j: usize,
    ) -> Result<(), Box<dyn Error>> {
        let mut lagrange_basis_polynomials: Vec<Polynomial> = vec![];
        let target_node = nodes[j];
        for node in nodes {
            // (x - xᵢ)
            let numerator = Polynomial::new(vec![FieldElement::one(), -node]);
            let denominator = Polynomial::new(vec![target_node, -node]);
        }
        Ok(())
    }
}

impl std::ops::Add for Polynomial {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (longer, shorter) = if self.0.len() >= rhs.0.len() {
            (self.0, rhs.0)
        } else {
            (rhs.0, self.0)
        };
        let mut result_coefficient: Vec<FieldElement> = longer;
        let diff = result_coefficient.len() - shorter.len();
        for (i, elem) in shorter.into_iter().enumerate() {
            result_coefficient[diff + i] = result_coefficient[diff + i] + elem;
        }

        Self::new(result_coefficient)
    }
}

impl std::ops::Sub for Polynomial {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let (longer, shorter) = if self.0.len() >= rhs.0.len() {
            (self.0, rhs.0)
        } else {
            (rhs.0, self.0)
        };
        let mut result_coefficient: Vec<FieldElement> = longer;
        let diff = result_coefficient.len() - shorter.len();
        for (i, elem) in shorter.into_iter().enumerate() {
            result_coefficient[diff + i] = result_coefficient[diff + i] - elem;
        }

        Self::new(result_coefficient)
    }
}

#[test]
fn test_polynomial_add() {
    let a = Polynomial::new(vec![
        FieldElement::from(5),
        FieldElement::from(7),
        FieldElement::from(0),
        FieldElement::from(10),
    ]);
    let b = Polynomial::new(vec![
        FieldElement::from(10),
        FieldElement::from(1),
        FieldElement::from(2),
    ]);

    let c = a + b;
    assert_eq!(
        c.0,
        vec![
            FieldElement::from(5),
            FieldElement::from(17),
            FieldElement::from(1),
            FieldElement::from(12),
        ]
    )
}

#[test]
fn test_polynomial_sub() {
    let a = Polynomial::new(vec![
        FieldElement::from(5),
        FieldElement::from(7),
        FieldElement::from(0),
        FieldElement::from(10),
    ]);
    let b = Polynomial::new(vec![
        FieldElement::from(10),
        FieldElement::from(1),
        FieldElement::from(2),
    ]);

    let c = a - b;
    assert_eq!(
        c.0,
        vec![
            FieldElement::from(5),
            FieldElement::from(-FieldElement::from(3)),
            FieldElement::from(-FieldElement::from(1)),
            FieldElement::from(8),
        ]
    )
}
