use core::fmt;

pub enum Value {
    Number(i32),
    Bool(bool),
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f,"{}",n),
            Value::Bool(b) => write!(f,"{}",b),
        }
    }
}