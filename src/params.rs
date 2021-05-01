use std::ops::Range;
use std::any::{Any, type_name};
use std::collections::HashMap;
use std::hash::Hash;
use std::any::TypeId;

use crate::error::{Result, Error};

pub enum ParamSet<T> {
    Range(Range<T>),
    Items(Vec<T>),
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
}

pub struct ParamBuilder<'a> {
    map: HashMap<&'a str, ParamType>,
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
                self.map.insert(name, param);

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
                self.map.insert(name, param);

                ()
            })
    }
}

pub enum Param {
    Float(f32),
    Usize(usize),
    Str(String),
}

impl Param {
    pub fn from_str<'a>(x: &'a str) -> Result<(&'a str, Self)> {
        let parts = x.splitn(3, "§").collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(Error::InvalidParamTypeStr(x.to_string()));
        } 

        let (kind, name, value) = (parts[0], parts[1], parts[2]);

        if kind == "usize" {
            return Ok((name, Param::Usize(value.parse()?)));
        } else if kind == "float" {
            return Ok((name, Param::Float(value.parse()?)));
        } else if kind == "str" {
            return Ok((name, Param::Str(value.to_string())));
        } else {
            return Err(Error::InvalidParamTypeStr(x.to_string()));
        }
    }
}

pub enum Params<'a> {
    Init,
    Args(HashMap<&'a str, Param>),
}

impl<'a> Params<'a> {
    pub fn init_mode() -> Self {
        Params::Init
    }

    pub fn from_vec(params: &'a str) -> Result<Self> {
        if params == "init" {
            return Ok(Params::Init);
        } 

        params.split(" ")
            .map(|x| {
                Param::from_str(x)
            })
            .collect::<Result<HashMap<&'a str, Param>>>()
            .map(|x| Params::Args(x))
    }

    pub fn is_init_mode(&self) -> bool {
        matches!(self, Params::Init)
    }
}
