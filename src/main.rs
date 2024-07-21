mod teq;
mod expr;

use std::rc::Rc;
use teq::Teq;
use expr::Expr;
use crate::expr::{AddExpr, EqExpr, ExprT, IntExpr};

fn main() {
    let int_teq = Teq::refl();
    let bool_teq = Teq::refl();

    let five = Expr::Int(5, int_teq.clone());
    let another_five = Expr::Int(5, int_teq.clone());
    let five2 = Expr::Int(5, int_teq.clone());
    
    let a_ptr = Rc::new(five2);
    let a_copy = Rc::clone(&a_ptr);
    
    let first_ten = Expr::Add(Rc::new(five), Rc::new(another_five), int_teq.clone());
    let ten = Expr::Add(a_ptr, a_copy, int_teq.clone());
    
    let even = Expr::Eq(Rc::new(first_ten), Rc::new(ten), bool_teq.clone());
    
    let is_even = Expr::evaluate(&even);

    let a = IntExpr(5);
    let b = IntExpr(5);
    let sum = AddExpr { a,b };
    let ten = IntExpr(10);
    let equal = EqExpr {a: sum, b: ten };
    
    println!("{}", equal.eval());
}