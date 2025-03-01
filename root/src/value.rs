// src/value.rs

#[derive(Clone, Debug)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}
