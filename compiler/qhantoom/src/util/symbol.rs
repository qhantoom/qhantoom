use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

thread_local!(static SYMBOLS: RefCell<Vec<String>> = RefCell::new(vec![]));

pub type SymbolIndex = u32;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Debug)]
pub struct Symbol(SymbolIndex);

impl fmt::Display for Symbol {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl<'a> PartialEq<&'a str> for Symbol {
  #[inline]
  fn eq(&self, rhs: &&'a str) -> bool {
    SYMBOLS.with(|s| s.borrow()[self.0 as usize] == *rhs)
  }
}

impl Symbol {
  #[inline]
  pub fn id(&self) -> u32 {
    self.0
  }

  #[inline]
  pub fn to_usize(&self) -> usize {
    self.0 as usize
  }

  #[inline]
  pub fn text(&self) -> String {
    SYMBOLS.with(|s| format!("{}", s.borrow()[self.to_usize()]))
  }
}

#[derive(Debug, Clone)]
pub struct Symgen {
  symbols: RefCell<Vec<String>>,
  table: RefCell<HashMap<String, Symbol>>,
}

impl Symgen {
  #[inline]
  pub fn new() -> Self {
    let symbols = SYMBOLS.with(|s| s.borrow().clone());

    let table = symbols
      .iter()
      .enumerate()
      .map(|(i, s)| (s.clone(), Symbol(i as u32)))
      .collect();

    Self {
      table: RefCell::new(table),
      symbols: RefCell::new(symbols),
    }
  }

  #[inline]
  pub fn intern(&self, buf: &str) -> Symbol {
    let mut symbols = self.symbols.borrow_mut();
    let mut table = self.table.borrow_mut();

    if let Some(symbol) = table.get(buf) {
      return *symbol;
    }

    let symbol = Symbol(symbols.len() as u32);
    let buf = String::from(buf);

    table.insert(buf.clone(), symbol);
    symbols.push(buf);

    symbol
  }

  #[inline]
  pub fn store(&self) {
    let symbols = self.symbols.borrow();

    SYMBOLS.with(move |s| *s.borrow_mut() = symbols.to_vec())
  }
}
