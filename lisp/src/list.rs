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

impl List {
    pub fn first(&self) -> Option<&Value> {
        match self {
            List::Nil => None,
            List::Cell { ref first, .. } => Some(first),
        }
    }

    pub fn rest(&self) -> Option<&Value> {
        match self {
            List::Nil => None,
            List::Cell { ref rest, .. } => Some(rest),
        }
    }

    pub fn second(&self) -> Option<&Value> {
        self.nth(1)
    }

    pub fn nth(&self, mut idx: usize) -> Option<&Value> {
        let mut cell = Some(self);
        while idx > 0 {
            cell = cell.and_then(|c| c.rest()).and_then(|c| c.as_list());
            idx -= 1;
        }
        cell.and_then(|c| c.first())
    }
}
