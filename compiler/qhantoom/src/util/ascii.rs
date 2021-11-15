#[inline]
pub fn is_end_of_file(ascii: char) -> bool {
  ascii == '\u{0}'
}

#[inline]
pub fn is_newline(ascii: char) -> bool {
  ascii == '\u{000A}'
}

#[inline]
pub fn is_number(ascii: char) -> bool {
  ascii.is_digit(10)
}

#[inline]
pub fn is_number_start(ascii: char) -> bool {
  ascii == '0'
}

#[inline]
pub fn is_number_continue(ascii: char) -> bool {
  match ascii {
    '1'..='9' => true,
    _ => false,
  }
}

#[inline]
pub fn is_id(ascii: char) -> bool {
  ascii.is_alphabetic() || is_underscore(ascii)
}

#[inline]
pub fn is_id_start(ascii: char) -> bool {
  is_id(ascii)
}

#[inline]
pub fn is_id_continue(ascii: char) -> bool {
  is_id(ascii) || is_number(ascii)
}

#[inline]
pub fn is_underscore(ascii: char) -> bool {
  ascii == '\u{005F}'
}

#[inline]
pub fn is_whitespace(ascii: char) -> bool {
  match ascii {
    | '\u{0009}' // \t
    // | '\u{000A}' // \n
    | '\u{000B}' // vertical tab
    | '\u{000C}' // form feed
    | '\u{000D}' // \r
    | '\u{0020}' // space
    | '\u{0085}' // next line from latin1
    | '\u{200E}' // left-to-right mark
    | '\u{200F}' // right-to-left mark
    | '\u{2028}' // line separator
    | '\u{2029}' // paragraph separator
    => true,
    _ => false,
  }
}
