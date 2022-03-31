use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

thread_local!(static SYMBOLS: RefCell<Vec<String>> = RefCell::new(Vec::new()));

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Debug)]
pub struct Symbol(pub u32);

impl fmt::Display for Symbol {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    SYMBOLS.with(|syms| write!(f, "{}", syms.borrow()[self.0 as usize]))
  }
}

impl<'a> PartialEq<&'a str> for Symbol {
  fn eq(&self, other: &&'a str) -> bool {
    SYMBOLS.with(|syms| syms.borrow()[self.0 as usize] == *other)
  }
}

impl Symbol {
  pub fn as_usize(self) -> usize {
    self.0 as usize
  }
}

#[derive(Clone)]
pub struct Symbols {
  syms: Vec<String>,
  table: HashMap<String, Symbol>,
}

impl Symbols {
  pub fn new() -> Self {
    let syms = SYMBOLS.with(|s| s.borrow().clone());

    let table = syms
      .iter()
      .enumerate()
      .map(|(i, s)| (s.to_owned(), Symbol(i as u32)))
      .collect();

    Self { table, syms }
  }

  pub fn intern(&mut self, s: &str) -> Symbol {
    if let Some(sym) = self.table.get(s) {
      return *sym;
    }

    let idx = self.syms.len() as u32;
    let sym = Symbol(idx);
    let s = String::from(s);

    self.table.insert(s.to_owned(), sym);
    self.syms.push(s);

    sym
  }

  pub fn store(self) {
    SYMBOLS.with(move |syms| {
      *syms.borrow_mut() = self.syms;
    })
  }
}
