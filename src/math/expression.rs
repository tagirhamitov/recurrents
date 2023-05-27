use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Clone)]
pub enum Expression {
    Constant(i64),
    Var,
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn binomial(n: Expression, k: i64) -> Expression {
        let mut result = Expression::Constant(1);
        for i in 0..k {
            result = result * (n.clone() - Expression::Constant(i));
        }
        result / Expression::Constant((1..=k).product())
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Constant(c) => write!(f, "{}", c),
            Expression::Var => write!(f, "n"),
            Expression::Add(a, b) => write!(f, "({} + {})", a, b),
            Expression::Sub(a, b) => write!(f, "({} - {})", a, b),
            Expression::Mul(a, b) => write!(f, "({} * {})", a, b),
            Expression::Div(a, b) => write!(f, "({} / {})", a, b),
        }
    }
}

impl Add for Expression {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if let Expression::Constant(0) = self {
            return rhs;
        }
        if let Expression::Constant(0) = rhs {
            return self;
        }
        Expression::Add(Box::new(self), Box::new(rhs))
    }
}

impl Sub for Expression {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if let Expression::Constant(0) = rhs {
            return self;
        }
        Expression::Sub(Box::new(self), Box::new(rhs))
    }
}

impl Mul for Expression {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if matches!(self, Expression::Constant(1)) {
            return rhs;
        }
        if matches!(self, Expression::Constant(0)) {
            return Expression::Constant(0);
        }
        if matches!(rhs, Expression::Constant(1)) {
            return self;
        }
        if matches!(rhs, Expression::Constant(0)) {
            return Expression::Constant(0);
        }
        Expression::Mul(Box::new(self), Box::new(rhs))
    }
}

impl Div for Expression {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if let Expression::Constant(1) = rhs {
            return self;
        }
        Expression::Div(Box::new(self), Box::new(rhs))
    }
}
