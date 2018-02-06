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
