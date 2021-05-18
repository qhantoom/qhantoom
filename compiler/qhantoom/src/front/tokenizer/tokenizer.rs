use std::collections::HashSet;
use std::path::Path;
use std::str::{CharIndices, FromStr};

use crate::util::ascii::*;
use crate::util::string::string;

use crate::util::reader;
use crate::util::session::session;
use crate::util::span::{Span, SPAN_ZERO};
use crate::util::symbol::Symbol;

use super::interface::TokenizerState;

use super::token::{
  NumberSize, Token,
  TokenKind::{self, *},
};

#[inline]
pub fn tokenize_capsule_from_file(path: &Path) -> Result<Vec<Token>, String> {
  let file = reader::readfile(path)?;
  let tokenizer = Tokenizer::new(&file);
  let tokens = tokenizer.collect::<Vec<Token>>();

  Ok(tokens)
}

#[inline]
pub fn tokenize_capsule_from_source(src: &str) -> Result<Vec<Token>, String> {
  let tokenizer = Tokenizer::new(src);
  let tokens = tokenizer.collect::<Vec<Token>>();

  Ok(tokens)
}

pub struct Tokenizer<'a> {
  input: CharIndices<'a>,
  buffer: String,
  next: Option<(usize, char)>,
  span: Span,
  current_char: char,
  state: TokenizerState,
  types: HashSet<Symbol>,
}

impl<'a> Iterator for Tokenizer<'a> {
  type Item = Token;

  fn next(&mut self) -> Option<Token> {
    let t = self.advance_token();

    if t.is_eof() {
      return None;
    }

    Some(t)
  }
}

impl<'a> Tokenizer<'a> {
  #[inline]
  pub fn new(input: &'a str) -> Self {
    Self {
      input: input.char_indices(),
      buffer: string![""],
      next: None,
      span: SPAN_ZERO,
      current_char: '\0',
      state: TokenizerState::StartState,
      types: HashSet::new(),
    }
  }

  #[inline]
  fn bind_ty(&mut self, ty: Symbol) {
    self.types.insert(ty);
  }

  #[inline]
  pub fn advance_token(&mut self) -> Token {
    let mut start = None;

    loop {
      let (pos, c) = match self.next.take().or_else(|| self.input.next()) {
        Some(pair) => pair,
        None => return Token::new(TokenKind::EOF, SPAN_ZERO),
      };

      let start = match start {
        Some(s) => s,
        None => {
          start = Some(pos);
          pos
        }
      };

      let end = pos + c.len_utf8();
      let span = self.merge(start, end);
      self.current_char = c;
      self.span = span;

      if let Some(kind) = self.step(c) {
        self.span = SPAN_ZERO;
        return Token::new(kind, span);
      }
    }
  }

  #[inline]
  fn emit(&mut self, kind: TokenKind) -> Option<TokenKind> {
    Some(kind)
  }

  #[inline]
  fn error(&mut self, msg: String) -> ! {
    session().abort(&msg, self.span)
  }

  #[inline]
  fn error_char(&mut self, c: char) -> ! {
    self.error(format!("unknown character: `{}`", c))
  }

  #[inline]
  fn error_zero(&mut self, c: char) -> ! {
    self.error(format!("invalid number: `{}`", c))
  }

  #[inline]
  fn keyword(&self, name: &str) -> Option<TokenKind> {
    TokenKind::keyword(name)
  }

  #[inline]
  fn push(&mut self, c: char) {
    self.buffer.push(c);
  }

  #[inline]
  fn is_end_of_file(&self) -> bool {
    is_end_of_file(self.current_char)
  }

  #[inline]
  fn is_newline(&self) -> bool {
    is_newline(self.current_char)
  }

  #[inline]
  fn is_reserved_keyword(&self, name: &str) -> bool {
    TokenKind::is_reserved_keyword(name)
  }

  #[inline]
  fn is_type_keyword(&mut self, symbol: &Symbol) -> bool {
    self.types.contains(symbol)
  }

  #[inline]
  fn merge(&self, lo: usize, hi: usize) -> Span {
    Span::merge(lo, hi)
  }

  #[inline]
  fn reset(&mut self, kind: TokenKind) -> Option<TokenKind> {
    self.state = TokenizerState::StartState;
    Some(kind)
  }

  #[inline]
  fn reset_back(&mut self, c: char) {
    assert!(self.next.is_none());
    self.next = Some((self.span.hi as usize - c.len_utf8(), c));
  }

  #[inline]
  fn scan_comment_line(&mut self) -> TokenKind {
    self.push(self.current_char);

    self.advance_token();
    while self.current_char != '\0' {
      self.push(self.current_char);
      print!("\nchar: {}", self.current_char);
      self.advance_token();
      self.advance_token();
    }

    TokenKind::CommentLine(self.buffer.to_string())
  }

  #[inline]
  fn scan_ident(&mut self) -> TokenKind {
    if let Some(kind) = self.keyword(&self.buffer) {
      return kind;
    }

    if self.is_reserved_keyword(&self.buffer) {
      self.error(format!("reserved keyword"));
    }

    let symbol = session().symgen.intern(&self.buffer);

    if self.is_type_keyword(&symbol) {
      return TokenKind::Type(symbol);
    }

    TokenKind::Ident(symbol)
  }

  #[inline]
  fn scan_number(&mut self, base: NumberSize) -> TokenKind {
    if self.buffer.len() == 0 {
      self.error(format!("invalid hex"));
    }

    self.scan_number_kind_from(*base)
  }

  #[inline]
  fn scan_number_kind_from(&mut self, base: u32) -> TokenKind {
    let num = &self.buffer.replace("_", "");

    if let Ok(n) = u64::from_str_radix(num, base) {
      if (base == 8 && n <= i32::MAX as u64)
        || (base == 10 && n <= i32::MAX as u64 + 1)
        || (base == 16 && n <= i32::MAX as u64)
      {
        return TokenKind::IntNumber(n as i32);
      }
    }

    if let Ok(n) = f64::from_str(num) {
      if n <= f32::MAX as f64 + 1.0 {
        return TokenKind::FloatNumber(n as f32);
      }
    }

    self.error(format!("invalid number"))
  }

  #[inline]
  pub fn scan_string(&mut self) -> TokenKind {
    self.push(self.current_char);

    while self.current_char != '"' {
      self.advance_token();
      self.push(self.current_char);
    }

    TokenKind::StrBuffer(self.buffer.to_string())
  }

  fn step(&mut self, c: char) -> Option<TokenKind> {
    match self.state {
      TokenizerState::StartState => match c {
        c if is_whitespace(c) => {}
        c if is_newline(c) => go!(self: emit NewLine),
        c if is_comment_start(c) => go!(self: push c; to CommentState),
        c if is_id_start(c) => go!(self: push c; to IdentState),
        c if is_number_start(c) => go!(self: push c; to ZeroState),
        c if is_number_continue(c) => go!(self: push c; to NumberState),
        c if is_quote(c) => go!(self: to QuoteState),

        '+' => go!(self: to AddState),
        '-' => go!(self: to SubState),
        '*' => go!(self: to MulState),
        '/' => go!(self: to DivState),
        '%' => go!(self: to ModState),
        '!' => go!(self: to BangState),
        '.' => go!(self: to DotState),
        ':' => go!(self: to ColonState),
        '&' => go!(self: to AndState),
        '<' => go!(self: to LtState),
        '>' => go!(self: to GtState),
        '=' => go!(self: to EqState),
        '|' => go!(self: to PipeState),

        '?' => go!(self: emit QuestionMark),
        '\\' => go!(self: emit BackSlash),
        ';' => go!(self: emit Semicolon),
        ',' => go!(self: emit Comma),
        '_' => go!(self: emit Underscore),
        '(' => go!(self: emit OpenParen),
        ')' => go!(self: emit CloseParen),
        '{' => go!(self: emit OpenBrace),
        '}' => go!(self: emit CloseBrace),
        '[' => go!(self: emit OpenBracket),
        ']' => go!(self: emit CloseBracket),

        _ => go!(self: error_char c),
      },
      TokenizerState::EqState => match c {
        '=' => go!(self: reset EqEq),
        '>' => go!(self: reset ArrowFat),
        _ => go!(self: reset_back c; reset Eq),
      },
      TokenizerState::AddState => match c {
        '+' => go!(self: reset AddAdd),
        '=' => go!(self: reset AddEq),
        _ => go!(self: reset_back c; reset Add),
      },
      TokenizerState::SubState => match c {
        '-' => go!(self: reset SubSub),
        '=' => go!(self: reset SubEq),
        '>' => go!(self: reset Arrow),
        _ => go!(self: reset_back c; reset Sub),
      },
      TokenizerState::MulState => match c {
        '*' => go!(self: reset MulMul),
        '=' => go!(self: reset MulEq),
        _ => go!(self: reset_back c; reset Mul),
      },
      TokenizerState::DivState => match c {
        '/' => go!(self: reset DivDiv),
        '=' => go!(self: reset DivEq),
        _ => go!(self: reset_back c; reset Div),
      },
      TokenizerState::ModState => match c {
        '%' => go!(self: reset ModMod),
        '=' => go!(self: reset ModEq),
        _ => go!(self: reset_back c; reset Mod),
      },
      TokenizerState::BangState => match c {
        '!' => go!(self: reset BangBang),
        '=' => go!(self: reset BangEq),
        _ => go!(self: reset_back c; reset Bang),
      },
      TokenizerState::DotState => match c {
        '.' => go!(self: reset DotDot),
        '=' => go!(self: reset DotEq),
        _ => go!(self: reset_back c; reset Dot),
      },
      TokenizerState::ColonState => match c {
        ':' => go!(self: reset ColonColon),
        '=' => go!(self: reset ColonEq),
        _ => go!(self: reset_back c; reset Colon),
      },
      TokenizerState::AndState => match c {
        '&' => go!(self: reset AndAnd),
        '=' => go!(self: reset AndEq),
        _ => go!(self: reset_back c; reset And),
      },
      TokenizerState::LtState => match c {
        '<' => go!(self: reset LtLt),
        '=' => go!(self: reset LtEq),
        _ => go!(self: reset_back c; reset Lt),
      },
      TokenizerState::GtState => match c {
        '>' => go!(self: reset GtGt),
        '=' => go!(self: reset GtEq),
        _ => go!(self: reset_back c; reset Gt),
      },
      TokenizerState::PipeState => match c {
        '|' => go!(self: reset PipePipe),
        '=' => go!(self: reset PipeEq),
        _ => go!(self: reset_back c; reset Pipe),
      },
      TokenizerState::NumberState => match c {
        c if c == 'b' => go!(self: push c; into Bin; to DecState),
        c if is_oct_start(c) => go!(self: push c; into Oct; to OctState),
        c if is_hex_start(c) => go!(self: push c; into Dec; to HexState),
        c if is_number(c) || c == '_' || c == '.' || c == 'e' || c == 'E' => {
          go!(self: push c; into Dec; to DecState)
        }
        c => go!(self: scan_number c),
      },
      TokenizerState::ZeroState => match c {
        c if is_oct_start(c) => go!(self: push c; into Oct; to OctState),
        c if is_hex_start(c) => go!(self: push c; into Dec; to HexState),
        c if is_number_continue(c) => go!(self: error_zero c),
        c => go!(self: reset_back c; reset IntNumber(0)),
      },
      TokenizerState::DecState => match c {
        c if is_number(c) || c == '_' => go!(self: push c),
        c => go!(self: scan_number c),
      },
      TokenizerState::OctState => match c {
        c if is_oct_continue(c) => go!(self: push c),
        c => go!(self: scan_number c),
      },
      TokenizerState::HexState => match c {
        c if is_hex_continue(c) => go!(self: push c),
        c => go!(self: scan_number c),
      },
      TokenizerState::ExpState => match c {
        c if c == 'e' || c == 'E' => go!(self: push c),
        c if is_number(c) => go!(self: push c),
        c => go!(self: scan_number c),
      },
      TokenizerState::CommentState => match c {
        _ => go!(self: scan_comment_line),
      },
      TokenizerState::QuoteState => match c {
        '\'' => go!(self: push c; to CharState),
        '"' => go!(self: push c; to StringState),
        _ => go!(self: error_char c),
      },
      TokenizerState::CharState => match c {
        '\'' => go!(self: push c; to StartState),
        ch => go!(self: emit_char ch),
      },
      TokenizerState::StringState => match c {
        '"' => go!(self: push c; to StartState),
        _ => go!(self: scan_string),
      },
      TokenizerState::IdentState => match c {
        c if is_id_continue(c) => go!(self: push c),
        _ => go!(self: scan_ident),
      },
      _ => {}
    }

    None
  }
}

// this DSL is inspired by the little DSL from xml5ever for sequencing shorthand actions.
// @see https://github.com/servo/html5ever/blob/master/xml5ever/src/tokenizer/mod.rs#L561
macro go {
  ( $me:ident : $a:tt ; $($rest:tt)* ) => ({ sh_trace!($me: $a); go!($me: $($rest)*); }),
  ( $me:ident : $a:tt $b:tt ; $($rest:tt)* ) => ({ sh_trace!($me: $a $b); go!($me: $($rest)*); }),
  ( $me:ident : to $s:ident ) => ({ $me.state = TokenizerState::$s; }),
  ( $me:ident : emit $k:ident ) => ( return $me.emit(TokenKind::$k); ),
  ( $me:ident : emit_char $k:ident ) => ( return $me.emit(TokenKind::CharAscii($k)); ),
  ( $me:ident : reset $n:expr ) => ( return $me.reset($n); ),

  ( $me:ident : scan_ident ) => ({
    let kind = $me.scan_ident();

    $me.buffer.truncate(0);

    return $me.reset(kind);
  }),

  ( $me:ident : scan_number $n:expr ) => ({
    let num = $me.scan_number(NumberSize::Dec);

    $me.buffer.truncate(0);
    $me.reset_back($n);

    return $me.reset(num);
  }),

  ( $me:ident : scan_comment_line ) => ({
    let kind = $me.scan_comment_line();

    $me.buffer.truncate(0);
    print!("\nOOOOO: {:?}", kind);

    return $me.reset(kind);
  }),

  ( $me:ident : scan_string ) => ({
    let kind = $me.scan_string();


    $me.buffer.truncate(0);

    return $me.reset(kind);
  }),

  ( $me:ident : $($cmd:tt)+ ) => ( sh_trace!($me: $($cmd)+); ),
  ( $me:ident : ) => (()),
}

macro sh_trace { ( $me:ident : $($cmds:tt)* ) => (
  shorthand!($me: $($cmds)*)
) }

macro shorthand {
  ( $me:ident : push $n:expr ) => ( $me.push($n); ),
  ( $me:ident : into $n:expr ) => (),
  ( $me:ident : error_char $s:expr ) => ( $me.error_char($s); ),
  ( $me:ident : error_zero $s:expr ) => ( $me.error_zero($s); ),
  ( $me:ident : reset_back $s:expr ) => ( $me.reset_back($s); ),
}
