#[derive(Debug)]
pub enum TokenizerState {
  StartState,
  ZeroState,
  NumberState,
  BinState,
  HexState,
  DecState,
  OctState,
  ExpState,
  IdentState,
  EqState,
  AddState,
  SubState,
  DivState,
  MulState,
  ModState,
  DotState,
  ColonState,
  LtState,
  GtState,
  AndState,
  PipeState,
  BangState,
  QuoteState,
  CharState,
  StringState,
  CommentState,
}
