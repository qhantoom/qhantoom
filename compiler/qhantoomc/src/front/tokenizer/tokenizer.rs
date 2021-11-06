use std::str::Chars;

use super::interface::TokenizerState;
use super::token::{Token, TokenKind, TOKEN_EOF};

use crate::{go, sh_trace, shorthand};

use crate::util::ascii::{
  is_id_continue, is_id_start, is_number_continue, is_number_start,
  is_whitespace,
};

use crate::util::error::{Error, Result};
use crate::util::symbol::Symbols;

// tokenize a string into a vector of tokens
#[inline]
pub fn tokenize(src: &str) -> Result<Vec<Token>> {
  let mut syms = Symbols::new();
  let mut tokenizer = Tokenizer::new(&src, &mut syms);

  tokenizer.tokenize()
}

pub struct Tokenizer<'a> {
  buffer: String,
  input: Chars<'a>,
  next: Option<char>,
  state: TokenizerState,
  pub syms: &'a mut Symbols,
}

impl<'a> Tokenizer<'a> {
  #[inline]
  pub fn new(input: &'a str, syms: &'a mut Symbols) -> Self {
    Self {
      buffer: String::new(),
      input: input.chars(),
      next: None,
      state: TokenizerState::Idle,
      syms,
    }
  }

  #[inline]
  pub fn next(&mut self) -> Token {
    loop {
      let c = match self.next.take().or_else(|| self.input.next()) {
        Some(c) => c,
        None => return TOKEN_EOF,
      };

      if let Some(k) = self.step(c) {
        return Token::new(k);
      }
    }
  }

  #[inline]
  fn err(&self, error: Error) {
    panic!("{}", error);
  }

  #[inline]
  fn tokenize(&mut self) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();

    loop {
      let token = self.next();

      if token.is_eof() {
        break tokens.push(token);
      }

      tokens.push(token);
    }

    Ok(tokens)
  }

  #[inline]
  fn push(&mut self, c: char) {
    self.buffer.push(c);
  }

  #[inline]
  fn step(&mut self, c: char) -> Option<TokenKind> {
    use super::token::TokenKind::*;

    match self.state {
      // read_idle_state
      TokenizerState::Idle => match c {
        // skip whitespace
        c if is_whitespace(c) => {}

        // one character tokens
        '\0' => go!(self: emit_token_kind EOF),
        '\n' => go!(self: emit_token_kind Newline),
        '\\' => go!(self: emit_token_kind BackSlash),
        '(' => go!(self: emit_token_kind OpenParen),
        ')' => go!(self: emit_token_kind CloseParen),
        '{' => go!(self: emit_token_kind OpenBrace),
        '}' => go!(self: emit_token_kind CloseBrace),
        '[' => go!(self: emit_token_kind OpenBracket),
        ']' => go!(self: emit_token_kind CloseBracket),
        ',' => go!(self: emit_token_kind Comma),
        ';' => go!(self: emit_token_kind Semicolon),
        '?' => go!(self: emit_token_kind Question),
        '@' => go!(self: emit_token_kind At),

        // n characters tokens
        '+' => go!(self: to StartAdd),
        '-' => go!(self: to StartSub),
        '*' => go!(self: to StartMul),
        '/' => go!(self: to StartDiv),
        '%' => go!(self: to StartMod),
        '=' => go!(self: to StartAssign),
        '&' => go!(self: to StartAnd),
        '!' => go!(self: to StartBang),
        '|' => go!(self: to StartPipe),
        '.' => go!(self: to StartDot),
        ':' => go!(self: to StartColon),
        '<' => go!(self: to StartLt),
        '>' => go!(self: to StartGt),
        '"' => go!(self: to StartString),
        '\'' => go!(self: to StartChar),

        // numbers
        c if is_number_start(c) => go!(self: push c; to StartNumber),
        c if is_number_continue(c) => go!(self: push c; to Number),

        // identifiers
        c if is_id_start(c) => go!(self: push c; to Identifier),

        // unexpected character
        _ => self.err(Error::UnexpectedCharacter(c)),
      },
      // read_start_add_state
      TokenizerState::StartAdd => match c {
        '=' => go!(self: to Idle; emit_token_kind AddAssign),
        c => return self.reset_back(c, TokenKind::Add),
      },
      // read_start_sub_state
      TokenizerState::StartSub => match c {
        '=' => go!(self: reset SubAssign),
        '>' => go!(self: reset ThinArrow),
        '-' => go!(self: to CommentLine),
        '!' => go!(self: to CommentLineDoc),
        '%' => go!(self: to StartCommentBlock),
        c => return self.reset_back(c, TokenKind::Sub),
      },
      // read_start_mul_state
      TokenizerState::StartMul => match c {
        '=' => go!(self: reset MulAssign),
        c => return self.reset_back(c, TokenKind::Mul),
      },
      // read_start_div_state
      TokenizerState::StartDiv => match c {
        '=' => go!(self: reset DivAssign),
        c => return self.reset_back(c, TokenKind::Div),
      },
      // read_start_mod_state
      TokenizerState::StartMod => match c {
        '=' => go!(self: reset ModAssign),
        c => return self.reset_back(c, TokenKind::Mod),
      },
      // read_start_assign_state
      TokenizerState::StartAssign => match c {
        '=' => go!(self: reset Equal),
        '>' => go!(self: reset FatArrow),
        c => return self.reset_back(c, TokenKind::Assign),
      },
      // read_start_and_state
      TokenizerState::StartAnd => match c {
        '=' => go!(self: reset AndAssign),
        '&' => go!(self: reset AndAnd),
        c => return self.reset_back(c, TokenKind::And),
      },
      // read_start_bang_state
      TokenizerState::StartBang => match c {
        '=' => go!(self: reset NotAssign),
        c => return self.reset_back(c, TokenKind::Not),
      },
      // read_start_pipe_state
      TokenizerState::StartPipe => match c {
        '=' => go!(self: reset PipeAssign),
        '|' => go!(self: reset PipePipe),
        c => return self.reset_back(c, TokenKind::Pipe),
      },
      // read_start_dot_state
      TokenizerState::StartDot => match c {
        '=' => go!(self: reset DotAssign),
        '.' => go!(self: reset DotDot),
        c => return self.reset_back(c, TokenKind::Dot),
      },
      // read_start_colon_state
      TokenizerState::StartColon => match c {
        '=' => go!(self: reset ColonAssign),
        ':' => go!(self: reset ColonColon),
        c => return self.reset_back(c, TokenKind::Colon),
      },
      // read_start_lt_state
      TokenizerState::StartLt => match c {
        '=' => go!(self: reset Le),
        '<' => go!(self: reset Shl),
        c => return self.reset_back(c, TokenKind::Lt),
      },
      // read_start_gt_state
      TokenizerState::StartGt => match c {
        '=' => go!(self: reset Ge),
        '>' => go!(self: reset Shr),
        c => return self.reset_back(c, TokenKind::Gt),
      },
      // read_start_char_state
      TokenizerState::StartChar => go!(self: push c; to InnerChar),
      // read_inner_char_state
      TokenizerState::InnerChar => match c {
        '\'' => go!(self: to EndChar),
        _ => go!(self: push c; to InnerChar),
      },
      // read_end_char_state
      TokenizerState::EndChar => go!(self: emit_char c),
      // read_start_string_state
      TokenizerState::StartString => go!(self: push c; to InnerString),
      // read_inner_string_state
      TokenizerState::InnerString => match c {
        '"' => go!(self: to EndString),
        '\\' => go!(self: to EscapeString),
        _ => go!(self: push c; to InnerString),
      },
      // read_escape_string_state
      TokenizerState::EscapeString => match c {
        '"' | '\\' => go!(self: push c; to InnerString),
        'r' => go!(self: push '\r'; to InnerString),
        'n' => go!(self: push '\n'; to InnerString),
        't' => go!(self: push '\t'; to InnerString),
        _ => self.err(Error::UnexpectedEscapeSequence(c)),
      },
      // read_end_string_state
      TokenizerState::EndString => go!(self: emit_str c),
      // read_start_number_state
      TokenizerState::StartNumber => match c {
        'x' | 'X' => go!(self: push c; to NumberHex),
        '1'..='9' => self.err(Error::UnexpectedLiteralNumber(c)),
        '.' => go!(self: push c; to NumberFloat),
        c => go!(self: emit_zero c),
      },
      // read_number_state
      TokenizerState::Number => match c {
        '0'..='9' => go!(self: push c),
        '.' => go!(self: push c; to NumberFloat),
        c => go!(self: emit_int_number c),
      },
      // read_number_float_state
      TokenizerState::NumberFloat => match c {
        '0'..='9' => go!(self: push c),
        c => go!(self: emit_float_number c),
      },
      // read_number_float_state
      TokenizerState::NumberHex => match c {
        '0'..='9' | 'a'..='f' | 'A'..='F' => go!(self: push c),
        c => go!(self: emit_hex_number c),
      },
      // read_identifier_state
      TokenizerState::Identifier => match c {
        c if is_id_continue(c) => go!(self: push c),
        c => go!(self: emit_identifier c),
      },
      // read_comment_line_state
      TokenizerState::CommentLine => {
        if c == '\n' || c == '\0' {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::CommentLine);
        }
      }
      // read_comment_doc_line_state
      TokenizerState::CommentLineDoc => {
        if c == '\n' || c == '\0' {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::CommentLineDoc);
        }
      }
      // read_comment_block_state
      TokenizerState::StartCommentBlock => go!(self: to InnerCommentBlock),
      // read_inner_comment_block_state
      TokenizerState::InnerCommentBlock => match c {
        '%' => go!(self: to EndCommentBlock),
        // TODO: start comment block never detected
        '!' => go!(self: to StartCommentDocBlock),
        _ => go!(self: to InnerCommentBlock),
      },
      // read_end_comment_block_state
      TokenizerState::EndCommentBlock => {
        if c == '-' {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::CommentBlock);
        }
      }
      // read_start_comment_doc_block_state
      TokenizerState::StartCommentDocBlock => {
        go!(self: to InnerCommentDocBlock)
      }
      // read_inner_comment_doc_block_state
      TokenizerState::InnerCommentDocBlock => match c {
        '%' => go!(self: to EndCommentDocBlock),
        _ => go!(self: to InnerCommentDocBlock),
      },
      // read_end_comment_doc_block_state
      TokenizerState::EndCommentDocBlock => {
        if c == '-' {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::CommentDocBlock);
        }
      }
    };

    None
  }

  #[inline]
  fn reset(&mut self, kind: TokenKind) -> Option<TokenKind> {
    self.state = TokenizerState::Idle;

    Some(kind)
  }

  #[inline]
  fn reset_back(&mut self, c: char, kind: TokenKind) -> Option<TokenKind> {
    self.next = Some(c);

    self.reset(kind)
  }
}

// this implementation is based on:
// https://github.com/servo/html5ever/blob/master/xml5ever/src/tokenizer/mod.rs
#[macro_export]
macro_rules! shorthand (
  ( $me:ident : push $c:expr ) => ( $me.push($c) );
  ( $me:ident : to $s:ident ) => ({ $me.state = TokenizerState::$s; });
  ( $me:ident : reset $c:expr  ) => ( return $me.reset($c) );
);

#[macro_export]
macro_rules! sh_trace ( ( $me:ident : $($cmds:tt)* ) => (
  shorthand!($me: $($cmds)*)
) );

#[macro_export]
macro_rules! go (
  ( $me:ident : $a:tt ; $($rest:tt)* ) => ({
    sh_trace!($me: $a); go!($me: $($rest)*);
  });

  ( $me:ident : $a:tt $b:tt ; $($rest:tt)* ) => ({
    sh_trace!($me: $a $b); go!($me: $($rest)*);
  });

  ( $me:ident : $a:tt $b:tt $c:tt ; $($rest:tt)* ) => ({
    sh_trace!($me: $a $b $c); go!($me: $($rest)*);
  });

  ( $me:ident : $a:tt $b:tt $c:tt $d:tt ; $($rest:tt)* ) => ({
    sh_trace!($me: $a $b $c $d); go!($me: $($rest)*);
  });

  ( $me:ident : emit_char $c:ident ) => ({
    if $me.buffer.len() == 1 {
      let buffer = $me.buffer.chars().next().expect("expected character");
      $me.buffer.clear();

      return $me.reset_back($c, TokenKind::CharAscii(buffer));
    }

    $me.err(Error::UnexpectedLiteralChar($me.buffer.clone()));
  });

  ( $me:ident : emit_zero $c:ident ) => ({
    $me.buffer.clear();

    return $me.reset_back($c, TokenKind::Int(0));
  });

  ( $me:ident : emit_token_kind $kind:expr ) => ({
    return Some($kind);
  });

  ( $me:ident : emit_int_number $c:expr) => ({
    let num = $me.buffer.to_owned();
    let num = num.parse::<i64>().unwrap();

    $me.buffer.clear();

    return $me.reset_back($c, TokenKind::Int(num));
  });

  ( $me:ident : emit_float_number $c:expr) => ({
    let num = $me.buffer.to_owned();
    let num = num.parse::<f64>().unwrap();

    $me.buffer.clear();

    return $me.reset_back($c, TokenKind::Float(num));
  });

  ( $me:ident : emit_hex_number $c:expr) => ({
    if $me.buffer.len() == 0 {
      $me.err(Error::Custom("invalid hex literal (need digits)"));
    }

    let buf = $me.buffer.to_owned();
    let without_prefix = buf.trim_start_matches("0x");
    let num = i64::from_str_radix(without_prefix, 16);

    match num {
      Ok(n) => {
        $me.buffer.clear();
        return $me.reset_back($c, TokenKind::Int(n));
      },
      Err(_) => $me.err(Error::Custom("invalid hex literal (need digits)")),
    }
  });

  ( $me:ident : emit_str $c:expr) => ({
    let buffer = $me.buffer.to_owned();
    $me.buffer.clear();

    let sym = $me.syms.intern(&buffer);

    return $me.reset_back($c, TokenKind::StrBuffer(sym));
  });

  ( $me:ident : emit_identifier $c:expr) => ({
    let buffer = $me.buffer.to_owned();
    $me.buffer.clear();

    if let Some(kind) = TokenKind::keywords(&buffer) {
      return $me.reset_back($c, kind);
    }

    let sym = $me.syms.intern(&buffer);

    return $me.reset_back($c, TokenKind::Identifier(sym));
  });

  ( $me:ident : $($cmd:tt)+ ) => ( sh_trace!($me: $($cmd)+) );

  ( $me:ident : ) => (());
);
