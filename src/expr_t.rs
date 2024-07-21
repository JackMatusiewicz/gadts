use std::ops::Add;

pub trait ExprT {
    type Target;
    fn eval(&self) -> Self::Target;
}

#[repr(transparent)]
pub struct IntExpr(pub(crate) i32);

impl ExprT for IntExpr {
    type Target = i32;
    fn eval(&self) -> i32 {
        self.0
    }
}

#[repr(transparent)]
pub struct BoolExpr(pub(crate) bool);

impl ExprT for BoolExpr {
    type Target = bool;

    fn eval(&self) -> bool {
        self.0
    }
}

pub struct AddExpr<F, G> {
    pub(crate) a: F,
    pub(crate) b: G,
}

impl<A: Copy + Add<Output = A>, F: ExprT<Target = A>, G: ExprT<Target = A>> ExprT
    for AddExpr<F, G>
{
    type Target = A;

    fn eval(&self) -> Self::Target {
        self.a.eval() + self.b.eval()
    }
}

pub struct EqExpr<F, G> {
    pub(crate) a: F,
    pub(crate) b: G,
}

impl<A: Eq, F: ExprT<Target = A>, G: ExprT<Target = A>> ExprT for EqExpr<F, G> {
    type Target = bool;

    fn eval(&self) -> Self::Target {
        self.a.eval() == self.b.eval()
    }
}
