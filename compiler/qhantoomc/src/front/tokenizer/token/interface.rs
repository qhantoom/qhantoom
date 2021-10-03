#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
  EOF,
  Newline,
  Int(u64),
  Float(f64),
  CharAscii(char),
  StrBuffer(String),
  Identifier(String),

  // comments
  CommentLine,

  // operators
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  Not,
  And,
  Pipe,
  Comma,
  Dot,
  Colon,
  Semicolon,
  Assign,
  Bang,
  Slash,
  BackSlash,
  Gt,
  Ge,
  Lt,
  Le,
  Shl,
  Shr,
  At,
  Question,
  Underscore,
  Equal,
  AddAssign,
  SubAssign,
  DivAssign,
  MulAssign,
  ModAssign,
  PipeAssign,
  AndAssign,
  DotAssign,
  NotAssign,
  ColonAssign,
  ColonColon,
  DotDot,
  AndAnd,
  PipePipe,
  FatArrow,
  ThinArrow,

  // delimiters
  OpenParen,
  CloseParen,
  OpenBrace,
  CloseBrace,
  OpenBracket,
  CloseBracket,

  // keywords
  Action,
  As,
  Async,
  Await,
  Bench,
  Bind,
  Break,
  Chan,
  Continue,
  Else,
  Enum,
  Exp,
  Ext,
  False,
  Fun,
  For,
  If,
  Imu,
  Load,
  Loop,
  Match,
  MeUpper,
  MeLower,
  Mock,
  Module,
  Mut,
  Pub,
  Ref,
  Return,
  Set,
  Spawn,
  Struct,
  Test,
  True,
  Type,
  Unit,
  Val,
  Void,
  Wasm,
  Where,
  While,

  // types
  Bool,
  Char,
  Str,
  F32,
  F64,
  S8,
  S16,
  S32,
  S64,
  SInt,
  U8,
  U16,
  U32,
  U64,
  UInt,
}

impl TokenKind {
  #[inline]
  pub fn is(&self, k: TokenKind) -> bool {
    *self == k
  }

  #[inline]
  pub fn is_eof(&self) -> bool {
    *self == Self::EOF
  }

  #[inline]
  pub fn keywords(ident: &str) -> Option<Self> {
    match ident {
      // keywords
      "action" => Some(Self::Action),
      "as" => Some(Self::As),
      "async" => Some(Self::Async),
      "await" => Some(Self::Await),
      "bench" => Some(Self::Bench),
      "bind" => Some(Self::Bind),
      "break" => Some(Self::Break),
      "chan" => Some(Self::Chan),
      "continue" => Some(Self::Continue),
      "else" => Some(Self::Else),
      "enum" => Some(Self::Enum),
      "exp" => Some(Self::Exp),
      "ext" => Some(Self::Ext),
      "false" => Some(Self::False),
      "fun" => Some(Self::Fun),
      "for" => Some(Self::For),
      "if" => Some(Self::If),
      "imu" => Some(Self::Imu),
      "load" => Some(Self::Load),
      "loop" => Some(Self::Loop),
      "match" => Some(Self::Match),
      "Me" => Some(Self::MeUpper),
      "me" => Some(Self::MeLower),
      "mock" => Some(Self::Mock),
      "mod" => Some(Self::Module),
      "mut" => Some(Self::Mut),
      "pub" => Some(Self::Pub),
      "ref" => Some(Self::Ref),
      "return" => Some(Self::Return),
      "set" => Some(Self::Set),
      "spawn" => Some(Self::Spawn),
      "struct" => Some(Self::Struct),
      "test" => Some(Self::Test),
      "true" => Some(Self::True),
      "type" => Some(Self::Type),
      "_" => Some(Self::Underscore),
      "unit" => Some(Self::Unit),
      "val" => Some(Self::Val),
      "void" => Some(Self::Void),
      "wasm" => Some(Self::Wasm),
      "where" => Some(Self::Where),
      "while" => Some(Self::While),
      // types
      "bool" => Some(Self::Bool),
      "char" => Some(Self::Char),
      "str" => Some(Self::Str),
      "f32" => Some(Self::F32),
      "f64" => Some(Self::F64),
      "s8" => Some(Self::S8),
      "s16" => Some(Self::S16),
      "s32" => Some(Self::S32),
      "s64" => Some(Self::S64),
      "sint" => Some(Self::SInt),
      "u8" => Some(Self::U8),
      "u16" => Some(Self::U16),
      "u32" => Some(Self::U32),
      "u64" => Some(Self::U64),
      "uint" => Some(Self::UInt),
      _ => None,
    }
  }
}
