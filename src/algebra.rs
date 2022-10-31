use std::ops::{Add, Mul};

#[derive(Clone, Debug)]
pub enum Algebra<Numeric, Variable: Eq> {
    Val(Numeric),
    Var(Variable),

    Add(Box<Self>, Box<Self>),
    Mul(Box<Self>, Box<Self>),
}

impl<Numeric: PartialEq, Variable: Eq> PartialEq<Self> for Algebra<Numeric, Variable> {
    fn eq(&self, other: &Self) -> bool {
        use Algebra::*;

        match (self, other) {
            (Val(l0), Val(r0)) => l0 == r0,
            (Var(l0), Var(r0)) => l0 == r0,

            // Assume commutativity
            (Add(l0, l1), Add(r0, r1)) => (l0 == r0 && l1 == r1) || (l0 == r1 && l0 == r1),
            (Mul(l0, l1), Mul(r0, r1)) => (l0 == r0 && l1 == r1) || (l0 == r1 && l0 == r1),

            _ => false,
        }
    }
}

impl<Numeric: Clone, Variable: Eq + Clone> Algebra<Numeric, Variable> {
    pub fn var_sub(&mut self, f: &Variable, t: &Self) {
        use Algebra::*;

        match self {
            Var(v) if v == f => *self = t.clone(),

            Add(a, b) => {
                a.var_sub(f, t);
                b.var_sub(f, t);
            }
            Mul(a, b) => {
                a.var_sub(f, t);
                b.var_sub(f, t);
            }
            _ => (),
        }
    }
}
impl<
        Numeric: Add<Numeric, Output = Numeric> + Mul<Numeric, Output = Numeric> + Clone,
        Variable: Eq,
    > Algebra<Numeric, Variable>
{
    pub fn simplify(&mut self) {
        match self {
            Self::Add(box Self::Val(a), box Self::Val(b)) => {
                *self = Self::Val(a.clone() + b.clone());
            }
            Self::Mul(box Self::Val(a), box Self::Val(b)) => {
                *self = Self::Val(a.clone() * b.clone());
            }
            Self::Add(box ref mut a, box ref mut b) | Self::Mul(box ref mut a, box ref mut b) => {
                a.simplify();
                b.simplify();

                self.simplify();
            }
            _ => (),
        }
    }
}

impl<
        Numeric: Add<Numeric, Output = Numeric> + Mul<Numeric, Output = Numeric> + num_traits::One + Clone,
        Variable: Eq + Clone,
    > Algebra<Numeric, Variable>
{
    pub fn into_zip_sequence(self) -> Vec<(Vec<Variable>, Numeric)> {
        use Algebra::*;

        match self {
            Val(a) => vec![(vec![], a)],
            Var(b) => vec![(vec![b], Numeric::one())],
            Add(box a, box b) => {
                let mut a = a.into_zip_sequence();
                let mut b = b.into_zip_sequence();

                for (x, ref mut xv) in a.iter_mut() {
                    b.retain(move |(y, yv)| {
                        if let None = y.iter().filter(|v| !x.contains(v)).next() {
                            *xv = xv.clone().add(yv.clone());

                            false
                        } else {
                            true
                        }
                    })
                }

                [a, b].concat()
            }
            Mul(box a, box b) => {
                let a = a.into_zip_sequence();
                let b = b.into_zip_sequence();

                let mut out = Vec::with_capacity(a.len());

                for (x, xv) in a {
                    for (y, yv) in b.iter() {
                        out.push(([x.clone(), y.clone()].concat(), xv.clone() * yv.clone()))
                    }
                }

                out
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_arithmetic() {
        use Algebra::*;
        let mut expr: Algebra<i32, ()> = Mul(box Add(box Val(1), box Val(3)), box Val(10));

        expr.simplify();

        assert_eq!(Val(40), expr);
    }

    #[test]
    fn it_can_convert_to_linear() {
        use Algebra::*;
        let expr: Algebra<i32, char> = Mul(box Add(box Val(5), box Var('x')), box Val(10));

        println!("{:#?}", expr.into_zip_sequence())
    }
}
