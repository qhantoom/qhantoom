use crate::front::parser::parse;

use crate::front::parser::ast::{
  Item, ItemKind, Pkg, Expr, ExprKind, Local, Ty, TyKind,
};

#[test]
fn parse_imu_item() {
  let file = read_file("../../samples/ast/items/imu.qh");
  let ast = parse(&file).unwrap();

  let expected = vec![
    Pkg {
      items: vec![
        box Item {
          kind: ItemKind::Imu(
            box Local {
              ident: box Expr {
                kind: ExprKind::Ident(
                  "LIMIT".into(),
                ),
              },
              immutable: true,
              ty: box Ty {
                kind: TyKind::UInt,
              },
              value: box Expr {
                kind: ExprKind::Int(
                  4,
                ),
              },
            },
          ),
        },
      ],
    }
  ];
}

fn parse_fun_decl_item() {}

fn parse_imu_stmt() {}

fn parse_val_stmt() {}

fn parse_mut_stmt() {}

fn parse_fun_decl_stmt() {}

fn parse_bool_expr() {}

fn parse_int_expr() {}

fn parse_float_expr() {}

fn parse_char_expr() {}

fn parse_str_expr() {}

fn parse_ident_expr() {}

fn parse_array_expr() {}

fn parse_group_expr() {}

fn parse_if_expr() {}

fn parse_unop_expr() {}

fn parse_binop_expr() {}

fn parse_assign_expr() {}

fn parse_loop_expr() {}

fn parse_while_expr() {}

fn read_file(path: &str) -> String {
  match crate::util::reader::readfile(&path) {
    Ok(f) => f,
    Err(e) => panic!("{}", e),
  }
}

fn run_test( len: usize, items: Vec<Box<Item>>, expected: Vec<ItemKind>) {
  assert_eq!(items.len(), len);

  for (i, item) in items.iter().enumerate() {
    assert_eq!(*item.kind(), expected[i]);
  }
}
