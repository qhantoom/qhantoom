use std::fmt;

use super::ast::{
  Arg, BinopKind, Block, Expr, ExprKind, Field, FieldExpr, Fun, Program,
  Prototype, Stmt, StmtKind, Struct, StructExpr, UnopKind,
};

use super::ty::{Ty, TyKind};

pub struct Sep<'a, T: 'a>(pub &'a [T], pub &'a str);

impl<'a, T: fmt::Display> fmt::Display for Sep<'a, T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let exprs = self
      .0
      .iter()
      .map(|a| a.to_string())
      .collect::<Vec<String>>()
      .join(self.1);

    write!(f, "{}", exprs)
  }
}

impl fmt::Display for Program {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", Sep(&self.stmts, "\n"))
  }
}

impl fmt::Display for Fun {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "fun {} {}", self.prototype, self.body)
  }
}

impl fmt::Display for Prototype {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} ({}) : {}", self.name, Sep(&self.args, ", "), self.ty,)
  }
}

impl fmt::Display for Arg {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} : {}", self.name, self.ty)
  }
}

impl fmt::Display for Block {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{{ {} }}", Sep(&self.stmts, "\n"))
  }
}

impl fmt::Display for Struct {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "struct {} {}", self.name, Sep(&self.fields, "\n"))
  }
}

impl fmt::Display for Field {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} : {}", self.name, self.ty)
  }
}

impl fmt::Display for Stmt {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.kind() {
      StmtKind::Ext(ref prototype) => {
        write!(f, "ext {};", prototype)
      }
      StmtKind::Fun(ref fun) => write!(f, "{}", fun),
      StmtKind::Val(ref local) => {
        if *local.ty.kind() == TyKind::Dynamic {
          return write!(f, "val {} := {};", local.name, local.value);
        }

        write!(f, "val {} : {} = {};", local.name, local.ty, local.value)
      }
      StmtKind::Mut(ref local) => {
        if *local.ty.kind() == TyKind::Dynamic {
          return write!(f, "mut {} := {};", local.name, local.value);
        }

        write!(f, "mut {} : {} = {};", local.name, local.ty, local.value)
      }
      StmtKind::Return(ref expr) => {
        if let Some(ref expr) = expr {
          write!(f, "return {};", expr)
        } else {
          write!(f, "return;")
        }
      }
      StmtKind::Break(ref expr) => {
        if let Some(ref expr) = *expr {
          write!(f, "break {};", expr)
        } else {
          write!(f, "break;")
        }
      }
      StmtKind::Continue(ref expr) => {
        if let Some(ref expr) = *expr {
          write!(f, "continue {};", expr)
        } else {
          write!(f, "continue;")
        }
      }
      StmtKind::Expr(ref expr) => write!(f, "{}", expr),
      StmtKind::Struct(ref def) => write!(f, "{}", def),
    }
  }
}

impl fmt::Display for StructExpr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} {{ {} }}", self.name, Sep(&self.fields, ",\n"))
  }
}

impl fmt::Display for FieldExpr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} : {}", self.name, self.value)
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
      ExprKind::Array(ref data) => write!(f, "[{}]", Sep(data, ", ")),
      ExprKind::Index(ref lhs, ref rhs) => write!(f, "({}[{}])", lhs, rhs),
      ExprKind::Assign(ref lhs, ref rhs) => write!(f, "{} = {};", lhs, rhs),
      ExprKind::AssignOp(ref op, ref lhs, ref rhs) => {
        write!(f, "{} {} {}", lhs, op, rhs)
      }
      ExprKind::Unop(ref op, ref rhs) => write!(f, "({}{})", op, rhs),
      ExprKind::Binop(ref lhs, ref op, ref rhs) => {
        write!(f, "({} {} {})", lhs, op, rhs)
      }
      ExprKind::Closure(ref fun) => {
        write!(f, "({}) -> {}", Sep(&fun.prototype.args, ", "), fun.body)
      }
      ExprKind::Call(ref callee, ref args) => {
        write!(f, "{}({})", callee, Sep(args, ", "))
      }
      ExprKind::If(ref condition, ref consequence, alternative) => {
        if let Some(alt) = alternative {
          write!(f, "if {} {} else {}", condition, consequence, alt)
        } else {
          write!(f, "if {} {}", condition, consequence)
        }
      }
      ExprKind::Loop(ref body) => write!(f, "loop {}", body),
      ExprKind::While(ref condition, ref body) => {
        write!(f, "while {} {}", condition, body)
      }
      ExprKind::For(ref iterable, ref iterator, ref body) => {
        write!(f, "for {} = ({}) {}", iterable, iterator, body)
      }
      ExprKind::Range(ref start, ref end, ref body) => {
        write!(f, "for {}..{} = (..) {}", start, end, body)
      }
      ExprKind::StructExpr(ref struct_expr) => write!(f, "{}", struct_expr),
      ExprKind::FieldAccess(ref lhs, ref name) => {
        write!(f, "{}.{}", lhs, name)
      }
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
      Self::Rem => write!(f, "%"),
      Self::And => write!(f, "&&"),
      Self::Or => write!(f, "||"),
      Self::Lt => write!(f, "<"),
      Self::Gt => write!(f, ">"),
      Self::Le => write!(f, "<="),
      Self::Ge => write!(f, ">="),
      Self::Eq => write!(f, "=="),
      Self::Ne => write!(f, "!="),
      Self::AddAssign => write!(f, "+="),
      Self::SubAssign => write!(f, "-="),
      Self::MulAssign => write!(f, "*="),
      Self::DivAssign => write!(f, "/="),
      Self::RemAssign => write!(f, "%="),
      Self::BitXorAssign => write!(f, "^="),
      Self::BitOrAssign => write!(f, "|="),
      Self::BitAndAssign => write!(f, "&="),
      Self::BitAnd => write!(f, "&"),
      Self::BitXor => write!(f, "^"),
      Self::BitOr => write!(f, "|"),
      Self::Shl => write!(f, "<<"),
      Self::Shr => write!(f, ">>"),
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
