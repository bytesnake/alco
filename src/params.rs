use std::ops::Range;
use std::any::{Any, type_name};
use std::collections::HashMap;
use std::hash::Hash;
use std::any::TypeId;

use crate::error::{Result, Error};

pub enum ParamsRange<T> {
    Range(Range<T>),
    Items(Vec<T>),
}

pub enum Param {
    Usize(ParamsRange<usize>),
    Float(ParamsRange<f32>),
    Str(Vec<String>),
}

impl Param {
    pub fn from_range<T: Any>(range: Range<T>) -> Result<Param> {
        let type_id = TypeId::of::<T>();

        if type_id == TypeId::of::<usize>() {
            let (a, b) = (range.start, range.end);
            let (a, b): (&usize, &usize) = (
                Any::downcast_ref(&a).unwrap(),
                Any::downcast_ref(&b).unwrap(),
            );

            return Ok(Param::Usize(ParamsRange::Range(*a..*b)));
        } else if type_id == TypeId::of::<f32>() {
            let (a, b) = (range.start, range.end);
            let (a, b): (&f32, &f32) = (
                Any::downcast_ref(&a).unwrap(),
                Any::downcast_ref(&b).unwrap(),
            );

            return Ok(Param::Float(ParamsRange::Range(*a..*b)));
        } else if type_id == TypeId::of::<String>() {
            return Err(Error::StringRange);
        } else {
            return Err(Error::InvalidType(type_name::<T>().into()));
        }
    }

    pub fn from_items<'a, T: Any>(items: &'a [T]) -> Result<Param> {
        let type_id = TypeId::of::<T>();

        if type_id == TypeId::of::<usize>() {
            let items = items.into_iter()
                .map(|x| *Any::downcast_ref(x).unwrap())
                .collect();

            return Ok(Param::Usize(ParamsRange::Items(items)));
        } else if type_id == TypeId::of::<f32>() {
            let items = items.into_iter()
                .map(|x| *Any::downcast_ref(x).unwrap())
                .collect();

            return Ok(Param::Float(ParamsRange::Items(items)));
        } else if type_id == TypeId::of::<String>() {
            let items = items.into_iter()
                .map(|x| Any::downcast_ref::<String>(x).unwrap().to_string())
                .collect();

            return Ok(Param::Str(items));
        } else {
            return Err(Error::InvalidType(type_name::<T>().into()));
        }
    }
}

pub struct ParamBuilder<'a> {
    map: HashMap<&'a str, Param>,
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

        Param::from_range(range)
            .map(|param| {
                self.map.insert(name, param);

                ()
            })
    }

    pub fn add_items<T: Any, S: AsRef<[T]>>(&mut self, name: &'a str, items: S) -> Result<()> {
        let items = items.as_ref();

        if self.map.contains_key(name) {
            return Err(Error::ArgumentAlreadyExists(name.to_string()));
        }

        Param::from_items(items)
            .map(|param| {
                self.map.insert(name, param);

                ()
            })
    }
}

pub enum Params<'a> {
    Init,
    Args(HashMap<&'a str, Box<Any>>),
}

impl<'a> Params<'a> {
    pub fn init_mode() -> Self {
        Params::Init
    }

    pub fn from_vec(params: Vec<String>) -> Self {
        panic!("");
    }
}
