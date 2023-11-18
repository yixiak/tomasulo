use std::rc::Rc;
use super::{Unit, Type};

pub type Value = Rc<ValueInner>;

#[derive(Debug)]
pub enum ValueInner {
    /// An immediate value.
    Imm(i64),
    /// A float value.
    Float(f64),
    /// A unit.
    Unit(Unit),
    /// A memory address.
    MemAddr(Value),
    // A operation.
    Op(Type, Value, Value),
}

impl From<i64> for ValueInner {
    fn from(i:i64)->ValueInner{
        return ValueInner::Imm(i);
    }
}

impl From<f64> for ValueInner {
    fn from(f:f64)->ValueInner{
        return ValueInner::Float(f);
    }
}


impl From<Unit> for ValueInner {
    fn from(u: Unit) -> ValueInner {
        return ValueInner::Unit(u);
    }
}

impl From<Value> for ValueInner {
    fn from(v: Value) -> ValueInner {
        return ValueInner::MemAddr(v);
    }
}