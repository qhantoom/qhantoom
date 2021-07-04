use super::ast::{
  Block, Capsule, Expr, ExprKind, FunDecl, HashKind, Item, ItemKind, Mod, Stmt,
  StmtKind, TyKind, UnopKind,
};

use std::fmt;

impl fmt::Display for Capsule {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Capsule {
  fn text(&self) -> String {
    format!("{}", self.module.text())
  }
}

impl fmt::Display for Mod {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Mod {
  fn text(&self) -> String {
    let items = self
      .items()
      .iter()
      .map(|i| i.kind().text())
      .collect::<Vec<String>>()
      .join("");

    format!("{}", items)
  }
}

impl fmt::Display for Item {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Item {
  #[inline]
  fn text(&self) -> String {
    format!("{}", self.kind.text())
  }
}

impl fmt::Display for ItemKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl ItemKind {
  #[inline]
  pub fn text(&self) -> String {
    match *self {
      Self::Fun(ref fun) => format!("{}", fun.text()),
      Self::Mod(ref module) => format!("{:?}", module.text()),
      Self::Empty => format!("empty"),
    }
  }
}

impl fmt::Display for Block {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Block {
  pub fn text(&self) -> String {
    format!("{:?}", &self.stmts)
  }
}

impl fmt::Display for Stmt {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Stmt {
  #[inline]
  pub fn text(&self) -> String {
    format!("{}", self.kind.text())
  }
}

impl fmt::Display for StmtKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl StmtKind {
  #[inline]
  fn text(&self) -> String {
    match *self {
      Self::Empty => format!("empty"),
      _ => format!("empty"), // tmp
    }
  }
}

impl fmt::Display for FunDecl {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl FunDecl {
  fn text(&self) -> String {
    let name = self.name();
    let ty = &self.ty;
    let args = &self.args;
    let block = &self.block;

    format!("fun {:?} : {:?} = ({:?}) {{ {:?} }}", name, ty, args, block)
  }
}

impl Expr {
  #[inline]
  fn text(&self) -> String {
    format!("{}", self.kind.text())
  }
}

impl fmt::Display for Expr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl ExprKind {
  pub fn text(&self) -> String {
    match *self {
      //       Self::Array(ref data) => {
      //         format!(
      //           "{}",
      //           data
      //             .iter()
      //             .map(|d| d.kind.text())
      //             .collect::<Vec<String>>()
      //             .join(", ")
      //         )
      //       }
      //       Self::Binary(ref lhs, ref binop, ref rhs) => {
      //         format!("{:?} {:?} {:?}", lhs.kind, binop, rhs.kind)
      //       }
      //       Self::Call(ref callee, ref args) => {
      //         format!(
      //           "{}({})",
      //           callee.kind.text(),
      //           args
      //             .iter()
      //             .map(|a| a.kind.text())
      //             .collect::<Vec<String>>()
      //             .join(", ")
      //         )
      //       }
      //       // Self::Comment(ref kind, ref value) => match kind {
      //       //   Line => format!("# {}", value),
      //       //   _ => unreachable!(),
      //       // },
      //       Self::Closure(ref fn_decl) | Self::FunDecl(ref fn_decl) => {
      //         format!("{:?}", fn_decl)
      //       }
      //       Self::Hash(ref data) => {
      //         format!(
      //           "{{ {} }}",
      //           data
      //             .iter()
      //             .map(|(key, value)| format!("{} : {}", key, value.kind.text()))
      //             .collect::<Vec<String>>()
      //             .join(", ")
      //         )
      //       }
      //       Self::Ident(ref name) => {
      //         format!("{}", name)
      //       }
      //       Self::Local(ref local) => {
      //         format!(
      //           "val {} : {} = {};",
      //           local.name,
      //           local.ty.as_ref().unwrap().kind().to_string(),
      //           local.value.as_ref().unwrap().kind.text()
      //         )
      //       }
      //       Self::Return(ref ins) => {
      //         format!("{}", ins.as_ref().unwrap().kind.text())
      //       }
      //       Self::SheBang(ref ins) => {
      //         format!("{}", ins)
      //       }
      //       Self::If(ref condition, ref consequence, ref alternative) => {
      //         format!(
      //           "if {:?} {{ {:?} }} else {{ {:?} }}",
      //           condition.kind.text(),
      //           consequence,
      //           alternative,
      //         )
      //       }
      //       Self::Index(ref lhs, ref rhs) => {
      //         format!("{:?}[{:?}]", lhs.kind, rhs.kind)
      //       }
      //       Self::Lit(ref kind) => match kind {
      //         LitKind::Bool(ref value) => format!("{}", value),
      //         LitKind::Char(ref value) => format!("{}", value),
      //         LitKind::Float(ref value) => format!("{}", value),
      //         LitKind::Int(ref value) => format!("{}", value),
      //         LitKind::Str(ref value) => format!("{}", value),
      //       },
      //       Self::Loop(ref kind) => format!("loop {{ {:?} }}", kind),
      //       Self::Unary(ref op, ref rhs) => format!("{:?}{:?}", op, rhs.kind),
      Self::Empty => format!("empty"),
      _ => format!("empty"),
    }
  }
}

impl fmt::Display for HashKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl HashKind {
  fn text(&self) -> String {
    match self {
      Self::Bool(value) => format!("{}", value),
      Self::Int(value) => format!("{}", value),
      Self::Str(value) => format!("{}", value),
    }
  }
}

impl fmt::Display for TyKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl TyKind {
  fn text(&self) -> String {
    let text = match self {
      Self::Array(..) => "[]",
      Self::Fun(..) => "\\ () ->",
      Self::Struct(..) => "struct",
      Self::Bool => "bool",
      Self::Char => "char",
      Self::Float => "float",
      Self::Int => "int",
      Self::Str => "str",
      Self::Void => "void",
    };

    format!("{}", text)
  }
}

impl fmt::Display for UnopKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl UnopKind {
  pub fn text(&self) -> &'static str {
    match *self {
      Self::Neg => "-",
      Self::Not => "!",
      Self::Deref => "*",
    }
  }
}
