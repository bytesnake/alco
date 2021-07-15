use std::ops::Range;
use std::any::{Any, type_name};
use std::collections::HashMap;
use std::any::TypeId;

use crate::error::{Result, Error};
use super::samples::{Sample, Samples};

pub trait ParamType {
    fn num_items(&self) -> Option<usize> {
        None
    }

    fn for_step(&self, step: usize) -> Option<Sample>;
}

pub struct UsizeRange(Range<usize>, usize);

impl ParamType for UsizeRange{
    fn for_step(&self, step: usize) -> Option<Sample> {
        let tmp = self.0.start + step * self.1;

        if tmp >= self.0.end {
            return None;
        } else {
            return Some(Sample::Usize(tmp));
        }
    }
}


pub struct FloatRange(Range<f32>, f32);

impl ParamType for FloatRange {
    fn for_step(&self, step: usize) -> Option<Sample> {
        let tmp = self.0.start + (step as f32) * self.1;

        if tmp >= self.0.end {
            return None;
        } else {
            return Some(Sample::Float(tmp));
        }
    }
}

pub struct Items(Vec<Sample>);

impl ParamType for Items {
    fn num_items(&self) -> Option<usize> {
        Some(self.0.len())
    }

    fn for_step(&self, step: usize) -> Option<Sample> {
        if step >= self.0.len() {
            return None
        } else {
            self.0[step].clone()
        }
    }
}

pub struct ParamBuilder<'a> {
    map: HashMap<&'a str, Box<ParamType>>,
}

impl<'a> ParamBuilder<'a> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    pub fn add_usize_range(&mut self, name: &'a str, range: Range<usize>) -> Result<()> {
        if self.map.contains_key(name) {
            return Err(Error::ArgumentAlreadyExists(name.to_string()));
        }

        self.map.insert(name, UsizeRange(range, 1));

        Ok(())
    }

    pub fn add_float_range(&mut self, name: &'a str, range: Range<f32>, min_val: f32) -> Result<()> {
        if self.map.contains_key(name) {
            return Err(Error::ArgumentAlreadyExists(name.to_string()));
        }

        self.map.insert(name, FloatRange(range, min_val));

        Ok(())
    }

    pub fn add_items<T: Any, S: AsRef<[T]>>(&mut self, name: &'a str, items: S) -> Result<()> {
        let items = items.as_ref();

        if self.map.contains_key(name) {
            return Err(Error::ArgumentAlreadyExists(name.to_string()));
        }

        self.map.insert(name, Items(items.clone()));

        Ok(())
    }

    pub fn indices(&self, indices: &[usize]) -> Sample {
        let res = self.map.iter().zip(indices.iter()).map(|(m, idx)| {
            (m.0, m.1.0.for_step(idx))
        }).collect();

        Samples::new(res)
    }

    pub fn lower_bound(&self) -> Samples {
        let zeros = vec![0; self.map.len()];

        Self::indices(&zeros)
    }

    pub fn params(&self) -> Vec<&str> {
        self.map.keys().into_iter().cloned().collect()
    }

    pub fn update_step(&self, prev: Samples, name: &str, step: usize) -> Samples {
        prev
    }
}
