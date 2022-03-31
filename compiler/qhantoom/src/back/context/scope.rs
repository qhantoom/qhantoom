use std::collections::HashMap;

use cranelift::prelude::Block;

#[derive(Debug, Clone)]
pub struct Scope<D, V> {
  data: HashMap<String, D>,
  variables: HashMap<String, V>,
}

impl<D, V> Scope<D, V> {
  fn new() -> Self {
    Self {
      data: HashMap::new(),
      variables: HashMap::new(),
    }
  }

  fn get_data(&self, name: &str) -> Option<&D> {
    self.data.get(name)
  }

  fn get_variable(&self, name: &str) -> Option<&V> {
    self.variables.get(name)
  }

  fn add_data(&mut self, name: String, var: D) -> Result<(), String> {
    match self.data.get(&name) {
      Some(_) => Err(format!("data value {} already exists", name)), // TODO: should be an error type
      None => {
        self.data.insert(name, var);
        Ok(())
      }
    }
  }

  fn add_variable(&mut self, name: String, var: V) -> Result<(), String> {
    match self.variables.get(&name) {
      Some(_) => Err(format!("variable {} already exists", name)), // TODO: should be an error type
      None => {
        self.variables.insert(name, var);
        Ok(())
      }
    }
  }
}

pub struct ScopeMap<D, V> {
  blocks: Vec<Block>,
  maps: Vec<Scope<D, V>>,
}

impl<D, V> ScopeMap<D, V> {
  pub fn new() -> Self {
    Self {
      blocks: vec![],
      maps: vec![Scope::new()],
    }
  }

  pub fn blocks(&mut self) -> &mut Vec<Block> {
    &mut self.blocks
  }

  pub fn get_data(&self, name: &str) -> Option<&D> {
    for map in self.maps.iter().rev() {
      if let Some(v) = map.get_data(name) {
        return Some(v);
      }
    }

    None
  }

  pub fn get_variable(&self, name: &str) -> Option<&V> {
    for map in self.maps.iter().rev() {
      if let Some(v) = map.get_variable(name) {
        return Some(v);
      }
    }

    None
  }

  pub fn add_data(&mut self, name: String, data: D) -> Result<(), String> {
    match self.maps.last_mut() {
      Some(map) => map.add_data(name, data),
      None => Err(format!("data value do not exist")), // TODO: should be an error type
    }
  }

  pub fn add_variable(&mut self, name: String, var: V) -> Result<(), String> {
    match self.maps.last_mut() {
      Some(map) => map.add_variable(name, var),
      None => Err(format!("empty scope map")), // TODO: should be an error type
    }
  }

  pub fn bind(&mut self) {
    self.maps.push(Scope::new());
  }

  pub fn unbind(&mut self) {
    if self.maps.len() > 1 {
      self.maps.pop();
    }
  }
}
