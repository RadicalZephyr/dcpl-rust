use crate::{List, Value};

use std::iter::FromIterator;

impl FromIterator<Value> for List {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Value>,
    {
        let values: Vec<Value> = iter.into_iter().collect();

        let mut cell = List::Nil;
        for value in values.into_iter().rev() {
            cell = List::Cell {
                first: Box::new(value),
                rest: Box::new(Value::List(cell)),
            };
        }
        cell
    }
}
