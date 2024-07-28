use std::error::Error;

use super::field::FieldElement;

/// Polynomial can be represent as coefficients
#[derive(PartialEq, Eq, Debug)]
pub struct Polynomial(pub Vec<FieldElement>);

impl Polynomial {
    pub fn new(coefficient: Vec<FieldElement>) -> Self {
        if coefficient.len() == 0 {
            panic!("polynomial should have at least one coefficient!")
        }
        Self(coefficient)
    }

    /// Given a set of k+1 nodes {x₀, x₁, ..., xₖ}, we can retrieve the Lagrange basis polynomial
    /// lⱼ(x) = ∏ (x - xᵢ) / (xⱼ - xᵢ), for i = 0 to k, i ≠ j
    ///        i≠j
    fn calculate_lagrange_basis_polynomial(
        nodes: Vec<FieldElement>,
        j: usize,
    ) -> Result<Self, Box<dyn Error>> {
        let target_node = nodes[j];
        let mut result = Polynomial::new(vec![FieldElement::one()]);
        for (i, node) in nodes.into_iter().enumerate() {
            if i != j {
                // Skip when i == j
                // (x - xᵢ)
                let numerator = Polynomial::new(vec![FieldElement::one(), -node]);
                // (xⱼ - xᵢ)
                let denominator: FieldElement = target_node - node;
                if denominator == FieldElement::zero() {
                    return Err("Duplicate x values are not allowed".into());
                }
                // (x - xᵢ) / (xⱼ - xᵢ)
                let term = numerator.scale(denominator.inverse());
                result = result * term;
            }
        }
        Ok(result)
    }

    pub fn interpolate_lagrange_polynomial(
        x_values: Vec<FieldElement>,
        y_values: Vec<FieldElement>,
    ) -> Result<Self, Box<dyn Error>> {
        if x_values.len() != y_values.len() {
            return Err("x values and y values should be same length".into());
        }
        let mut result = Polynomial::new(vec![FieldElement::zero()]);
        for (i, y) in y_values.into_iter().enumerate() {
            let basis_polynomial = Self::calculate_lagrange_basis_polynomial(x_values.clone(), i)?;
            let term = basis_polynomial.scale(y);
            result = result + term;
        }
        Ok(result)
    }

    pub fn evaluation(&self, x_value: FieldElement) -> FieldElement {
        let mut result = FieldElement::zero();
        for (i, x) in self.0.iter().enumerate() {
            let term = x_value.pow(FieldElement::from(self.degree() - i)) * x.to_owned();
            result += term;
        }
        result
    }

    ///Return degree of polynomial
    pub fn degree(&self) -> usize {
        if self.0.len() == 0 {
            panic!("polynomial should have at least one coefficient!")
        }
        self.0.len() - 1
    }

    pub fn scale(&self, scale_factor: FieldElement) -> Self {
        let mut scaled_coeff = Vec::new();
        for x in &self.0 {
            scaled_coeff.push(scale_factor * (*x))
        }
        Polynomial::new(scaled_coeff)
    }
}

impl std::ops::Add for Polynomial {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (longer, shorter) = if self.degree() >= rhs.degree() {
            (self, rhs)
        } else {
            (rhs, self)
        };
        let mut result_polynomial = longer;
        let diff = result_polynomial.degree() - shorter.degree();
        for (i, elem) in shorter.0.into_iter().enumerate() {
            result_polynomial.0[diff + i] = result_polynomial.0[diff + i] + elem;
        }
        result_polynomial
    }
}

impl std::ops::Sub for Polynomial {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let (longer, shorter) = if self.degree() >= rhs.degree() {
            (self, rhs)
        } else {
            (rhs, self)
        };
        let mut result_polynomial = longer;
        let diff = result_polynomial.degree() - shorter.degree();
        for (i, elem) in shorter.0.into_iter().enumerate() {
            result_polynomial.0[diff + i] = result_polynomial.0[diff + i] - elem;
        }
        result_polynomial
    }
}

impl std::ops::Mul for Polynomial {
    type Output = Polynomial;

    // [1,0] * [-2,0] = [-2,0,0]
    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = [FieldElement::zero()].repeat((self.degree() + rhs.degree() + 1) as usize);
        for (i, c1) in self.0.clone().into_iter().enumerate() {
            for (j, c2) in rhs.0.clone().into_iter().enumerate() {
                if let Some(value) = res.get_mut(i + j) {
                    *value += c1 * c2;
                }
            }
        }

        Polynomial::new(res)
    }
}

#[test]
fn test_polynomial_add() {
    // 5 * x^3 + 7 * x^2 + 10
    let a = Polynomial::new(vec![
        FieldElement::from(5),
        FieldElement::from(7),
        FieldElement::from(0),
        FieldElement::from(10),
    ]);
    // 10 * x^2 + x + 2
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
    // 5 * x^3 + 7 * x^2 + 10
    let a = Polynomial::new(vec![
        FieldElement::from(5),
        FieldElement::from(7),
        FieldElement::from(0),
        FieldElement::from(10),
    ]);
    // 10 * x^2 + x + 2
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

#[test]
fn test_polynomial_mul() {
    // 5 * x^3 + 7 * x^2 + 10
    let a = Polynomial::new(vec![
        FieldElement::from(5),
        FieldElement::from(7),
        FieldElement::from(0),
        FieldElement::from(10),
    ]);
    // 10 * x^2 + x + 2
    let b = Polynomial::new(vec![
        FieldElement::from(10),
        FieldElement::from(1),
        FieldElement::from(2),
    ]);

    let c = a * b;
    assert_eq!(
        c.0,
        vec![
            FieldElement(50),
            FieldElement(75),
            FieldElement(17),
            FieldElement(114),
            FieldElement(10),
            FieldElement(20)
        ]
    )
}

#[test]
fn test_linear_interpolation() {
    // interpolate over (2,4), (0,2)
    let x_values = vec![FieldElement::from(2), FieldElement::from(0)];
    let y_values = vec![FieldElement::from(4), FieldElement::from(2)];
    // should return y = x + 2
    let interpolated =
        Polynomial::interpolate_lagrange_polynomial(x_values.clone(), y_values.clone()).unwrap();
    for (i, x) in x_values.into_iter().enumerate() {
        let y_expected = interpolated.evaluation(x);
        assert_eq!(y_values[i], y_expected);
    }
}

#[test]
fn test_quadratic_interpolation() {
    // interpolate over (0,0), (1,1), (2,4)
    let x_values = vec![
        FieldElement::from(0),
        FieldElement::from(1),
        FieldElement::from(2),
    ];
    let y_values = vec![
        FieldElement::from(0),
        FieldElement::from(1),
        FieldElement::from(4),
    ];
    let interpolated =
        Polynomial::interpolate_lagrange_polynomial(x_values.clone(), y_values.clone()).unwrap();
    for (i, x) in x_values.into_iter().enumerate() {
        let y_expected = interpolated.evaluation(x);
        assert_eq!(y_values[i], y_expected);
    }
}

#[test]
fn test_cubic_interpolation() {
    let x_values = vec![
        -FieldElement::from(1),
        FieldElement::from(0),
        FieldElement::from(1),
        FieldElement::from(2),
    ];
    let y_values = vec![
        -FieldElement::from(1),
        FieldElement::from(0),
        FieldElement::from(1),
        FieldElement::from(8),
    ];
    let interpolated =
        Polynomial::interpolate_lagrange_polynomial(x_values.clone(), y_values.clone()).unwrap();
    for (i, x) in x_values.into_iter().enumerate() {
        let y_expected = interpolated.evaluation(x);
        assert_eq!(y_values[i], y_expected);
    }
}

#[test]
fn test_interpolation_with_zero_y_values() {
    let x_values = vec![
        -FieldElement::from(1),
        FieldElement::from(0),
        FieldElement::from(1),
    ];
    let y_values = vec![
        FieldElement::from(1),
        FieldElement::from(0),
        FieldElement::from(1),
    ];
    let interpolated =
        Polynomial::interpolate_lagrange_polynomial(x_values.clone(), y_values.clone()).unwrap();
    for (i, x) in x_values.into_iter().enumerate() {
        let y_expected = interpolated.evaluation(x);
        assert_eq!(y_values[i], y_expected);
    }
}

#[test]
fn test_constant_polynomial_interpolation() {
    let x_values = vec![
        FieldElement::from(1),
        FieldElement::from(2),
        FieldElement::from(3),
    ];
    let y_values = vec![
        FieldElement::from(5),
        FieldElement::from(5),
        FieldElement::from(5),
    ];
    let interpolated =
        Polynomial::interpolate_lagrange_polynomial(x_values.clone(), y_values.clone()).unwrap();
    for (i, x) in x_values.into_iter().enumerate() {
        let y_expected = interpolated.evaluation(x);
        assert_eq!(y_values[i], y_expected);
    }
}

#[test]
fn test_interpolation_with_repeated_x_values() {
    let x_values = vec![
        FieldElement::from(1),
        FieldElement::from(1),
        FieldElement::from(2),
    ];
    let y_values = vec![
        FieldElement::from(1),
        FieldElement::from(1),
        FieldElement::from(2),
    ];
    let result = Polynomial::interpolate_lagrange_polynomial(x_values, y_values);
    assert!(
        result.is_err(),
        "Expected an error due to repeated x values"
    );
    if let Err(e) = result {
        assert_eq!(
            e.to_string(),
            "Duplicate x values are not allowed",
            "Unexpected error message"
        );
    }
}
