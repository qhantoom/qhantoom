use super::token::{Token, TokenKind};
use super::Tokenizer;

use crate::util::symbol::{Symbol, Symbols};

#[test]
fn tokenize_empty_token() {
  let file = read_file("../../samples/tests/tokens/empty.qh");
  let tokens = tokenize(&file);
  let expected = vec![TokenKind::EOF];

  run_test(1, tokens, expected);
}

#[test]
fn tokenize_comments_token() {
  let file = read_file("../../samples/tests/tokens/comments.qh");
  let tokens = tokenize(&file);

  let expected = vec![
    TokenKind::CommentLine,
    TokenKind::CommentLine,
    TokenKind::CommentDocLine,
    TokenKind::CommentDocLine,
    TokenKind::CommentBlock,
    TokenKind::EOF,
  ];

  run_test(6, tokens, expected);
}

#[test]
fn tokenize_delimiters_token() {
  let file = read_file("../../samples/tests/tokens/delimiters.qh");
  let tokens = tokenize(&file);

  let expected = vec![
    TokenKind::OpenParen,
    TokenKind::CloseParen,
    TokenKind::OpenBrace,
    TokenKind::CloseBrace,
    TokenKind::OpenBracket,
    TokenKind::CloseBracket,
    TokenKind::EOF,
  ];

  run_test(7, tokens, expected);
}

#[test]
fn tokenize_operators_token() {
  let file = read_file("../../samples/tests/tokens/operators.qh");
  let tokens = tokenize(&file);

  let expected = vec![
    TokenKind::Add,
    TokenKind::Sub,
    TokenKind::Mul,
    TokenKind::Div,
    TokenKind::Rem,
    TokenKind::Not,
    TokenKind::Assign,
    TokenKind::And,
    TokenKind::Or,
    TokenKind::Lt,
    TokenKind::Gt,
    TokenKind::Dot,
    TokenKind::Colon,
    TokenKind::Semi,
    TokenKind::Comma,
    TokenKind::Num,
    TokenKind::Question,
    TokenKind::At,
    TokenKind::Underscore,
    TokenKind::Eq,
    TokenKind::AndAnd,
    TokenKind::OrOr,
    TokenKind::Shl,
    TokenKind::Shr,
    TokenKind::DotDot,
    TokenKind::ColonColon,
    TokenKind::AddAssign,
    TokenKind::SubAssign,
    TokenKind::MulAssign,
    TokenKind::DivAssign,
    TokenKind::RemAssign,
    TokenKind::Ne,
    TokenKind::BitAndAssign,
    TokenKind::BitOrAssign,
    TokenKind::Le,
    TokenKind::Ge,
    TokenKind::DotAssign,
    TokenKind::ColonAssign,
    TokenKind::ThinArrow,
    TokenKind::FatArrow,
    TokenKind::EOF,
  ];

  run_test(41, tokens, expected);
}

#[test]
fn tokenize_ints_token() {
  let file = read_file("../../samples/tests/tokens/ints.qh");
  let tokens = tokenize(&file);

  let expected = vec![
    TokenKind::Int(0),
    TokenKind::Int(10),
    TokenKind::Int(1000),
    TokenKind::Int(1000000),
    TokenKind::Int(10000000000),
    TokenKind::Int(123456789),
    TokenKind::EOF,
  ];

  run_test(7, tokens, expected);
}

#[test]
fn tokenize_floats_token() {
  let file = read_file("../../samples/tests/tokens/floats.qh");
  let tokens = tokenize(&file);

  let expected = vec![
    TokenKind::Float(0.5),
    TokenKind::Float(1.0),
    TokenKind::Float(230.47),
    TokenKind::Float(30949.374),
    TokenKind::Float(0.123_456_789),
    TokenKind::EOF,
  ];

  run_test(6, tokens, expected);
}

#[test]
fn tokenize_hex_token() {
  let file = read_file("../../samples/tests/tokens/hex.qh");
  let tokens = tokenize(&file);

  let expected = vec![
    TokenKind::Int(100),
    TokenKind::Int(2736),
    TokenKind::Int(41120),
    TokenKind::Int(64250),
    TokenKind::Int(256),
    TokenKind::EOF,
  ];

  run_test(6, tokens, expected);
}

#[test]
fn tokenize_chars_token() {
  let file = read_file("../../samples/tests/tokens/chars.qh");
  let tokens = tokenize(&file);

  let expected = vec![
    TokenKind::CharAscii('a'),
    TokenKind::CharAscii('b'),
    TokenKind::CharAscii('c'),
    TokenKind::CharAscii('b'),
    TokenKind::CharAscii('c'),
    TokenKind::CharAscii('d'),
    TokenKind::CharAscii('c'),
    TokenKind::CharAscii('d'),
    TokenKind::CharAscii('e'),
    TokenKind::EOF,
  ];

  run_test(10, tokens, expected);
}

#[test]
fn tokenize_strings_token() {
  let file = read_file("../../samples/tests/tokens/strings.qh");
  let tokens = tokenize(&file);

  let expected = vec![
    TokenKind::StrBuffer(Symbol(0)),
    TokenKind::StrBuffer(Symbol(1)),
    TokenKind::StrBuffer(Symbol(2)),
    TokenKind::StrBuffer(Symbol(3)),
    TokenKind::EOF,
  ];

  run_test(5, tokens, expected);
}

#[test]
fn tokenize_identifiers_token() {
  let file = read_file("../../samples/tests/tokens/identifiers.qh");
  let tokens = tokenize(&file);

  let expected = vec![
    TokenKind::Identifier(Symbol(0)),
    TokenKind::Identifier(Symbol(1)),
    TokenKind::Identifier(Symbol(2)),
    TokenKind::Identifier(Symbol(3)),
    TokenKind::Identifier(Symbol(4)),
    TokenKind::Identifier(Symbol(5)),
    TokenKind::Identifier(Symbol(6)),
    TokenKind::Identifier(Symbol(7)),
    TokenKind::Identifier(Symbol(8)),
    TokenKind::EOF,
  ];

  run_test(10, tokens, expected);
}

#[test]
fn tokenize_keywords_token() {
  let file = read_file("../../samples/tests/tokens/keywords.qh");
  let tokens = tokenize(&file);

  let expected = vec![
    TokenKind::Action,
    TokenKind::As,
    TokenKind::Async,
    TokenKind::Await,
    TokenKind::Bench,
    TokenKind::Bind,
    TokenKind::Break,
    TokenKind::Chan,
    TokenKind::Continue,
    TokenKind::Else,
    TokenKind::Enum,
    TokenKind::Exp,
    TokenKind::Ext,
    TokenKind::False,
    TokenKind::Fun,
    TokenKind::For,
    TokenKind::If,
    TokenKind::Imu,
    TokenKind::Load,
    TokenKind::Loop,
    TokenKind::Match,
    TokenKind::MeUpper,
    TokenKind::MeLower,
    TokenKind::Mock,
    TokenKind::Mod,
    TokenKind::Mut,
    TokenKind::Pub,
    TokenKind::Ref,
    TokenKind::Return,
    TokenKind::Set,
    TokenKind::Spawn,
    TokenKind::Struct,
    TokenKind::Test,
    TokenKind::Type,
    TokenKind::Unit,
    TokenKind::Val,
    TokenKind::Void,
    TokenKind::Wasm,
    TokenKind::While,
    TokenKind::EOF,
  ];

  run_test(40, tokens, expected);
}

#[test]
fn tokenize_tys_token() {
  let file = read_file("../../samples/tests/tokens/tys.qh");
  let tokens = tokenize(&file);

  let expected = vec![
    TokenKind::U8,
    TokenKind::U16,
    TokenKind::U32,
    TokenKind::U64,
    TokenKind::UInt,
    TokenKind::S8,
    TokenKind::S16,
    TokenKind::S32,
    TokenKind::S64,
    TokenKind::SInt,
    TokenKind::F32,
    TokenKind::F64,
    TokenKind::Bool,
    TokenKind::Char,
    TokenKind::Str,
    TokenKind::EOF,
  ];

  run_test(16, tokens, expected);
}

fn read_file(path: &str) -> String {
  match crate::util::reader::read_file(&path) {
    Ok(f) => f,
    Err(e) => panic!("{}", e),
  }
}

fn run_test(len: usize, tokens: Vec<Token>, expected: Vec<TokenKind>) {
  assert_eq!(tokens.len(), len);

  for (i, token) in tokens.iter().enumerate() {
    assert_eq!(*token.kind(), expected[i]);
  }
}

fn tokenize(file: &str) -> Vec<Token> {
  let mut syms = Symbols::new();
  let mut tokenizer = Tokenizer::new(file, &mut syms);

  tokenizer
    .tokenize()
    .into_iter()
    .filter(|t| *t.kind() != TokenKind::Newline)
    .collect::<Vec<Token>>()
}
