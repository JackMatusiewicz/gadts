use std::rc::Rc;
use crate::teq::Teq;

pub trait ExprT {
    type Target;
    fn eval(&self) -> Self::Target;
}

#[repr(transparent)]
pub struct IntExpr(pub(crate) i32);

impl ExprT for IntExpr {
    type Target = i32;
    fn eval(&self) -> i32 { self.0 }
}

#[repr(transparent)]
pub struct BoolExpr(pub(crate) bool);

impl ExprT for BoolExpr {
    type Target = bool;

    fn eval(&self) -> bool { self.0 }
}

pub struct AddExpr<F, G> {
    pub(crate) a: F,
    pub(crate) b: G
}

impl<F: ExprT<Target = i32>, G: ExprT<Target = i32>> ExprT for AddExpr<F,G> {
    type Target = i32;

    fn eval(&self) -> Self::Target {
        self.a.eval() + self.b.eval()
    }
}

pub struct EqExpr<F, G> {
    pub(crate) a: F,
    pub(crate) b: G
}

impl<A: Eq, F: ExprT<Target = A>, G: ExprT<Target = A>> ExprT for EqExpr<F,G> {
    type Target = bool;

    fn eval(&self) -> Self::Target {
        self.a.eval() == self.b.eval()
    }
}

pub enum Expr<A> {
    Int(i32, Teq<A, i32>),
    Bool(bool, Teq<A, bool>),
    Add(Rc<Expr<i32>>, Rc<Expr<i32>>, Teq<A, i32>),
    Eq(Rc<Expr<i32>>, Rc<Expr<i32>>, Teq<A, bool>)
}

// The copy trait constraint is used here so you can reuse
// expressions in your tree
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
