mod teq;
mod expr;

use std::rc::Rc;
use teq::Teq;
use expr::Expr;

fn id<A>(v: A) -> A { v }

fn main() {
    let int_teq = Teq::refl(id::<i32>);
    let bool_teq = Teq::refl(id::<bool>);

    let five = Expr::Int(5, int_teq.clone());
    let six = Expr::Int(5, int_teq.clone());
    let five2 = Expr::Int(5, int_teq.clone());
    let six2 = Expr::Int(6, int_teq.clone());
    
    let a_ptr = Rc::new(five2);
    let a_copy = Rc::clone(&a_ptr);
    
    let eleven = Expr::Add(Rc::new(five), Rc::new(six), int_teq.clone());
    let eleven2 = Expr::Add(a_ptr, a_copy, int_teq.clone());
    
    let even = Expr::Eq(Rc::new(eleven), Rc::new(eleven2), bool_teq.clone());
    
    let is_even = Expr::evaluate(&even);
    
    println!("{}", is_even);
}