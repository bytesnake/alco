use std::ops::Range;
use std::any::{Any, type_name};
use std::collections::HashMap;
use std::any::TypeId;

use crate::error::{Result, Error};
use super::samples::{Sample, Samples};

pub enum ParamSet<T> {
    Range(Range<T>),
    Items(Vec<T>),
}

impl<T: Clone> ParamSet<T> {
    pub fn lower_bound(&self) -> T {
        match self {
            ParamSet::Range(range) => range.start.clone(),
            ParamSet::Items(items) => items[0].clone(),
        }
    }
}

pub enum ParamType {
    Usize(ParamSet<usize>),
    Float(ParamSet<f32>),
    Str(Vec<String>),
}

impl ParamType {
    pub fn from_range<T: Any>(range: Range<T>) -> Result<ParamType> {
        let type_id = TypeId::of::<T>();

        if type_id == TypeId::of::<usize>() {
            let (a, b) = (range.start, range.end);
            let (a, b): (&usize, &usize) = (
                Any::downcast_ref(&a).unwrap(),
                Any::downcast_ref(&b).unwrap(),
            );

            return Ok(ParamType::Usize(ParamSet::Range(*a..*b)));
        } else if type_id == TypeId::of::<f32>() {
            let (a, b) = (range.start, range.end);
            let (a, b): (&f32, &f32) = (
                Any::downcast_ref(&a).unwrap(),
                Any::downcast_ref(&b).unwrap(),
            );

            return Ok(ParamType::Float(ParamSet::Range(*a..*b)));
        } else if type_id == TypeId::of::<String>() {
            return Err(Error::StringRange);
        } else {
            return Err(Error::InvalidType(type_name::<T>().into()));
        }
    }

    pub fn from_items<'a, T: Any>(items: &'a [T]) -> Result<ParamType> {
        let type_id = TypeId::of::<T>();

        if type_id == TypeId::of::<usize>() {
            let items = items.into_iter()
                .map(|x| *Any::downcast_ref(x).unwrap())
                .collect();

            return Ok(ParamType::Usize(ParamSet::Items(items)));
        } else if type_id == TypeId::of::<f32>() {
            let items = items.into_iter()
                .map(|x| *Any::downcast_ref(x).unwrap())
                .collect();

            return Ok(ParamType::Float(ParamSet::Items(items)));
        } else if type_id == TypeId::of::<String>() {
            let items = items.into_iter()
                .map(|x| Any::downcast_ref::<String>(x).unwrap().to_string())
                .collect();

            return Ok(ParamType::Str(items));
        } else {
            return Err(Error::InvalidType(type_name::<T>().into()));
        }
    }

    pub fn lower_bound(&self) -> Sample {
        match self {
            ParamType::Usize(u) => Sample::Usize(u.lower_bound()),
            ParamType::Float(f) => Sample::Float(f.lower_bound()),
            ParamType::Str(s) => Sample::Str(s.first().unwrap().clone()),
        }
    }
}

pub struct ParamBuilder<'a> {
    map: HashMap<&'a str, (usize, ParamType)>,
}

impl<'a> ParamBuilder<'a> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    pub fn add_range<T: Any>(&mut self, name: &'a str, range: Range<T>) -> Result<()> {
        if self.map.contains_key(name) {
            return Err(Error::ArgumentAlreadyExists(name.to_string()));
        }

        ParamType::from_range(range)
            .map(|param| {
                self.map.insert(name, (20, param));

                ()
            })
    }

    pub fn add_items<T: Any, S: AsRef<[T]>>(&mut self, name: &'a str, items: S) -> Result<()> {
        let items = items.as_ref();

        if self.map.contains_key(name) {
            return Err(Error::ArgumentAlreadyExists(name.to_string()));
        }

        ParamType::from_items(items)
            .map(|param| {
                self.map.insert(name, (20, param));

                ()
            })
    }

    pub fn lower_bound(&self) -> Samples {
        let res = self.map.iter().map(|(a, b)| {
            (a.to_string(), b.1.lower_bound())
        }).collect();

        Samples::new(res)
    }

    pub fn params(&self) -> Vec<&str> {
        self.map.keys().into_iter().cloned().collect()
    }

    pub fn next_step(&self, prev: Samples, name: &str) -> Samples {
        prev
    }
}
