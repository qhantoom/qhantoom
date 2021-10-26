use std::collections::HashMap;

use cranelift::prelude::Block;

#[derive(Debug, Clone)]
pub struct Scope<V> {
  variables: HashMap<String, V>,
}

impl<V> Scope<V> {
  #[inline]
  fn new() -> Self {
    Self {
      variables: HashMap::new(),
    }
  }

  #[inline]
  fn get_variable(&self, name: &str) -> Option<&V> {
    self.variables.get(name)
  }

  #[inline]
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

pub struct ScopeMap<V> {
  blocks: Vec<Block>,
  maps: Vec<Scope<V>>,
}

impl<V> ScopeMap<V> {
  #[inline]
  pub fn new() -> Self {
    Self {
      blocks: vec![],
      maps: vec![Scope::new()],
    }
  }

  #[inline]
  pub fn blocks(&mut self) -> &mut Vec<Block> {
    &mut self.blocks
  }

  #[inline]
  pub fn get_variable(&self, name: &str) -> Option<&V> {
    for map in self.maps.iter().rev() {
      if let Some(v) = map.get_variable(name) {
        return Some(v);
      }
    }

    None
  }

  #[inline]
  pub fn add_variable(&mut self, name: String, var: V) -> Result<(), String> {
    match self.maps.last_mut() {
      Some(map) => map.add_variable(name, var),
      None => Err(format!("empty scope map")), // TODO: should be an error type
    }
  }

  #[inline]
  pub fn bind(&mut self) {
    self.maps.push(Scope::new());
  }

  #[inline]
  pub fn unbind(&mut self) {
    if self.maps.len() > 1 {
      self.maps.pop();
    }
  }
}
