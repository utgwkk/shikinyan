use syntax::Expr;
use syntax::Expr::*;
use eval::Value::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Value {
    IntV(i64),
    BoolV(bool),
}

pub fn evaluate (exp: Expr) -> Result<Value, &'static str> {
    match exp {
        Var(x) => unimplemented!(),
        Int(i) => Ok(IntV(i)),
        Bool(b) => Ok(BoolV(b)),
        Plus(be1, be2) => {
            let lhs = evaluate(*be1);
            let rhs = evaluate(*be2);
            match (lhs, rhs) {
                (Ok(IntV(i1)), Ok(IntV(i2))) => Ok(IntV(i1 + i2)),
                (_, _) => Err("both arguments must be integer: +")
            }
        },
        Mult(be1, be2) => {
            let lhs = evaluate(*be1);
            let rhs = evaluate(*be2);
            match (lhs, rhs) {
                (Ok(IntV(i1)), Ok(IntV(i2))) => Ok(IntV(i1 * i2)),
                (_, _) => Err("both arguments must be integer: *")
            }
        },
        Lt(be1, be2) => {
            let lhs = evaluate(*be1);
            let rhs = evaluate(*be2);
            match (lhs, rhs) {
                (Ok(IntV(i1)), Ok(IntV(i2))) => Ok(BoolV(i1 < i2)),
                (_, _) => Err("both arguments must be integer: <")
            }
        },
        If(be1, be2, be3) => {
            let e1 = evaluate(*be1);
            let e2 = evaluate(*be2);
            let e3 = evaluate(*be3);
            match e1 {
                Ok(BoolV(true)) => e2,
                Ok(BoolV(false)) => e3,
                _ => Err("Condition must be boolean: if")
            }
        },
    }
}

#[test]
fn test_1_plus_1_is_2() {
    let ast = Plus(Box::new(Int(1)), Box::new(Int(1)));
    assert_eq!(Ok(IntV(2)), evaluate(ast));
}

#[test]
fn test_2_times_3_is_6() {
    let ast = Mult(Box::new(Int(2)), Box::new(Int(3)));
    assert_eq!(Ok(IntV(6)), evaluate(ast));
}

#[test]
fn test_1_lt_2_is_true() {
    let ast = Lt(Box::new(Int(1)), Box::new(Int(2)));
    assert_eq!(Ok(BoolV(true)), evaluate(ast));
}

#[test]
fn test_3_lt_2_is_false() {
    let ast = Lt(Box::new(Int(3)), Box::new(Int(2)));
    assert_eq!(Ok(BoolV(false)), evaluate(ast));
}

#[test]
fn test_true() {
    let ast = Bool(true);
    assert_eq!(Ok(BoolV(true)), evaluate(ast));
}

#[test]
fn test_false() {
    let ast = Bool(false);
    assert_eq!(Ok(BoolV(false)), evaluate(ast));
}

#[test]
fn test_if_true_then_2_else_3() {
    let ast = If(Box::new(Bool(true)), Box::new(Int(2)), Box::new(Int(3)));
    assert_eq!(Ok(IntV(2)), evaluate(ast));
}

#[test]
fn test_if_false_then_2_else_3() {
    let ast = If(Box::new(Bool(false)), Box::new(Int(2)), Box::new(Int(3)));
    assert_eq!(Ok(IntV(3)), evaluate(ast));
}
