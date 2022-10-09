use crate::front::parser::ast::{PBox, Ty};

use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
struct Scope {
  decls: HashMap<String, PBox<Ty>>,
  funs: HashMap<String, (PBox<Ty>, Vec<PBox<Ty>>)>,
}

impl Scope {
  fn decl(&self, name: &str) -> Option<&PBox<Ty>> {
    self.decls.get(name)
  }

  fn fun(&self, name: &str) -> Option<&(PBox<Ty>, Vec<PBox<Ty>>)> {
    self.funs.get(name)
  }

  fn set_decl(&mut self, name: String, ty: PBox<Ty>) -> Result<(), String> {
    match self.decls.get(&name) {
      Some(_) => Err(format!("variable `{}` already exists", name)),
      None => {
        self.decls.insert(name, ty);
        Ok(())
      }
    }
  }

  fn set_fun(
    &mut self,
    name: String,
    ty: (PBox<Ty>, Vec<PBox<Ty>>),
  ) -> Result<(), String> {
    match self.funs.get(&name) {
      Some(_) => Err(format!("function `{}` already exists", name)),
      None => {
        self.funs.insert(name, ty);
        Ok(())
      }
    }
  }
}

#[derive(Clone, Debug)]
pub struct ScopeMap {
  maps: Vec<Scope>,
}

impl ScopeMap {
  pub fn enter_scope(&mut self) {
    self.maps.push(Scope::default());
  }

  pub fn exit_scope(&mut self) {
    if self.maps.len() > 1 {
      self.maps.pop();
    }
  }

  pub fn decl(&self, name: &str) -> Option<&PBox<Ty>> {
    for map in self.maps.iter().rev() {
      if let Some(decl) = map.decl(name) {
        return Some(decl);
      }
    }

    None
  }

  pub fn fun(&self, name: &str) -> Option<&(PBox<Ty>, Vec<PBox<Ty>>)> {
    for map in self.maps.iter().rev() {
      if let Some(fun) = map.fun(name) {
        return Some(fun);
      }
    }

    None
  }

  pub fn set_decl(&mut self, name: String, ty: PBox<Ty>) -> Result<(), String> {
    match self.maps.last_mut() {
      Some(map) => map.set_decl(name, ty),
      None => Err(format!("variable {} value do not exist", name)),
    }
  }

  pub fn set_fun(
    &mut self,
    name: String,
    ty: (PBox<Ty>, Vec<PBox<Ty>>),
  ) -> Result<(), String> {
    match self.maps.last_mut() {
      Some(map) => map.set_fun(name, ty),
      None => Err(format!("function {} value do not exist", name)),
    }
  }
}

impl Default for ScopeMap {
  fn default() -> Self {
    Self {
      maps: vec![Scope::default()],
    }
  }
}
