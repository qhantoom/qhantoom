use super::interface::ScopeLinked;
use super::scope::Scope;

use crate::front::interpreter::value::{Fun, Local};

#[derive(Clone, Debug, PartialEq)]
pub struct Scopemap {
  scopes: ScopeLinked<Scope>,
}

impl Scopemap {
  #[inline]
  pub fn new() -> Self {
    Self {
      scopes: ScopeLinked::new(),
    }
  }

  #[inline]
  pub fn bind(&mut self) {
    self.scopes.push_front(Scope::new());
  }

  #[inline]
  pub fn unbind(&mut self) {
    self.scopes.pop_front().unwrap();
  }

  #[inline]
  pub fn bind_variable(&mut self, local: Local) -> Result<(), String> {
    match self.scopes.front_mut() {
      Some(scope) => scope.bind_variable(local),
      None => Err(format!("bind variable error")),
    }
  }

  #[inline]
  pub fn bind_function(&mut self, fun: Fun) -> Result<(), String> {
    match self.scopes.front_mut() {
      Some(scope) => scope.bind_function(fun),
      None => Err(format!("bind fn error")),
    }
  }

  #[inline]
  pub fn get_function(&self, name: &str) -> Option<&Fun> {
    for scope in self.scopes.iter() {
      match scope.get_function(name) {
        Some(f) => return Some(f),
        None => continue,
      };
    }

    None
  }

  #[inline]
  pub fn get_variable(&self, name: &str) -> Option<&Local> {
    for scope in self.scopes.iter() {
      match scope.get_variable(name) {
        Some(v) => return Some(v),
        None => continue,
      };
    }

    None
  }
}
