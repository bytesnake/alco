use std::ops::Range;
use std::any::Any;
use std::collections::HashMap;

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
            Some(self.0[step].clone())
        }
    }
}

pub struct ParamBuilder<'a> {
    map: HashMap<&'a str, Box<dyn ParamType>>,
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

        self.map.insert(name, Box::new(UsizeRange(range, 1)));

        Ok(())
    }

    pub fn add_float_range(&mut self, name: &'a str, range: Range<f32>, min_val: f32) -> Result<()> {
        if self.map.contains_key(name) {
            return Err(Error::ArgumentAlreadyExists(name.to_string()));
        }

        self.map.insert(name, Box::new(FloatRange(range, min_val)));

        Ok(())
    }

    pub fn add_items<T: Any, S: AsRef<[T]>>(&mut self, name: &'a str, items: S) -> Result<()> {
        let items = items.as_ref();

        if self.map.contains_key(name) {
            return Err(Error::ArgumentAlreadyExists(name.to_string()));
        }

        let items = items.iter().map(|x| {
            let x = x as &dyn Any;
            if let Some(x) = x.downcast_ref::<usize>() {
                Sample::Usize(*x)
            } else if let Some(x) = x.downcast_ref::<f32>() {
                Sample::Float(*x)
            } else if let Some(x) = x.downcast_ref::<f64>() {
                Sample::Float(*x as f32)
            } else if let Some(x) = x.downcast_ref::<String>() {
                Sample::Str(x.to_string())
            } else {
                panic!("Not support type");
            }
        }).collect();

        self.map.insert(name, Box::new(Items(items)));

        Ok(())
    }

    pub fn from_indices(&self, indices: Vec<(&str, usize)>) -> Option<Samples> {
        let res = indices.into_iter().map(|(name, val)| {
            let t = self.map.get(name).unwrap();
            t.for_step(val).map(|x| (name.to_string(), x))
        }).collect::<Option<HashMap<_, _>>>();

        res.map(Samples::new)
    }

    pub fn lower_bound(&self) -> Samples {
        let zeros = self.map.keys().map(|key| (*key, 0)).collect();

        self.from_indices(zeros).unwrap()
    }

    pub fn params(&self) -> &HashMap<&'a str, Box<dyn ParamType>> {
        &self.map
    }

    pub fn update_step(&self, prev: Samples, name: &str, step: usize) -> Samples {
        let mut samples = prev.samples();

        // create new value with given step
        let new_val = {
            let t = self.map.get(name).unwrap();
            t.for_step(step).unwrap()
        };

        // insert into map and return a new sample set
        samples.insert(name.to_string(), new_val);

        Samples::new(samples)
    }
}
