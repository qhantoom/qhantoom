pub macro pathto {
  ( $e:expr ) => ({
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let p = p.join($e);

    p
  }),
  () => ( path![""] ),
}
