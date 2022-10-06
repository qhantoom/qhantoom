use crate::front::parser::ast::Ty;

use std::collections::HashMap;

#[derive(Debug, Default)]
struct Scope {
  decls: HashMap<String, Ty>,
  funs: HashMap<String, (Ty, Vec<Ty>)>,
  tys: HashMap<String, Ty>,
}

impl Scope {
  fn decl(&self, name: &str) -> Option<&Ty> {
    self.decls.get(name)
  }

  fn fun(&self, name: &str) -> Option<&(Ty, Vec<Ty>)> {
    self.funs.get(name)
  }

  fn ty(&self, name: &str) -> Option<&Ty> {
    self.tys.get(name)
  }

  fn set_decl(&mut self, name: String, ty: Ty) -> Result<(), String> {
    match self.decls.get(&name) {
      Some(_) => Err(format!("variable `{}` already exists", name)),
      None => {
        self.decls.insert(name, ty);
        Ok(())
      }
    }
  }

  fn set_fun(&mut self, name: String, ty: (Ty, Vec<Ty>)) -> Result<(), String> {
    match self.funs.get(&name) {
      Some(_) => Err(format!("function `{}` already exists", name)),
      None => {
        self.funs.insert(name, ty);
        Ok(())
      }
    }
  }

  fn set_ty(&mut self, name: String, ty: Ty) -> Result<(), String> {
    match self.tys.get(&name) {
      Some(_) => Err(format!("type `{}` already exists", name)),
      None => {
        self.tys.insert(name, ty);
        Ok(())
      }
    }
  }
}

#[derive(Debug)]
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

  pub fn decl(&self, name: &str) -> Option<&Ty> {
    for map in self.maps.iter().rev() {
      if let Some(decl) = map.decl(name) {
        return Some(decl);
      }
    }

    None
  }

  pub fn fun(&self, name: &str) -> Option<&(Ty, Vec<Ty>)> {
    for map in self.maps.iter().rev() {
      if let Some(fun) = map.fun(name) {
        return Some(fun);
      }
    }

    None
  }

  pub fn ty(&self, name: &str) -> Option<&Ty> {
    for map in self.maps.iter().rev() {
      if let Some(ty) = map.ty(name) {
        return Some(ty);
      }
    }

    None
  }

  pub fn set_decl(&mut self, name: String, ty: Ty) -> Result<(), String> {
    match self.maps.last_mut() {
      Some(map) => map.set_decl(name, ty),
      None => Err(format!("variable {} value do not exist", name)),
    }
  }

  pub fn set_fun(
    &mut self,
    name: String,
    ty: (Ty, Vec<Ty>),
  ) -> Result<(), String> {
    match self.maps.last_mut() {
      Some(map) => map.set_fun(name, ty),
      None => Err(format!("function {} value do not exist", name)),
    }
  }

  pub fn set_ty(&mut self, name: String, ty: Ty) -> Result<(), String> {
    match self.maps.last_mut() {
      Some(map) => map.set_ty(name, ty),
      None => Err(format!("type {} value do not exist", name)),
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
