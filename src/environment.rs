use std::{collections::HashMap, io::{Error, ErrorKind}};

use crate::{ast::Value, scanner::Token};

pub struct EnvironmentArena {
    pub envs: Vec<Environment>
}

impl EnvironmentArena {
    pub fn new() -> EnvironmentArena {
        EnvironmentArena { envs: vec![].into() }
    }

    pub fn add(&mut self, parent: Option<usize>) -> usize {
        let next_index = self.envs.len();

        let env = Environment { index: next_index, parent, values: HashMap::new() };

        self.envs.push(env);
        next_index
    }

    pub fn default_env(&mut self) -> &mut Environment {
        self.envs.get_mut(0).expect("default env")
    }

    pub fn define(&mut self, env: usize, name: &str, value: Value) {
        self.envs.get_mut(env).expect("env").values.insert(name.into(), value);
    }

    pub fn get(&self, env: usize, name: &Token) -> Result<&Value, Error> {
        let env = self.envs.get(env).expect("env");
        if env.values.contains_key(&name.lexeme) {
            return Ok(env.values.get(&name.lexeme).unwrap());
        }

        if let Some(parent) = env.parent {
            return self.get(parent, name);
        }

        Err(Error::new(ErrorKind::Other, format!("Undefined variable '{}'", &name.lexeme)))
    }

    pub fn assign(&mut self, env: usize, name: &Token, value: Value) -> Result<(), Error> {
        let env = self.envs.get_mut(env).expect("env");
        if env.values.contains_key(&name.lexeme) {
            env.values.insert(name.lexeme.clone(), value);
            return Ok(());
        }

        if let Some(parent) = env.parent {
            return self.assign(parent, name, value);
        }

        Err(Error::new(ErrorKind::Other, format!("Undefined variable '{}'", &name.lexeme)))
    }
}

pub struct Environment {
    index: usize,
    parent: Option<usize>,
    values: HashMap<String, Value>
}