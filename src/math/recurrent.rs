use std::{
    cmp::{max, min},
    ops::{Add, Mul, Sub},
};

use super::{expression::Expression, utils};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Recurrent {
    coeffs: Vec<i64>,
}

impl Recurrent {
    pub fn constant(c: i64) -> Self {
        Self {
            coeffs: match c {
                0 => vec![],
                c => vec![c],
            },
        }
    }

    pub fn indvar() -> Self {
        Self { coeffs: vec![0, 1] }
    }

    pub fn nth_element_expr(&self) -> Expression {
        self.coeffs
            .iter()
            .enumerate()
            .map(|(i, &c)| {
                Expression::Constant(c) * Expression::binomial(Expression::Var, i as i64)
            })
            .fold(Expression::Constant(0), |e1, e2| e1 + e2)
    }

    pub fn sum_expr(&self) -> Expression {
        let mut coeffs = vec![0];
        coeffs.extend_from_slice(&self.coeffs);
        Self { coeffs }.nth_element_expr()
    }
}

impl Add for Recurrent {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut coeffs = Vec::new();
        let n = max(self.coeffs.len(), rhs.coeffs.len());
        for i in 0..n {
            coeffs.push(
                self.coeffs.get(i).copied().unwrap_or_default()
                    + rhs.coeffs.get(i).copied().unwrap_or_default(),
            );
        }
        utils::pop_zeros(&mut coeffs);
        Self { coeffs }
    }
}

impl Sub for Recurrent {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut coeffs = Vec::new();
        let n = max(self.coeffs.len(), rhs.coeffs.len());
        for i in 0..n {
            coeffs.push(
                self.coeffs.get(i).copied().unwrap_or_default()
                    - rhs.coeffs.get(i).copied().unwrap_or_default(),
            );
        }
        utils::pop_zeros(&mut coeffs);
        Self { coeffs }
    }
}

impl Mul for Recurrent {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut coeffs = vec![0; self.coeffs.len() + rhs.coeffs.len() - 1];
        for (i, c1) in self.coeffs.iter().enumerate() {
            for (j, c2) in rhs.coeffs.iter().enumerate() {
                let c = c1 * c2;
                for k in 0..=min(i, j) {
                    let pos = i + j - k;
                    let val = utils::multinomial(i + j - k, &[k, i - k, j - k]);
                    coeffs[pos] += (val as i64) * c;
                }
            }
        }
        utils::pop_zeros(&mut coeffs);
        Self { coeffs }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        let r1 = Recurrent {
            coeffs: vec![1, 2, 3],
        };
        let r2 = Recurrent { coeffs: vec![3, 4] };
        assert_eq!(
            r1 + r2,
            Recurrent {
                coeffs: vec![4, 6, 3]
            }
        );

        let r1 = Recurrent { coeffs: vec![3, 4] };
        let r2 = Recurrent {
            coeffs: vec![1, 2, 3],
        };
        assert_eq!(
            r1 + r2,
            Recurrent {
                coeffs: vec![4, 6, 3]
            }
        );
    }

    #[test]
    fn addition_clear_zeros() {
        let r1 = Recurrent { coeffs: vec![3, 4] };
        let r2 = Recurrent {
            coeffs: vec![-3, -4],
        };
        assert_eq!(r1 + r2, Recurrent { coeffs: vec![] });
    }

    #[test]
    fn multiplication() {
        let r1 = Recurrent { coeffs: vec![1, 1] };
        let r2 = Recurrent {
            coeffs: vec![1, 3, 2],
        };
        assert_eq!(
            r1 * r2,
            Recurrent {
                coeffs: vec![1, 7, 12, 6]
            }
        );
    }
}
