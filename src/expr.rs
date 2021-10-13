use std::rc::Rc;
use crate::teq::Teq;

// The copy trait constraint is used here so you can reuse
// expressions in your tree
pub enum Expr<A: Copy> {
    Int(i32, Teq<A, i32>),
    Bool(bool, Teq<A, bool>),
    Add(Rc<Expr<i32>>, Rc<Expr<i32>>, Teq<A, i32>),
    Eq(Rc<Expr<i32>>, Rc<Expr<i32>>, Teq<A, bool>)
}

impl<A: Copy> Expr<A> {
    pub fn evaluate(e: &Expr<A>) -> A {
        match e {
            Expr::Int(v, t) =>
                Teq::from(t, v.clone()),
            Expr::Bool(v, t) =>
                Teq::from(t, v.clone()),
            Expr::Add(left, right, t) => {
                let l = Expr::evaluate(&left);
                let r = Expr::evaluate(&right);
                let sum = l + r;
                Teq::from(t, sum)
            },
            Expr::Eq(left, right, t) => {
                let l = Expr::evaluate(&left);
                let r = Expr::evaluate(&right);
                let sum = l == r;
                Teq::from(t, sum)
            }
        }
    }
}