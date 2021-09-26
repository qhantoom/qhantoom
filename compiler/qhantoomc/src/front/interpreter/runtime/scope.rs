use crate::front::interpreter::value::{Fun, Local};

use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Scope {
  functions: HashMap<String, Fun>,
  variables: HashMap<String, Local>,
}

impl Scope {
  #[inline]
  pub fn new() -> Self {
    Self {
      functions: HashMap::new(),
      variables: HashMap::new(),
    }
  }

  #[inline]
  pub fn bind_function(&mut self, fun: Fun) -> Result<(), String> {
    match self.get_function(&fun.name()) {
      Some(_) => Err(format!("function already exist")),
      None => Ok({
        self.functions.insert(fun.name(), fun);
      }),
    }
  }

  #[inline]
  pub fn bind_variable(&mut self, local: Local) -> Result<(), String> {
    match self.get_variable(&local.name()) {
      Some(_) => Err(format!("variable already exist")),
      None => Ok({
        self.variables.insert(local.name(), local);
      }),
    }
  }

  #[inline]
  pub fn get_function(&self, name: &str) -> Option<&Fun> {
    self.functions.get(name)
  }

  #[inline]
  pub fn get_variable(&self, name: &str) -> Option<&Local> {
    self.variables.get(name)
  }
}
