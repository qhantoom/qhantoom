use std::cmp;

pub const SPAN_ZERO: Span = Span::zero();

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Span {
  pub lo: u32,
  pub hi: u32,
}

impl Span {
  #[inline]
  pub const fn zero() -> Self {
    Self { lo: 0, hi: 0 }
  }

  #[inline]
  pub const fn new(lo: u32, hi: u32) -> Self {
    Self { lo, hi }
  }

  #[inline]
  pub fn extend(self, span: Span) -> Self {
    Self {
      lo: self.lo,
      hi: span.hi,
    }
  }

  #[inline]
  pub fn merge(lo: usize, hi: usize) -> Span {
    let start = cmp::min(lo, hi) as u32;
    let end = cmp::max(lo, hi) as u32;

    Span::new(start, end)
  }
}
