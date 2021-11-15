use super::ast::{self, Stmt, StmtKind};
use super::parser::Parser;

use crate::front::tokenizer::Tokenizer;
use crate::util::symbol::{Symbol, Symbols};

#[test]
fn parse_empty() {
  let file = read_file("../../samples/tests/ast/empty.qh");
  let tokens = parse(&file);
  let expected = vec![];

  run_test(0, tokens, expected);
}

#[test]
fn parse_comments() {
  let file = read_file("../../samples/tests/ast/comment.qh");
  let tokens = parse(&file);
  let expected = vec![];

  run_test(0, tokens, expected);
}

#[test]
fn parse_stmt_vals() {
  let file = read_file("../../samples/tests/ast/var.qh");
  let tokens = parse(&file);

  let expected = vec![
    StmtKind::Val(box ast::mk_local(
      box ast::mk_expr(ast::mk_ident(Symbol(0))),
      true,
      box ast::mk_ty(ast::TyKind::UInt),
      box ast::mk_expr(ast::ExprKind::Int(0)),
    )),
    StmtKind::Val(box ast::mk_local(
      box ast::mk_expr(ast::mk_ident(Symbol(1))),
      true,
      box ast::mk_ty(ast::TyKind::Dynamic),
      box ast::mk_expr(ast::ExprKind::Int(1)),
    )),
    StmtKind::Mut(box ast::mk_local(
      box ast::mk_expr(ast::mk_ident(Symbol(2))),
      false,
      box ast::mk_ty(ast::TyKind::UInt),
      box ast::mk_expr(ast::ExprKind::Int(0)),
    )),
    StmtKind::Mut(box ast::mk_local(
      box ast::mk_expr(ast::mk_ident(Symbol(3))),
      false,
      box ast::mk_ty(ast::TyKind::Dynamic),
      box ast::mk_expr(ast::ExprKind::Int(1)),
    )),
  ];

  run_test(4, tokens, expected);
}

fn read_file(path: &str) -> String {
  match crate::util::reader::read_file(&path) {
    Ok(f) => f,
    Err(e) => panic!("{}", e),
  }
}

fn run_test(len: usize, stmts: Vec<Stmt>, expected: Vec<StmtKind>) {
  assert_eq!(stmts.len(), len);

  for (i, stmts) in stmts.iter().enumerate() {
    assert_eq!(*stmts.kind(), expected[i]);
  }
}

fn parse(file: &str) -> Vec<Stmt> {
  let mut syms = Symbols::new();
  let mut tokenizer = Tokenizer::new(file, &mut syms);
  let tokens = tokenizer.tokenize();
  let mut parser = Parser::new(tokens);
  let ast = parser.parse();

  syms.store();

  ast.stmts
}
