// Chain Reversi Board Cell Value.

#[derive(Clone, Copy, PartialEq)]
pub enum Value {
    Empty,
    Red,
    Blue
}

impl Value {
    pub fn opposite(&self) -> Value {
        match self {
            Value::Red => Value::Blue,
            Value::Blue => Value::Red,
            Value::Empty => Value::Empty
        }
    }
}
