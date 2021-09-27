use std::mem;

use super::ast::{
  BinopKind, Block, Expr, ExprKind, FunDecl, Item, ItemKind, Local, Pkg, Stmt,
  StmtKind, Ty, TyKind, UnopKind,
};

use super::interface::Precedence;

use crate::front::tokenizer::token::{Token, TokenKind, TOKEN_EOF};
use crate::front::tokenizer::Tokenizer;

// parse a source into an AST
#[inline]
pub fn parse(src: &str) -> Result<Pkg, String> {
  let mut tokenizer = Tokenizer::new(src);
  let mut parser = Parser::new(&mut tokenizer);

  parser.parse_pkg_mod()
}

#[inline]
pub fn parse_stmts(src: &str) -> Result<Vec<Box<Stmt>>, String> {
  let mut tokenizer = Tokenizer::new(src);
  let mut parser = Parser::new(&mut tokenizer);

  parser.parse_stmts()
}

pub struct Parser<'a> {
  first: Token,
  current: Token,
  tokenizer: &'a mut Tokenizer<'a>,
}

impl<'a> Parser<'a> {
  #[inline]
  pub fn new(tokenizer: &'a mut Tokenizer<'a>) -> Self {
    Self {
      first: TOKEN_EOF,
      current: TOKEN_EOF,
      tokenizer: tokenizer,
    }
  }

  #[inline]
  fn err(msg: &str) -> ! {
    panic!("{}", msg)
  }

  #[inline]
  fn expect_first(&mut self, kind: &TokenKind) -> Result<(), String> {
    if self.first.is(kind.to_owned()) {
      self.next();
      return Ok(());
    }

    Err(format!(
      "token {:?} expected, but the current token is {:?}!",
      kind,
      self.first.kind()
    ))
  }

  #[inline]
  fn is_eof(&mut self) -> bool {
    self.current.is_eof()
  }

  #[inline]
  fn next(&mut self) {
    self.current = mem::replace(&mut self.first, self.tokenizer.next());
  }

  #[inline]
  fn precedence(&self) -> Precedence {
    Precedence::from(self.current.kind())
  }

  #[inline]
  fn should_precedence_has_priority(&self, kind: &Precedence) -> bool {
    *kind < Precedence::from(self.first.kind())
  }

  #[inline]
  fn parse_pkg_mod(&mut self) -> Result<Pkg, String> {
    let items = self.parse_mod()?;

    Ok(Pkg { items })
  }

  #[inline]
  fn parse_mod(&mut self) -> Result<Vec<Box<Item>>, String> {
    let mut items = vec![];

    self.next();
    self.next();

    while !self.is_eof() {
      match self.parse_item() {
        Ok(item) => items.push(item),
        Err(_) => {}
      }

      self.next();
    }

    Ok(items)
  }

  #[inline]
  fn parse_stmts(&mut self) -> Result<Vec<Box<Stmt>>, String> {
    let mut stmts = Vec::new();

    self.next();
    self.next();

    while !self.is_eof() {
      match self.current.kind() {
        TokenKind::CommentLine => continue,
        _ => match self.parse_stmt() {
          Ok(stmt) => stmts.push(stmt),
          Err(_) => {}
        },
      }

      self.next();
    }

    Ok(stmts)
  }

  #[inline]
  fn parse_item(&mut self) -> Result<Box<Item>, String> {
    let kind = match self.current.kind() {
      TokenKind::Imu => self.parse_local_item(),
      _ => self.parse_fun_decl_item(),
    }?;

    Ok(box Item { kind })
  }

  #[inline]
  fn parse_local_item(&mut self) -> Result<ItemKind, String> {
    // SYNTAX: imu <ident> : <ty> = <expr> | imu <ident> := <expr>
    // move to the next token
    self.next();
    // parse identifier
    let ident = self.parse_ident_expr()?;
    // move to the next
    self.next();
    // parse type
    let ty = self.parse_ty_expr()?;
    // skip '='
    self.next();
    // parse expr value
    let value = self.parse_expr_by_precedence(&Precedence::Lowest)?;
    // check if the next token is ';'
    self.expect_first(&TokenKind::Semicolon)?;
    // return local statement
    Ok(ItemKind::Imu(box Local {
      ident: box Expr::new(ident),
      immutable: true,
      ty,
      value: value,
    }))
  }

  #[inline]
  fn parse_fun_decl_item(&mut self) -> Result<ItemKind, String> {
    // SYNTAX: fun <ident> : <ty> = () {} | fun <ident> := {}
    // skip fun keyword
    self.next();
    // parse identifier
    let ident = self.parse_ident_expr()?;
    // advance token
    self.next();
    // parse type
    let ty = self.parse_ty_expr()?;
    // check if the next token is '('
    self.expect_first(&TokenKind::OpenParen)?;
    // parse args
    let args = self.parse_args()?;
    // check if the next token is '{'
    self.expect_first(&TokenKind::OpenBrace)?;
    // parse block
    let block = self.parse_block_stmt()?;
    // returns fun declaration
    Ok(ItemKind::Fun(box FunDecl {
      ident: box Expr::new(ident),
      ty,
      args,
      block,
    }))
  }

  #[inline]
  fn parse_args(&mut self) -> Result<Vec<Box<Expr>>, String> {
    let mut args = vec![];

    if self.first.is(TokenKind::CloseParen) {
      self.next();
      return Ok(args);
    }

    self.next();
    args.push(box Expr::new(self.parse_ident_expr()?));

    while self.first.is(TokenKind::Comma) {
      self.next();
      self.next();
      args.push(box Expr::new(self.parse_ident_expr()?));
    }

    self.expect_first(&TokenKind::CloseParen)?;

    Ok(args)
  }

  #[inline]
  fn parse_stmt(&mut self) -> Result<Box<Stmt>, String> {
    match self.current.kind() {
      TokenKind::Imu | TokenKind::Mut | TokenKind::Val => {
        self.parse_local_stmt()
      }
      TokenKind::Fun => self.parse_fun_decl_stmt(),
      TokenKind::Return => self.parse_return_stmt(),
      _ => self.parse_expr_stmt(),
    }
  }

  #[inline]
  fn parse_assign_expr(&mut self, lhs: ExprKind) -> Result<ExprKind, String> {
    // SYNTAX: <ident> = <expr>
    // move to the next token by skipping '='
    self.next();
    // parse expression
    let value = self.parse_expr()?;
    // check if the next token is ';'
    self.expect_first(&TokenKind::Semicolon)?;
    // return assign statement
    Ok(ExprKind::Assign {
      lhs: box Expr::new(lhs),
      rhs: box Expr::new(value),
    })
  }

  #[inline]
  fn parse_block_stmt(&mut self) -> Result<Box<Block>, String> {
    // SYNTAX: { <stmt>* }
    // create an array of statements
    let mut stmts = vec![];
    // skip '{'
    self.next();
    // until the next token is '}' or EOF
    while *self.current.kind() != TokenKind::CloseBrace
      && *self.current.kind() != TokenKind::EOF
    {
      // store the statement
      stmts.push(self.parse_stmt()?);
      // then move to the next token
      self.next();
    }
    // return block statement
    Ok(box Block { stmts })
  }

  #[inline]
  fn parse_fun_decl_stmt(&mut self) -> Result<Box<Stmt>, String> {
    // SYNTAX: fun <ident> : <ty> = () {} | fun <ident> := {}
    // skip fun keyword
    self.next();
    // parse identifier
    let ident = self.parse_ident_expr()?;
    // advance token
    self.next();
    // parse type
    let ty = self.parse_ty_expr()?;
    // check if the next token is '('
    self.expect_first(&TokenKind::OpenParen)?;
    // parse args
    let args = self.parse_args()?;
    // check if the next token is '{'
    self.expect_first(&TokenKind::OpenBrace)?;
    // parse block
    let block = self.parse_block_stmt()?;
    // returns fun declaration
    Ok(box Stmt::new(StmtKind::Fun(box FunDecl {
      ident: box Expr::new(ident),
      ty,
      args,
      block,
    })))
  }

  #[inline]
  fn parse_return_stmt(&mut self) -> Result<Box<Stmt>, String> {
    // SYNTAX: return <expr> ;
    // move to the next token
    self.next();
    // if the next token is ';'
    if self.current.is(TokenKind::Semicolon) {
      // return empty return statement
      return Ok(box Stmt::new(StmtKind::Return(None)));
    }
    // parse expr by precedence
    let expr = self.parse_expr_by_precedence(&Precedence::Lowest)?;
    // until we don't reach ';' or newline
    while *self.current.kind() != TokenKind::Semicolon
      && *self.current.kind() != TokenKind::Newline
    {
      // keep going
      self.next();
    }
    // return return statement
    Ok(box Stmt::new(StmtKind::Return(Some(expr))))
  }

  #[inline]
  fn parse_local_stmt(&mut self) -> Result<Box<Stmt>, String> {
    // SYNTAX: val x : uint = 10; | mut y : uint = 0;
    // store the keyword
    let kw = self.current.to_owned();
    // move to the next token
    self.next();
    // parse identifier
    let ident = self.parse_ident_expr()?;
    // move to the next
    self.next();
    // parse type
    let ty = self.parse_ty_expr()?;
    // skip '='
    self.next();
    // parse expr value
    let value = self.parse_expr_by_precedence(&Precedence::Lowest)?;
    // check if the next token is ';'
    self.expect_first(&TokenKind::Semicolon)?;
    // return local statement
    match kw.kind() {
      TokenKind::Imu => Ok(box Stmt::new(StmtKind::Imu(box Local {
        ident: box Expr::new(ident),
        immutable: true,
        ty,
        value: value,
      }))),
      TokenKind::Val => Ok(box Stmt::new(StmtKind::Val(box Local {
        ident: box Expr::new(ident),
        immutable: true,
        ty,
        value: value,
      }))),
      _ => Ok(box Stmt::new(StmtKind::Mut(box Local {
        ident: box Expr::new(ident),
        immutable: false,
        ty,
        value: value,
      }))),
    }
  }

  #[inline]
  fn parse_expr_stmt(&mut self) -> Result<Box<Stmt>, String> {
    let expr = self.parse_expr_by_precedence(&Precedence::Lowest)?;

    if self.first.is(TokenKind::Semicolon) {
      self.next();
    }

    Ok(box Stmt::new(StmtKind::Expr(expr)))
  }

  fn parse_expr_by_precedence(
    &mut self,
    precedence: &Precedence,
  ) -> Result<Box<Expr>, String> {
    let mut lhs = self.parse_expr()?;

    while !self.first.is(TokenKind::Semicolon)
      && self.should_precedence_has_priority(precedence)
    {
      self.next();

      lhs = self.parse_binop_expr_by_lhs(lhs)?;
    }

    Ok(box Expr::new(lhs))
  }

  #[inline]
  fn parse_binop_expr_by_lhs(
    &mut self,
    lhs: ExprKind,
  ) -> Result<ExprKind, String> {
    match self.current.kind() {
      TokenKind::Assign => self.parse_assign_expr(lhs),
      TokenKind::OpenBracket => self.parse_index_expr(lhs),
      TokenKind::OpenParen => self.parse_call_expr(lhs),
      _ => self.parse_binop_expr(lhs),
    }
  }

  #[inline]
  fn parse_binop_expr(&mut self, lhs: ExprKind) -> Result<ExprKind, String> {
    // SYNTAX: <expr> <binop> <expr>
    // store the precedence from the current token kind
    let precedence = self.precedence();
    // store the operator from the current token kind
    let op = BinopKind::from(&self.current.kind());
    // move to the next token
    self.next();
    // parse the right hand side of the expression
    let rhs = self.parse_expr_by_precedence(&precedence)?;
    // return the binary expression
    Ok(ExprKind::Binop {
      lhs: box Expr::new(lhs),
      op,
      rhs,
    })
  }

  #[inline]
  fn parse_expr(&mut self) -> Result<ExprKind, String> {
    match self.current.kind() {
      TokenKind::True | TokenKind::False => self.parse_bool_expr(),
      TokenKind::Int(..) => self.parse_int_expr(),
      TokenKind::Float(..) => self.parse_float_expr(),
      TokenKind::CharAscii(..) => self.parse_char_expr(),
      TokenKind::StrBuffer(..) => self.parse_str_expr(),
      TokenKind::Identifier(..) => self.parse_ident_expr(),
      TokenKind::OpenBracket => self.parse_array_expr(),
      TokenKind::OpenParen => self.parse_group_expr(),
      TokenKind::If => self.parse_if_expr(),
      TokenKind::Sub | TokenKind::Not => self.parse_unop_expr(),
      TokenKind::Loop => self.parse_infinite_loop_expr(),
      TokenKind::While => self.parse_while_loop_expr(),
      _ => Err(format!("expr error: {:?}", &self.current)),
    }
  }

  #[inline]
  fn parse_call_expr(&mut self, lhs: ExprKind) -> Result<ExprKind, String> {
    // SYNTAX: <expr> ( <expr> , <expr> , ... )
    // parse args list
    let args = self.parse_until(&TokenKind::CloseParen)?;
    // return call expression
    Ok(ExprKind::Call {
      callee: box Expr::new(lhs),
      args,
    })
  }

  #[inline]
  fn parse_index_expr(&mut self, lhs: ExprKind) -> Result<ExprKind, String> {
    // SYNTAX: <expr> [ <expr> ]
    // move to the next token
    self.next();
    // parse index expr by precedence
    let rhs = self.parse_expr_by_precedence(&Precedence::Lowest)?;
    // check if the next token is ']'
    self.expect_first(&TokenKind::CloseBracket)?;
    // return index expression
    Ok(ExprKind::Index {
      lhs: box Expr::new(lhs),
      rhs,
    })
  }

  #[inline]
  fn parse_ident_expr(&mut self) -> Result<ExprKind, String> {
    match self.current.kind() {
      TokenKind::Identifier(ref ident) => Ok(ExprKind::Ident(ident.into())),
      _ => Err(format!(
        "unexpected error on ident parse with {:?}",
        &self.current
      )),
    }
  }

  #[inline]
  fn parse_int_expr(&mut self) -> Result<ExprKind, String> {
    match self.current.kind() {
      TokenKind::Int(ref num) => Ok(ExprKind::Int(*num as i32)),
      _ => Err(format!("parse int literal expression error.")),
    }
  }

  #[inline]
  fn parse_float_expr(&mut self) -> Result<ExprKind, String> {
    match self.current.kind() {
      TokenKind::Float(ref num) => Ok(ExprKind::Float(*num as f32)),
      _ => Err(format!("parse float literal expression error")),
    }
  }

  #[inline]
  fn parse_char_expr(&mut self) -> Result<ExprKind, String> {
    match self.current.kind() {
      TokenKind::CharAscii(ref ascii) => Ok(ExprKind::Char(*ascii)),
      _ => Err(format!("parse char literal expression error")),
    }
  }

  #[inline]
  fn parse_str_expr(&mut self) -> Result<ExprKind, String> {
    match self.current.kind() {
      TokenKind::StrBuffer(ref buf) => Ok(ExprKind::Str(buf.into())),
      _ => Err(format!("parse str literal expression error")),
    }
  }

  #[inline]
  fn parse_bool_expr(&mut self) -> Result<ExprKind, String> {
    match self.current.kind() {
      TokenKind::True => Ok(ExprKind::Bool(true)),
      TokenKind::False => Ok(ExprKind::Bool(false)),
      _ => Err(format!("parse bool literal expression error")),
    }
  }

  #[inline]
  fn parse_array_expr(&mut self) -> Result<ExprKind, String> {
    // SYNTAX: [expr, expr, expr, ...]
    // parse expression list
    let exprs = self.parse_until(&TokenKind::CloseBracket)?;
    // return array expression
    Ok(ExprKind::Array(exprs))
  }

  #[inline]
  fn parse_group_expr(&mut self) -> Result<ExprKind, String> {
    // SYNTAX: ( <expr> )
    // move to the next token
    self.next();
    // parse expression by precedence
    let expr = self.parse_expr_by_precedence(&Precedence::Lowest)?;
    // check if the next token is a close paren
    self.expect_first(&TokenKind::CloseParen)?;
    // return the expression
    Ok(expr.kind().to_owned())
  }

  #[inline]
  fn parse_if_expr(&mut self) -> Result<ExprKind, String> {
    // SYNTAX: if <expr> { <expr> } else { <expr> }
    // skip 'if'
    self.next();
    // parse condition
    let condition = self.parse_expr_by_precedence(&Precedence::Lowest)?;
    // check if the next token is '{'
    self.expect_first(&TokenKind::OpenBrace)?;
    // parse then branch
    let consequence = self.parse_block_stmt()?;
    // check if the next token is 'else'
    let alternative = if self.first.is(TokenKind::Else) {
      // skip 'else'
      self.next();
      // check if the next token is '{'
      self.expect_first(&TokenKind::OpenBrace)?;
      // parse else branch
      Some(self.parse_block_stmt()?)
    } else {
      // if not, return None
      None
    };
    // return the if expression
    Ok(ExprKind::If {
      condition: condition,
      consequence: consequence,
      alternative: alternative,
    })
  }

  #[inline]
  fn parse_infinite_loop_expr(&mut self) -> Result<ExprKind, String> {
    self.expect_first(&TokenKind::OpenBrace)?;

    let block = self.parse_block_stmt()?;

    Ok(ExprKind::Loop { block })
  }

  #[inline]
  fn parse_while_loop_expr(&mut self) -> Result<ExprKind, String> {
    // SYNTAX: while <expr> { <expr> }
    // skip 'while'
    self.next();
    // parse the condition expression
    let condition = self.parse_expr_by_precedence(&Precedence::Lowest)?;
    // check if the next token is '{'
    self.expect_first(&TokenKind::OpenBrace)?;
    // parse the block statement
    let block = self.parse_block_stmt()?;
    // return the while expression
    Ok(ExprKind::While { condition, block })
  }

  #[inline]
  fn parse_unop_expr(&mut self) -> Result<ExprKind, String> {
    // SYNTAX: - <expr> | ! <expr> | * <expr>
    // store the operand
    let op = UnopKind::from(self.current.kind());
    // move to the next token
    self.next();
    // parse right hand side expression by precedence
    let rhs = self.parse_expr_by_precedence(&Precedence::Unary)?;
    // return the unary expression
    Ok(ExprKind::Unop { op: op, rhs: rhs })
  }

  #[inline]
  fn parse_ty_expr(&mut self) -> Result<Box<Ty>, String> {
    if self.current.is(TokenKind::ColonAssign) {
      return Ok(box Ty::new(TyKind::Dynamic));
    }

    self.next();

    let ty = match self.current.kind() {
      TokenKind::OpenBracket => return self.parse_array_ty(),
      TokenKind::OpenParen => return self.parse_fun_ty(),
      _ => self.parse_ty()?,
    };

    self.next();

    Ok(ty)
  }

  #[inline]
  fn parse_array_ty(&mut self) -> Result<Box<Ty>, String> {
    // SYNTAX: [<ty>]
    // skip '['
    self.next();
    // parse the type
    let ty = self.parse_ty()?;
    // check if the next token is ']'
    self.expect_first(&TokenKind::CloseBracket)?;
    // move to the next token
    self.next();
    // return the array type
    Ok(box Ty::new(TyKind::Array(ty)))
  }

  #[inline]
  fn parse_fun_ty(&mut self) -> Result<Box<Ty>, String> {
    // parse types
    let tys = self.parse_ty_args_exprs()?;
    // check if the first token is a thin arrow token
    self.expect_first(&TokenKind::ThinArrow)?;
    // parse return type
    let ret_ty = self.parse_ty_expr()?;
    // returns a fn ty
    Ok(box Ty::new(TyKind::Fun(tys, ret_ty)))
  }

  #[inline]
  fn parse_ty_args_exprs(&mut self) -> Result<Vec<Box<Ty>>, String> {
    let mut tys = Vec::new();
    // no type args case
    if self.first.is(TokenKind::CloseParen) {
      self.next();
      return Ok(tys);
    }
    // skip open parenthesis
    self.next();
    // parse first type args
    tys.push(self.parse_ty()?);
    // parse rest type args
    while self.first.is(TokenKind::Comma) {
      self.next();
      self.next();
      tys.push(self.parse_ty()?);
    }
    // skip close parenthesis
    self.expect_first(&TokenKind::CloseParen)?;
    // return type args
    Ok(tys)
  }

  #[inline]
  fn parse_ty(&mut self) -> Result<Box<Ty>, String> {
    let kind = match self.current.kind() {
      TokenKind::U8 => TyKind::U8,
      TokenKind::U16 => TyKind::U16,
      TokenKind::U32 => TyKind::U32,
      TokenKind::U64 => TyKind::U64,
      TokenKind::UInt => TyKind::UInt,
      TokenKind::S8 => TyKind::S8,
      TokenKind::S16 => TyKind::S16,
      TokenKind::S32 => TyKind::S32,
      TokenKind::S64 => TyKind::S64,
      TokenKind::SInt => TyKind::SInt,
      TokenKind::Bool => TyKind::Bool,
      TokenKind::Char => TyKind::Char,
      TokenKind::Str => TyKind::Str,
      TokenKind::Void => TyKind::Void,
      _ => return Err(format!("parse ty expr error")),
    };

    Ok(box Ty::new(kind))
  }

  #[inline]
  fn parse_until(
    &mut self,
    kind: &TokenKind,
  ) -> Result<Vec<Box<Expr>>, String> {
    let mut exprs = vec![];

    if self.first.is(kind.to_owned()) {
      self.next();
      return Ok(exprs);
    }

    self.next();
    exprs.push(self.parse_expr_by_precedence(&Precedence::Lowest)?);

    while self.first.is(TokenKind::Comma) {
      self.next();
      self.next();
      exprs.push(self.parse_expr_by_precedence(&Precedence::Lowest)?);
    }

    self.expect_first(&kind)?;

    Ok(exprs)
  }
}
