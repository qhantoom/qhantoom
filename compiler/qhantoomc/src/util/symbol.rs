use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

thread_local!(static SYMBOLS: RefCell<Vec<String>> = RefCell::new(Vec::new()));

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Debug)]
pub struct Symbol(pub u32);

impl Symbol {
  #[inline]
  pub fn as_usize(self) -> usize {
    self.0 as usize
  }
}

#[derive(Clone)]
pub struct SymbolTable {
  syms: Vec<String>,
  table: HashMap<String, Symbol>,
}

impl fmt::Display for Symbol {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    SYMBOLS.with(|syms| syms.borrow()[self.0 as usize].fmt(f))
  }
}

impl<'a> PartialEq<&'a str> for Symbol {
  #[inline]
  fn eq(&self, other: &&'a str) -> bool {
    SYMBOLS.with(|syms| syms.borrow()[self.0 as usize] == *other)
  }
}

impl SymbolTable {
  #[inline]
  pub fn new() -> Self {
    let syms = SYMBOLS.with(|s| s.borrow().clone());

    let table = syms
      .iter()
      .enumerate()
      .map(|(i, s)| (s.clone(), Symbol(i as u32)))
      .collect();

    Self { table, syms }
  }

  #[inline]
  pub fn intern(&mut self, s: &str) -> Symbol {
    if let Some(sym) = self.table.get(s) {
      return *sym;
    }

    let idx = self.syms.len() as u32;
    let sym = Symbol(idx);
    let s = String::from(s);

    self.table.insert(s.clone(), sym);
    self.syms.push(s);

    sym
  }

  #[inline]
  pub fn store(self) {
    SYMBOLS.with(move |syms| {
      *syms.borrow_mut() = self.syms;
    })
  }
}