// TODO: needs work

#[cfg(test)]
mod unit_tests {
  use crate::front::tokenizer::{
    tokenize_capsule_from_file, tokenize_capsule_from_source,
  };

  macro run_or_die($e:expr) {
    match $e {
      Ok(e) => e,
      Err(e) => panic!("{} - {}", stringify!($e), e),
    }
  }

  #[test]
  fn tokenize_empty() {
    let curpath = format!("../../samples/tokens/empty.qh");
    let path = std::path::Path::new(&curpath);
    let tokens_from_file = run_or_die!(tokenize_capsule_from_file(&path));

    let src = r#""#;
    let tokens_from_source = run_or_die!(tokenize_capsule_from_source(&src));

    assert_eq!(tokens_from_file.is_empty(), tokens_from_source.is_empty());
    assert_eq!(tokens_from_file.len(), tokens_from_source.len());
  }

  #[test]
  fn tokenize_operators() {
    let filename = format!("../../samples/tokens/operators.qh");
    let pathname = std::path::Path::new(&filename);
    let tokens_from_file = run_or_die!(tokenize_capsule_from_file(&pathname));

    let src = r#"
      +   -   *   /   %   !   .   :   &   <   >   =   |
      ++  --  **  //  %%  !!  ..  ::  &&  <<  >>  ==  ||
      +=  -=  *=  /=  %=  !=  .=  :=  &=  <=  >=
      ?   \   ->  =>  []  ()  {}
    "#;

    let tokens_from_source = run_or_die!(tokenize_capsule_from_source(&src));

    assert_eq!(!tokens_from_file.is_empty(), !tokens_from_source.is_empty(),);

    assert_eq!(tokens_from_file.len(), tokens_from_source.len(),);
  }

  #[test]
  fn tokenize_ints() {
    let filename = format!("../../samples/tokens/ints.qh");
    let pathname = std::path::Path::new(&filename);
    let tokens_from_file = run_or_die!(tokenize_capsule_from_file(&pathname));

    let src = r#"1 123 123_000 1e4"#;
    let tokens_from_source = run_or_die!(tokenize_capsule_from_source(&src));

    assert_eq!(!tokens_from_file.is_empty(), !tokens_from_source.is_empty(),);

    assert_eq!(tokens_from_file.len(), tokens_from_source.len(),);
  }

  #[test]
  fn tokenize_floats() {
    let filename = format!("../../samples/tokens/floats.qh");
    let pathname = std::path::Path::new(&filename);
    let tokens_from_file = run_or_die!(tokenize_capsule_from_file(&pathname));

    let src = r#"1.0 123.456 123_456"#;
    let tokens_from_source = run_or_die!(tokenize_capsule_from_source(&src));

    assert_eq!(!tokens_from_file.is_empty(), !tokens_from_source.is_empty(),);

    assert_eq!(tokens_from_file.len(), tokens_from_source.len(),);
  }

  #[test]
  fn tokenize_idents() {
    let filename = format!("../../samples/tokens/idents.qh");
    let pathname = std::path::Path::new(&filename);
    let tokens_from_file = run_or_die!(tokenize_capsule_from_file(&pathname));

    let src = r#"
      square    cosinus   degrees
      _tmp      add_tmp   to_tmp_
      vector1   vector2   vector3
    "#;

    let tokens_from_source = run_or_die!(tokenize_capsule_from_source(&src));

    assert_eq!(!tokens_from_file.is_empty(), !tokens_from_source.is_empty(),);

    assert_eq!(tokens_from_file.len(), tokens_from_source.len(),);
  }

  #[test]
  fn tokenize_keywords() {
    let filename = format!("../../samples/tokens/keywords.qh");
    let pathname = std::path::Path::new(&filename);
    let tokens_from_file = run_or_die!(tokenize_capsule_from_file(&pathname));

    let src = r#"
      as      async     await   bench   bind      bool
      break   capsule   chan    char    continue  else
      enum    exp       ext     f32     f64       false
      fun     Fun       for     if      load      loop
      match   me        Me      mock    mod       mut
      pack    pub       ref     return  s8        s16
      s32     s64       set     sint    spawn     str
      struct  test      true    type    u8        u16
      u32     u64       uint    unit    val       imu
      wasm    where     while   _
    "#;

    let tokens_from_source = run_or_die!(tokenize_capsule_from_source(&src));

    assert_eq!(!tokens_from_file.is_empty(), !tokens_from_source.is_empty(),);

    assert_eq!(tokens_from_file.len(), tokens_from_source.len(),);
  }

  // #[test]
  fn tokenize_comment_line() {
    let filename = format!("../../samples/tokens/comment_line.qh");
    let pathname = std::path::Path::new(&filename);
    let tokens_from_file = run_or_die!(tokenize_capsule_from_file(&pathname));

    let src = r#"# this is a line comment"#;
    let tokens_from_source = run_or_die!(tokenize_capsule_from_source(&src));

    assert_eq!(!tokens_from_file.is_empty(), !tokens_from_source.is_empty(),);

    assert_eq!(tokens_from_file.len(), tokens_from_source.len(),);
  }

  // #[test]
  fn tokenize_strings() {
    let filename = format!("../../samples/tokens/strings.qh");
    let pathname = std::path::Path::new(&filename);
    let tokens_from_file = run_or_die!(tokenize_capsule_from_file(&pathname));

    let src = r#""hello, world! 👽""#;
    let tokens_from_source = run_or_die!(tokenize_capsule_from_source(&src));

    assert_eq!(!tokens_from_file.is_empty(), !tokens_from_source.is_empty(),);

    assert_eq!(tokens_from_file.len(), tokens_from_source.len(),);
  }

  // #[test]
  fn tokenize_char_ascii() {
    let filename = format!("../../samples/tokens/char_ascii.qh");
    let pathname = std::path::Path::new(&filename);
    let tokens_from_file = run_or_die!(tokenize_capsule_from_file(&pathname));

    let src = r#"'a' 'b' 'c' 'd'"#;
    let tokens_from_source = run_or_die!(tokenize_capsule_from_source(&src));

    assert_eq!(!tokens_from_file.is_empty(), !tokens_from_source.is_empty(),);

    assert_eq!(tokens_from_file.len(), tokens_from_source.len(),);
  }
}
