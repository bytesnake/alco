use std::collections::HashMap;
use crate::error::{Result, Error};

#[derive(Clone)]
pub enum Sample {
    Float(f32),
    Usize(usize),
    Str(String),
}

impl Sample {
    pub fn from_str(x: &str) -> Result<(String, Self)> {
        let parts = x.splitn(3, "§").collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(Error::InvalidParamTypeStr(x.to_string()));
        } 

        let (name, kind, value) = (parts[0].to_string(), parts[1], parts[2]);

        if kind == "usize" {
            return Ok((name, Sample::Usize(value.parse()?)));
        } else if kind == "float" {
            return Ok((name, Sample::Float(value.parse()?)));
        } else if kind == "str" {
            return Ok((name, Sample::Str(value.to_string())));
        } else {
            return Err(Error::InvalidParamTypeStr(x.to_string()));
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Sample::Float(f) => format!("float§{}", f),
            Sample::Usize(u) => format!("usize§{}", u),
            Sample::Str(s) => format!("str§{}", s),
        }
    }
}

#[derive(Clone)]
pub struct Samples {
    setup_run: bool,
    args: HashMap<String, Sample>
}

impl Samples {
    pub fn new(args: HashMap<String, Sample>) -> Self {
        Samples { 
            setup_run: false,
            args
        }
    }

    pub fn from_string(setup_run: bool, params: String) -> Result<Self> {
        let parsed_params = params.split(" ")
            .map(|x| {
                Sample::from_str(x)
            })
            .collect::<Result<HashMap<String, Sample>>>()?;

        Ok(Samples {
            setup_run,
            args: parsed_params,
        })
    }

    pub fn to_string(&self) -> Vec<String> {
        self.args.iter().map(|(name, val)| {
                format!("{}§{}", name, val.to_string())
            })
            .collect::<Vec<_>>()
    }

    pub fn get_usize(&self, name: &str) -> Option<usize> {
        match self.args.get(name) {
            Some(Sample::Usize(x)) => Some(*x),
            _ => None
        }
    }

    pub fn samples(self) -> HashMap<String, Sample> {
        self.args
    }

    pub fn is_setup(&self) -> bool {
        self.setup_run
    }

    pub fn setup_run(mut self, setup_run: bool) -> Self {
        self.setup_run = setup_run;

        self
    }
}
