use super::{
    expr::{BinaryOp, BinaryOpTy, Expr, Literal},
    value::Value,
};

pub fn interpret_expr(expr: &Expr) -> Value {
    match expr {
        Expr::Literal(lit) => interpret_literal(lit),
        Expr::Unary(unary_op, expr) => todo!(),
        Expr::Binary(lhs, op, rhs) => interpret_binary(lhs, *op, rhs),
        Expr::Logical(expr, logical_op, expr1) => todo!(),
        Expr::Grouping(expr) => todo!(),
    }
}

fn interpret_literal(lit: &Literal) -> Value {
    match lit {
        Literal::Number(n) => Value::Number(*n),
        Literal::Register(_) => todo!(),
    }
}

fn interpret_binary(lhs: &Expr, op: BinaryOp, rhs: &Expr) -> Value {
    let lhs = interpret_expr(lhs);
    let rhs = interpret_expr(rhs);

    match (&lhs, op.ty, &rhs) {
        (Value::Number(n1), BinaryOpTy::Plus, Value::Number(n2)) => Value::Number(n1 + n2),
        (Value::Number(n1), BinaryOpTy::Minus, Value::Number(n2)) => Value::Number(n1 - n2),
        (Value::Number(n1), BinaryOpTy::Mul, Value::Number(n2)) => Value::Number(n1 * n2),
        (Value::Number(n1), BinaryOpTy::Div, Value::Number(n2)) => {
            if *n2 == 0 {
                return Value::Bool(false);
            } else {
                Value::Number(n1 / n2)
            }
        }
        _ => {
            println!("invalid operands in binary operator ");
            Value::Bool(false)
        }
    }
}
