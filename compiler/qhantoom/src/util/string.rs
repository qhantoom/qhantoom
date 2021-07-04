pub macro strbuf {
  () => ( String::new() ),
  ( $elmt:expr ) => ( String::from($elmt) ),
  ( $($x:expr,)* ) => ({
    let buf = String::new();

    $( write!(buf, "{}", $x) );*

    buf
  }),
  ( $s:expr, $($x:expr,)* ) => ({
    let buf = String::new();

    write!(buf, $s, $( $x )*);

    buf
  }),
  ( $s:expr, $($x:expr,)* ) => ( format!($s, $($x:expr,)*) ),
}
