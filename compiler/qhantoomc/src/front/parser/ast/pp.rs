use std::fmt;

use super::ast::{
  BinopKind, Block, Expr, ExprKind, Fun, Prototype, Stmt, StmtKind, Ty, TyKind,
  UnopKind,
};

pub struct CommaSep<'a, T: 'a>(pub &'a [T]);

impl<'a, T: fmt::Display> fmt::Display for CommaSep<'a, T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let exprs = self
      .0
      .iter()
      .map(|a| a.to_string())
      .collect::<Vec<String>>()
      .join(", ");

    write!(f, "{}", exprs)
  }
}

impl fmt::Display for Fun {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "fun {} {}", self.prototype, self.body)
  }
}

impl fmt::Display for Prototype {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{} : {} = ({})",
      self.name,
      self.ty,
      CommaSep(&self.args),
    )
  }
}

impl fmt::Display for Block {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{{ {} }}", CommaSep(&self.stmts))
  }
}

impl fmt::Display for Stmt {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.kind() {
      StmtKind::Expr(ref expr) => write!(f, "{:?}", expr),
      _ => write!(f, "{:?}", self.kind()),
    }
  }
}

impl fmt::Display for Expr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.kind() {
      ExprKind::Bool(ref boolean) => write!(f, "{}", boolean),
      ExprKind::Int(ref num) => write!(f, "{}", num),
      ExprKind::Float(ref num) => write!(f, "{}", num),
      ExprKind::Char(ref ch) => write!(f, "{}", ch),
      ExprKind::Str(ref string) => write!(f, "\"{}\"", string),
      ExprKind::Ident(ref ident) => write!(f, "{}", ident),
      ExprKind::Array(ref data) => write!(f, "[{}]", CommaSep(data)),
      ExprKind::Index { ref lhs, ref rhs } => write!(f, "({}[{}])", lhs, rhs),
      ExprKind::Assign { ref lhs, ref rhs } => write!(f, "{} = {};", lhs, rhs),
      ExprKind::Unop { ref op, ref rhs } => write!(f, "({}{})", op, rhs),
      ExprKind::Binop {
        ref lhs,
        ref op,
        ref rhs,
      } => write!(f, "({} {} {})", lhs, op, rhs),
      ExprKind::Call {
        ref callee,
        ref args,
      } => write!(f, "{}({})", callee, CommaSep(args)),
      ExprKind::If {
        ref condition,
        ref consequence,
        alternative: None,
      } => write!(f, "if {} {}", condition, consequence),
      ExprKind::If {
        ref condition,
        ref consequence,
        alternative: Some(ref alternative),
      } => write!(f, "if {} {} else {}", condition, consequence, alternative),
      ExprKind::While {
        ref condition,
        ref block,
      } => write!(f, "while {} {}", condition, block),
      ExprKind::Loop { ref block } => write!(f, "loop {}", block),
    }
  }
}

impl fmt::Display for Ty {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.kind() {
      TyKind::S8 => write!(f, "s8"),
      TyKind::S16 => write!(f, "s16"),
      TyKind::S32 => write!(f, "s32"),
      TyKind::S64 => write!(f, "s64"),
      TyKind::SInt => write!(f, "sint"),
      TyKind::U8 => write!(f, "u8"),
      TyKind::U16 => write!(f, "u16"),
      TyKind::U32 => write!(f, "u32"),
      TyKind::U64 => write!(f, "u64"),
      TyKind::UInt => write!(f, "uint"),
      TyKind::F32 => write!(f, "f32"),
      TyKind::F64 => write!(f, "f64"),
      TyKind::Bool => write!(f, "bool"),
      TyKind::Str => write!(f, "str"),
      TyKind::Char => write!(f, "char"),
      TyKind::Void => write!(f, "void"),
      TyKind::Dynamic => write!(f, "dynamic"),
      TyKind::Array(ref ty) => write!(f, "[{}]", ty),
      TyKind::Fun(ref tys, ref ret_ty) => {
        write!(f, "({}) -> {}", CommaSep(tys), ret_ty,)
      }
    }
  }
}

impl fmt::Display for BinopKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Self::Add => write!(f, "+"),
      Self::Sub => write!(f, "-"),
      Self::Mul => write!(f, "*"),
      Self::Div => write!(f, "/"),
      Self::Mod => write!(f, "%"),
      Self::And => write!(f, "&&"),
      Self::Or => write!(f, "||"),
      Self::Lt => write!(f, "<"),
      Self::Gt => write!(f, ">"),
      Self::Le => write!(f, "<="),
      Self::Ge => write!(f, ">="),
      Self::Eq => write!(f, "=="),
      Self::Ne => write!(f, "!="),
    }
  }
}

impl fmt::Display for UnopKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Self::Neg => write!(f, "-"),
      Self::Not => write!(f, "!"),
    }
  }
}
