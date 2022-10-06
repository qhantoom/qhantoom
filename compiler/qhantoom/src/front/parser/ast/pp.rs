use super::ast::*;
use super::ty::{Ty, TyKind};

use std::fmt::{Display, Formatter, Result};

pub struct Sep<'a, T: 'a>(pub &'a [T], pub &'a str);

impl<'a, T: Display> Display for Sep<'a, T> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let nodes = self
      .0
      .iter()
      .map(|node| node.to_string())
      .collect::<Vec<String>>()
      .join(self.1);

    write!(f, "{}", nodes)
  }
}

impl Display for Public {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match self {
      Self::Yes(_) => write!(f, "pub"),
      Self::No => write!(f, ""),
    }
  }
}

impl Display for Async {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match self {
      Self::Yes(_) => write!(f, "async"),
      Self::No => write!(f, ""),
    }
  }
}

impl Display for Mutability {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match self {
      Self::Yes => write!(f, "mut"),
      Self::Not => write!(f, ""),
    }
  }
}

impl Display for BindingAnnotation {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.0)
  }
}

impl Display for Program {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", Sep(&self.items, "\n"))
  }
}

impl Display for Load {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "load {}", self.path_view)
  }
}

impl Display for PathView {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "load {}", self.kind)
  }
}

impl Display for PathViewKind {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match self {
      Self::Identifier(name) => write!(f, "{name}"),
      Self::Path(path, names) => write!(f, "{path}::({})", Sep(names, ", ")),
    }
  }
}

impl Display for Pattern {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", self.kind)
  }
}

impl Display for PatternKind {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Self::Underscore => write!(f, "_"),
      Self::Identifier(_, name) => write!(f, "{name}"),
      Self::Lit(lit) => write!(f, "{lit}"),
    }
  }
}

impl Display for Item {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.kind)
  }
}

impl Display for ItemKind {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match self {
      Self::Val(decl) => write!(f, "{decl}"),
      _ => todo!(),
    }
  }
}

impl Display for Ext {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let _ = match &self.public {
      Public::No => write!(f, ""),
      Public::Yes(_) => write!(f, "pub "),
    };

    write!(f, "ext {}", self.prototype).ok();

    let Some(body) = &self.body else { return write!(f, ""); };

    write!(f, "{}", body)
  }
}

impl Display for Decl {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(
      f,
      "{} {}: {} = {}",
      self.kind, self.pattern, self.ty, self.value
    )
  }
}

impl Display for DeclKind {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match self {
      Self::Val => write!(f, "val"),
      Self::Imu => write!(f, "imu"),
      Self::Mut => write!(f, "mut"),
    }
  }
}

impl Display for Fun {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "fun {} {}", self.prototype, self.body)
  }
}

impl Display for Prototype {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(
      f,
      "{} ({}) {}",
      self.name,
      Sep(&self.inputs, ", "),
      self.output
    )
  }
}

impl Display for Arg {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}: {}", self.pattern, self.ty)
  }
}

impl Display for ReturnTy {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match self {
      Self::Ty(ty) => write!(f, ": {ty}"),
      Self::Default(_) => write!(f, ""),
    }
  }
}

impl Display for Block {
  fn fmt(&self, f: &mut Formatter) -> Result {
    if self.stmts.is_empty() {
      write!(f, "{{}}")
    } else {
      write!(f, "{{\n{}\n}}", Sep(&self.stmts, "\n"))
    }
  }
}

impl Display for Stmt {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.kind)
  }
}

impl Display for StmtKind {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match self {
      Self::Item(item) => write!(f, "{item}"),
      Self::Decl(decl) => write!(f, "{decl}"),
      Self::Expr(expr) => write!(f, "{expr}"),
    }
  }
}

impl Display for Expr {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.kind)
  }
}

impl Display for ExprKind {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match self {
      Self::Lit(lit) => write!(f, "{lit}"),
      Self::Identifier(identifier) => write!(f, "{identifier}"),
      Self::Call(callee, args) => write!(f, "{callee}({})", Sep(args, ", ")),
      Self::UnOp(op, rhs) => write!(f, "{}({})", op.node, rhs),
      Self::BinOp(lhs, op, rhs) => write!(f, "({lhs} {op} {rhs})"),
      Self::Assign(lhs, op, rhs) => write!(f, "{lhs} {op} {rhs}"),
      Self::AssignOp(lhs, op, rhs) => write!(f, "{lhs} {op} {rhs}"),
      Self::Return(value) => {
        let Some(value) = value else { return write!(f, ""); };

        write!(f, "{value}")
      }
      Self::Block(block) => write!(f, "{block}"),
      Self::Loop(body) => write!(f, "for {body}"),
      Self::While(condition, body) => write!(f, "while {condition} {body}"),
    }
  }
}

impl Display for Lit {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", self.kind)
  }
}

impl Display for LitKind {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Self::Bool(boolean) => write!(f, "{boolean}"),
      Self::Int(num) => write!(f, "{num}"),
      Self::Float(num) => write!(f, "{num}"),
      Self::Str(string) => write!(f, "{string}"),
    }
  }
}

impl Display for BinOpKind {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match self {
      Self::Add => write!(f, "+"),
      Self::Sub => write!(f, "-"),
      Self::Mul => write!(f, "*"),
      Self::Div => write!(f, "/"),
      Self::Rem => write!(f, "%"),
      Self::And => write!(f, "&&"),
      Self::Or => write!(f, "||"),
      Self::Lt => write!(f, "<"),
      Self::Gt => write!(f, ">"),
      Self::Le => write!(f, "<="),
      Self::Ge => write!(f, ">="),
      Self::Eq => write!(f, "=="),
      Self::Ne => write!(f, "!="),
      Self::Shl => write!(f, "<<"),
      Self::Shr => write!(f, ">>"),
      Self::BitAnd => write!(f, "&"),
      Self::BitOr => write!(f, "|"),
      Self::BitXor => write!(f, "^"),
      Self::As => write!(f, "as"),
      Self::Range => write!(f, ".."),
    }
  }
}

impl Display for UnOpKind {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match self {
      Self::Neg => write!(f, "-"),
      Self::Not => write!(f, "!"),
    }
  }
}

impl Display for Ty {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", self.kind)
  }
}

impl Display for TyKind {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Self::Void => write!(f, "void"),
      Self::Bool => write!(f, "bool"),
      Self::U8 => write!(f, "u8"),
      Self::U16 => write!(f, "u16"),
      Self::U32 => write!(f, "u32"),
      Self::U64 => write!(f, "u64"),
      Self::UInt => write!(f, "uint"),
      Self::S8 => write!(f, "s8"),
      Self::S16 => write!(f, "s16"),
      Self::S32 => write!(f, "s32"),
      Self::S64 => write!(f, "s64"),
      Self::SInt => write!(f, "sint"),
      Self::F32 => write!(f, "f32"),
      Self::F64 => write!(f, "f64"),
      Self::Str => write!(f, "str"),
      Self::Fn(args, ty) => write!(f, "Fn({}): {ty}", Sep(args, ", ")),
    }
  }
}
