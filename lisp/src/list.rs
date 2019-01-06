use crate::{List, Value};

use std::iter::{FromIterator, IntoIterator};

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

pub struct IntoIter(List);

impl Iterator for IntoIter {
    type Item = Value;

    fn next(&mut self) -> Option<Value> {
        match self.0.clone() {
            List::Cell { first, rest } => {
                self.0 = rest.into_list().unwrap_or(List::Nil);

                Some(*first)
            }
            List::Nil => None,
        }
    }
}

impl IntoIterator for List {
    type Item = Value;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

pub struct Iter<'a>(&'a List);

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<&'a Value> {
        match self.0 {
            List::Cell {
                ref first,
                ref rest,
            } => {
                self.0 = rest.as_list().unwrap_or(&List::Nil);

                Some(first)
            }
            List::Nil => None,
        }
    }
}

impl<'a> IntoIterator for &'a List {
    type Item = &'a Value;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter(self)
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
