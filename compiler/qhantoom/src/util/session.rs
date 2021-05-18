use std::cell::RefCell;
use std::rc::Rc;

use super::errors;
use super::span::Span;
use super::symbol::Symgen;

#[inline]
pub fn session() -> Rc<Session> {
  thread_local! {
    static SESSION: Rc<Session> = Rc::new(Session::new(
      Symgen::new(),
      RefCell::new(false),
    ))
  };

  SESSION.with(|o| Rc::clone(o))
}

pub struct Session {
  pub symgen: Symgen,
  pub errors: RefCell<bool>,
}

impl Session {
  #[inline]
  pub const fn new(symgen: Symgen, errors: RefCell<bool>) -> Self {
    Self { symgen, errors }
  }

  #[inline]
  pub fn err(&self, msg: &str) {
    errors::error(msg);
    *self.errors.borrow_mut() = true;
  }

  #[inline]
  pub fn abort(&self, msg: &str, span: Span) -> ! {
    errors::error_at(msg, span);
    *self.errors.borrow_mut() = true;
    errors::abort()
  }
}
