use std::str::Chars;

use super::interface::TokenizerState;
use super::token::{Token, TokenKind};

use crate::util::ascii::{
  is_id_continue, is_id_start, is_number_continue, is_number_start,
  is_whitespace,
};

use crate::util::error::{Error, Result};

// tokenize a string into a vector of tokens
#[inline]
pub fn tokenize(src: &str) -> Result<Vec<Token>> {
  let mut tokenizer = Tokenizer::new(&src);

  tokenizer.tokenize()
}

pub struct Tokenizer<'a> {
  buffer: String,
  input: Chars<'a>,
  next: Option<char>,
  state: TokenizerState,
}

impl<'a> Tokenizer<'a> {
  #[inline]
  pub fn new(input: &'a str) -> Self {
    Self {
      buffer: String::new(),
      input: input.chars(),
      next: None,
      state: TokenizerState::Idle,
    }
  }

  #[inline]
  pub fn next(&mut self) -> Token {
    loop {
      let c = match self.next.take().or_else(|| self.input.next()) {
        Some(c) => c,
        None => return Token::new(TokenKind::EOF),
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

  // TODO: use macros
  #[inline]
  fn step(&mut self, c: char) -> Option<TokenKind> {
    match self.state {
      // read_idle_state
      TokenizerState::Idle => match c {
        // skip whitespace
        c if is_whitespace(c) => {}

        // one character tokens
        '\0' => return Some(TokenKind::EOF),
        '\n' => return Some(TokenKind::Newline),
        '\\' => return Some(TokenKind::BackSlash),
        '(' => return Some(TokenKind::OpenParen),
        ')' => return Some(TokenKind::CloseParen),
        '{' => return Some(TokenKind::OpenBrace),
        '}' => return Some(TokenKind::CloseBrace),
        '[' => return Some(TokenKind::OpenBracket),
        ']' => return Some(TokenKind::CloseBracket),
        ',' => return Some(TokenKind::Comma),
        ';' => return Some(TokenKind::Semicolon),
        '?' => return Some(TokenKind::Question),
        '@' => return Some(TokenKind::At),

        // n characters tokens
        '+' => {
          self.state = TokenizerState::StartAdd;
        }
        '-' => {
          self.state = TokenizerState::StartSub;
        }
        '*' => {
          self.state = TokenizerState::StartMul;
        }
        '/' => {
          self.state = TokenizerState::StartDiv;
        }
        '%' => {
          self.state = TokenizerState::StartMod;
        }
        '=' => {
          self.state = TokenizerState::StartAssign;
        }
        '&' => {
          self.state = TokenizerState::StartAnd;
        }
        '!' => {
          self.state = TokenizerState::StartBang;
        }
        '|' => {
          self.state = TokenizerState::StartPipe;
        }
        '.' => {
          self.state = TokenizerState::StartDot;
        }
        ':' => {
          self.state = TokenizerState::StartColon;
        }
        '<' => {
          self.state = TokenizerState::StartLt;
        }
        '>' => {
          self.state = TokenizerState::StartGt;
        }
        '"' => {
          self.state = TokenizerState::StartString;
        }
        '\'' => {
          self.state = TokenizerState::StartChar;
        }

        // numbers
        c if is_number_start(c) => {
          self.state = TokenizerState::StartNumber;
          self.buffer.push(c);
        }
        c if is_number_continue(c) => {
          self.state = TokenizerState::Number;
          self.buffer.push(c);
        }

        // identifiers
        c if is_id_start(c) => {
          self.state = TokenizerState::Identifier;
          self.buffer.push(c);
        }

        // unexpected character
        _ => self.err(Error::UnexpectedCharacter(c)),
      },
      // read_start_add_state
      TokenizerState::StartAdd => match c {
        '=' => {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::AddAssign);
        }
        c => {
          return self.reset_back(c, TokenKind::Add);
        }
      },
      // read_start_sub_state
      TokenizerState::StartSub => match c {
        '=' => {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::SubAssign);
        }
        '>' => {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::ThinArrow);
        }
        '-' => {
          self.state = TokenizerState::CommentLine;
        }
        '!' => {
          self.state = TokenizerState::CommentLineDoc;
        }
        '%' => {
          self.state = TokenizerState::StartCommentBlock;
        }
        c => {
          return self.reset_back(c, TokenKind::Sub);
        }
      },
      // read_start_mul_state
      TokenizerState::StartMul => match c {
        '=' => {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::MulAssign);
        }
        c => {
          return self.reset_back(c, TokenKind::Mul);
        }
      },
      // read_start_div_state
      TokenizerState::StartDiv => match c {
        '=' => {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::DivAssign);
        }
        c => {
          return self.reset_back(c, TokenKind::Div);
        }
      },
      // read_start_mod_state
      TokenizerState::StartMod => match c {
        '=' => {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::ModAssign);
        }
        c => {
          return self.reset_back(c, TokenKind::Mod);
        }
      },
      // read_start_assign_state
      TokenizerState::StartAssign => match c {
        '=' => {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::Equal);
        }
        '>' => {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::FatArrow);
        }
        c => {
          return self.reset_back(c, TokenKind::Assign);
        }
      },
      // read_start_and_state
      TokenizerState::StartAnd => match c {
        '=' => {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::AndAssign);
        }
        '&' => {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::AndAnd);
        }
        c => {
          return self.reset_back(c, TokenKind::And);
        }
      },
      // read_start_bang_state
      TokenizerState::StartBang => match c {
        '=' => {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::NotAssign);
        }
        c => {
          return self.reset_back(c, TokenKind::Not);
        }
      },
      // read_start_pipe_state
      TokenizerState::StartPipe => match c {
        '=' => {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::PipeAssign);
        }
        '|' => {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::PipePipe);
        }
        c => {
          return self.reset_back(c, TokenKind::Pipe);
        }
      },
      // read_start_dot_state
      TokenizerState::StartDot => match c {
        '=' => {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::DotAssign);
        }
        '.' => {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::DotDot);
        }
        c => {
          return self.reset_back(c, TokenKind::Dot);
        }
      },
      // read_start_colon_state
      TokenizerState::StartColon => match c {
        '=' => {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::ColonAssign);
        }
        ':' => {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::ColonColon);
        }
        c => {
          return self.reset_back(c, TokenKind::Colon);
        }
      },
      // read_start_lt_state
      TokenizerState::StartLt => match c {
        '=' => {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::Le);
        }
        '<' => {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::Shl);
        }
        c => {
          return self.reset_back(c, TokenKind::Lt);
        }
      },
      // read_start_gt_state
      TokenizerState::StartGt => match c {
        '=' => {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::Ge);
        }
        '>' => {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::Shr);
        }
        c => {
          return self.reset_back(c, TokenKind::Gt);
        }
      },
      // read_start_char_state
      TokenizerState::StartChar => {
        self.state = TokenizerState::InnerChar;
        self.buffer.push(c);
      }
      // read_inner_char_state
      TokenizerState::InnerChar => match c {
        '\'' => {
          self.state = TokenizerState::EndChar;
        }
        _ => {
          self.state = TokenizerState::InnerChar;
          self.buffer.push(c);
        }
      },
      // read_end_char_state
      TokenizerState::EndChar => {
        if self.buffer.len() == 1 {
          self.state = TokenizerState::Idle;
          let buffer = self.buffer.chars().next().expect("expected character");
          self.buffer.clear();

          return self.reset_back(c, TokenKind::CharAscii(buffer));
        }

        self.err(Error::UnexpectedLiteralChar(self.buffer.clone()));
      }
      // read_start_string_state
      TokenizerState::StartString => {
        self.state = TokenizerState::InnerString;
        self.buffer.push(c);
      }
      // read_inner_string_state
      TokenizerState::InnerString => match c {
        '"' => {
          self.state = TokenizerState::EndString;
        }
        '\\' => {
          self.state = TokenizerState::EscapeString;
        }
        _ => {
          self.state = TokenizerState::InnerString;
          self.buffer.push(c);
        }
      },
      // read_escape_string_state
      TokenizerState::EscapeString => match c {
        '"' | '\\' => {
          self.state = TokenizerState::InnerString;
          self.buffer.push(c);
        }
        'r' => {
          self.state = TokenizerState::InnerString;
          self.buffer.push('\r');
        }
        'n' => {
          self.state = TokenizerState::InnerString;
          self.buffer.push('\n');
        }
        't' => {
          self.state = TokenizerState::InnerString;
          self.buffer.push('\t');
        }
        _ => self.err(Error::UnexpectedEscapeSequence(c)),
      },
      // read_end_string_state
      TokenizerState::EndString => {
        self.state = TokenizerState::Idle;
        let buffer = self.buffer.to_owned();
        self.buffer.clear();

        return self.reset_back(c, TokenKind::StrBuffer(buffer));
      }
      // read_start_number_state
      TokenizerState::StartNumber => match c {
        '1'..='9' => {
          self.err(Error::UnexpectedLiteralNumber(c));
        }
        '.' => {
          self.state = TokenizerState::NumberFloat;
          self.buffer.push(c);
        }
        c => {
          self.buffer.clear();

          return self.reset_back(c, TokenKind::Int(0));
        }
      },
      // read_number_state
      TokenizerState::Number => match c {
        '0'..='9' => {
          self.buffer.push(c);
        }
        '.' => {
          self.state = TokenizerState::NumberFloat;
          self.buffer.push(c);
        }
        c => {
          let num = self.buffer.to_owned();
          let num = num.parse::<i64>().unwrap();
          self.buffer.clear();

          return self.reset_back(c, TokenKind::Int(num));
        }
      },
      // read_number_float_state
      TokenizerState::NumberFloat => match c {
        '0'..='9' => {
          self.buffer.push(c);
        }
        c => {
          let num = self.buffer.to_owned();
          let num = num.parse::<f64>().unwrap();
          self.buffer.clear();

          return self.reset_back(c, TokenKind::Float(num));
        }
      },
      // read_identifier_state
      TokenizerState::Identifier => match c {
        c if is_id_continue(c) => {
          self.buffer.push(c);
        }
        c => {
          let buffer = self.buffer.to_owned();
          self.buffer.clear();

          if let Some(kind) = TokenKind::keywords(&buffer) {
            return self.reset_back(c, kind);
          }

          return self.reset_back(c, TokenKind::Identifier(buffer));
        }
      },
      // read_comment_line_state
      TokenizerState::CommentLine => {
        if c == '\n' || c == '\0' {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::CommentLine);
        }

        // TODO: error handling
      }
      // read_comment_doc_line_state
      TokenizerState::CommentLineDoc => {
        if c == '\n' || c == '\0' {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::CommentLineDoc);
        }

        // TODO: error handling
      }
      // read_comment_block_state
      TokenizerState::StartCommentBlock => {
        self.state = TokenizerState::InnerCommentBlock;
      }
      // read_inner_comment_block_state
      TokenizerState::InnerCommentBlock => match c {
        '%' => {
          self.state = TokenizerState::EndCommentBlock;
        }
        '!' => {
          // TODO: start comment block never detected
          self.state = TokenizerState::StartCommentDocBlock;
        }
        _ => {
          self.state = TokenizerState::InnerCommentBlock;
        }
      },
      // read_end_comment_block_state
      TokenizerState::EndCommentBlock => {
        if c == '-' {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::CommentBlock);
        }

        // TODO: error handling
      }
      // read_start_comment_doc_block_state
      TokenizerState::StartCommentDocBlock => {
        self.state = TokenizerState::InnerCommentDocBlock;
      }
      // read_inner_comment_doc_block_state
      TokenizerState::InnerCommentDocBlock => match c {
        '%' => {
          self.state = TokenizerState::EndCommentDocBlock;
        }
        _ => {
          self.state = TokenizerState::InnerCommentDocBlock;
        }
      },
      // read_end_comment_doc_block_state
      TokenizerState::EndCommentDocBlock => {
        if c == '-' {
          self.state = TokenizerState::Idle;
          return Some(TokenKind::CommentDocBlock);
        }

        // TODO: error handling
      }
    };

    return None;
  }

  #[inline]
  fn reset(&mut self, kind: TokenKind) -> Option<TokenKind> {
    self.state = TokenizerState::Idle;

    return Some(kind);
  }

  #[inline]
  fn reset_back(&mut self, c: char, kind: TokenKind) -> Option<TokenKind> {
    self.next = Some(c);

    return self.reset(kind);
  }
}
