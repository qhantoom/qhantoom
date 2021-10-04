use super::token::{Token, TokenKind};
use super::tokenizer::tokenize;

#[test]
fn tokenize_empty_token() {
  let file = read_file("../../samples/tokens/empty.qh");
  let tokens = tokenize(&file).unwrap();
  let expected = vec![TokenKind::EOF];

  run_test(1, tokens, expected);
}

#[test]
fn tokenize_comments_token() {
  let file = read_file("../../samples/tokens/comments.qh");
  let tokens = tokenize(&file).unwrap();

  let expected = vec![TokenKind::CommentLine, TokenKind::EOF];

  run_test(2, tokens, expected);
}

#[test]
fn tokenize_delimiters_token() {
  let file = read_file("../../samples/tokens/delimiters.qh");
  let tokens = tokenize(&file).unwrap();

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
  let file = read_file("../../samples/tokens/operators.qh");
  let tokens = tokenize(&file).unwrap();

  let expected = vec![
    TokenKind::Add,
    TokenKind::Sub,
    TokenKind::Mul,
    TokenKind::Div,
    TokenKind::Mod,
    TokenKind::Not,
    TokenKind::Assign,
    TokenKind::And,
    TokenKind::Pipe,
    TokenKind::Lt,
    TokenKind::Gt,
    TokenKind::Dot,
    TokenKind::Colon,
    TokenKind::Semicolon,
    TokenKind::Comma,
    TokenKind::BackSlash,
    TokenKind::Question,
    TokenKind::At,
    TokenKind::Underscore,
    TokenKind::Equal,
    TokenKind::AndAnd,
    TokenKind::PipePipe,
    TokenKind::Shl,
    TokenKind::Shr,
    TokenKind::DotDot,
    TokenKind::ColonColon,
    TokenKind::AddAssign,
    TokenKind::SubAssign,
    TokenKind::MulAssign,
    TokenKind::DivAssign,
    TokenKind::ModAssign,
    TokenKind::NotAssign,
    TokenKind::AndAssign,
    TokenKind::PipeAssign,
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
  let file = read_file("../../samples/tokens/ints.qh");
  let tokens = tokenize(&file).unwrap();

  let expected = vec![
    TokenKind::Int(0),
    TokenKind::Int(10),
    TokenKind::Int(1000),
    TokenKind::Int(1000000),
    TokenKind::Int(10000000000),
    TokenKind::EOF,
  ];

  run_test(6, tokens, expected);
}

fn tokenize_floats_token() {}

fn tokenize_hex_token() {}

#[test]
fn tokenize_chars_token() {
  let file = read_file("../../samples/tokens/chars.qh");
  let tokens = tokenize(&file).unwrap();

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
  let file = read_file("../../samples/tokens/strings.qh");
  let tokens = tokenize(&file).unwrap();

  let expected = vec![
    TokenKind::StrBuffer("hello, world! 👽".into()),
    TokenKind::StrBuffer("yo la mif".into()),
    TokenKind::StrBuffer("你好!".into()),
    TokenKind::StrBuffer("hello, \"world\"! 👽\nwesh la famille! 🤘".into()),
    TokenKind::EOF,
  ];

  run_test(5, tokens, expected);
}

#[test]
fn tokenize_identifiers_token() {
  let file = read_file("../../samples/tokens/identifiers.qh");
  let tokens = tokenize(&file).unwrap();

  let expected = vec![
    TokenKind::Identifier("square".into()),
    TokenKind::Identifier("cosinus".into()),
    TokenKind::Identifier("degrees".into()),
    TokenKind::Identifier("_tmp".into()),
    TokenKind::Identifier("add_tmp".into()),
    TokenKind::Identifier("to_tmp_".into()),
    TokenKind::Identifier("vector1".into()),
    TokenKind::Identifier("vector2".into()),
    TokenKind::Identifier("vector3".into()),
    TokenKind::EOF,
  ];

  run_test(10, tokens, expected);
}

#[test]
fn tokenize_keywords_token() {
  let file = read_file("../../samples/tokens/keywords.qh");
  let tokens = tokenize(&file).unwrap();

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
    TokenKind::Module,
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
    TokenKind::Where,
    TokenKind::While,
    TokenKind::EOF,
  ];

  run_test(41, tokens, expected);
}

fn tokenize_tys_token() {
  let file = read_file("../../samples/tokens/tys.qh");
  let tokens = tokenize(&file).unwrap();

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
  ];

  run_test(16, tokens, expected);
}

fn read_file(path: &str) -> String {
  match crate::util::reader::readfile(&path) {
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
