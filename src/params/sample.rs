pub enum Param {
    Float(f32),
    Usize(usize),
    Str(String),
}

impl Param {
    pub fn from_str(x: &str) -> Result<(String, Self)> {
        let parts = x.splitn(3, "§").collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(Error::InvalidParamTypeStr(x.to_string()));
        } 

        let (name, kind, value) = (parts[0].to_string(), parts[1], parts[2]);

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

    pub fn to_string(&self) -> String {
        match self {
            Param::Float(f) => format!("float§{}", f),
            Param::Usize(u) => format!("usize§{}", u),
            Param::Str(s) => format!("str§{}", s),
        }
    }
}

pub struct Params {
    args: HashMap<String, Param>
}

impl Params {
    pub fn from_string(params: String) -> Result<Self> {
        let parsed_params = params.split(" ")
            .map(|x| {
                Param::from_str(x)
            })
            .collect::<Result<HashMap<String, Param>>>()?;

        Ok(Params {
            args: parsed_params,
        })
    }

    pub fn to_string(self) -> Vec<String> {
        self.args.into_iter().map(|(name, val)| {
                format!("{}§{}", name, val.to_string())
            })
            .collect::<Vec<_>>()
    }

    pub fn get_usize(&self, name: &str) -> Option<usize> {
        match self.args.get(name) {
            Some(Param::Usize(x)) => Some(*x),
            _ => None
        }
    }
}
