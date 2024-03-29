use std::convert::TryInto;
use super::{TryAsRef, VariableContext, List, Path};

#[derive(Clone, PartialEq, Debug)]
pub struct VariablePointer { 
    pub(crate) name: String,
    pub(crate) context: VariableContext,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Int(i64),
    Float(f64),
    List(List),
    String(String),

    DivertTarget(Path),
    VariablePointer(VariablePointer),
}

impl Value {
    /// Checks the truthiness of the Value:
    /// *   Int: value is not 0
    /// *   Float: value is not 0.0
    /// *   String: string is not empty
    /// *   List: list is not empty
    ///
    /// # Panics
    ///
    /// Panics if the value is a divert target or a variable pointer
    pub(crate) fn is_truthy(&self) -> bool {
        match self {
            Value::Int(val) => *val != 0,
            Value::Float(val) => *val != 0.0,
            Value::String(val) => !val.is_empty(),
            Value::List(val) => !val.is_empty(),
            Value::DivertTarget(..) => panic!("Cannot check the truthiness of a divert target"),
            Value::VariablePointer(..) => panic!("Cannot check the truthiness of a variable pointer"),
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Int(if value { 1 } else { 0 })
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<List> for Value {
    fn from(value: List) -> Self {
        Self::List(value)
    }
}

impl From<Path> for Value {
    fn from(value: Path) -> Self {
        Self::DivertTarget(value)
    }
}

impl From<VariablePointer> for Value {
    fn from(value: VariablePointer) -> Self {
        Self::VariablePointer(value)
    }
}

// TODO: settle on some representation for the other types that is good enough to pass out to
// external functions in the future. For now, these will suffice as a proof of concept.

impl TryAsRef<i64> for Value {
    fn try_as_ref(&self) -> Option<&i64> {
        match self {
            Value::Int(ref value) => Some(value),
            _ => None,
        }
    }
}

impl TryAsRef<f64> for Value {
    fn try_as_ref(&self) -> Option<&f64> {
        match self {
            Value::Float(ref value) => Some(value),
            _ => None,
        }
    }
}

impl TryAsRef<String> for Value {
    fn try_as_ref(&self) -> Option<&String> {
        match self {
            Value::String(ref value) => Some(value),
            _ => None,
        }
    }
}

impl TryAsRef<List> for Value {
    fn try_as_ref(&self) -> Option<&List> {
        match self {
            Value::List(ref value) => Some(value),
            _ => None,
        }
    }
}

impl TryAsRef<Path> for Value {
    fn try_as_ref(&self) -> Option<&Path> {
        match self {
            Value::DivertTarget(ref value) => Some(value),
            _ => None,
        }
    }
}

impl TryAsRef<VariablePointer> for Value {
    fn try_as_ref(&self) -> Option<&VariablePointer> {
        match self {
            Value::VariablePointer(ref value) => Some(value),
            _ => None,
        }
    }
}


impl TryInto<i64> for Value {
    type Error = ();
    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            Value::Int(value) => Ok(value),
            Value::Float(value) => Ok(value as i64),
            Value::String(value) => value.parse().map_err(|_| ()),
            _ => Err(()),
        }
    }
}

impl TryInto<f64> for Value {
    type Error = ();
    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Value::Int(value) => Ok(value as f64),
            Value::Float(value) => Ok(value),
            Value::String(value) => value.parse().map_err(|_| ()),
            _ => Err(()),
        }
    }
}

impl TryInto<String> for Value {
    type Error = ();
    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Value::Int(value) => Ok(format!("{}", value)),
            Value::Float(value) => Ok(format!("{}", value)),
            Value::String(value) => Ok(value),
            _ => Err(()),
        }
    }
}

impl TryInto<Path> for Value {
    type Error = ();
    fn try_into(self) -> Result<Path, Self::Error> {
        match self {
            Value::DivertTarget(value) => Ok(value),
            _ => Err(()),
        }
    }
}

impl TryInto<VariablePointer> for Value {
    type Error = ();
    fn try_into(self) -> Result<VariablePointer, Self::Error> {
        match self {
            Value::VariablePointer(value) => Ok(value),
            _ => Err(()),
        }
    }
}

