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
pub fn is_hex_start(ascii: char) -> bool {
  match ascii {
    'x' | 'X' => true,
    _ => false,
  }
}

#[inline]
pub fn is_hex_continue(ascii: char) -> bool {
  match ascii {
    '0'..='9' | 'a'..='f' | 'A'..='F' => true,
    _ => false,
  }
}

#[inline]
pub fn is_oct_start(ascii: char) -> bool {
  match ascii {
    'o' | 'O' => true,
    _ => false,
  }
}

#[inline]
pub fn is_oct_continue(ascii: char) -> bool {
  match ascii {
    '1'..='7' => true,
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
pub fn is_quote(ascii: char) -> bool {
  is_double_quote(ascii) || is_single_quote(ascii)
}

#[inline]
pub fn is_double_quote(ascii: char) -> bool {
  ascii == '\u{0022}'
}

#[inline]
pub fn is_single_quote(ascii: char) -> bool {
  ascii == '\u{0027}'
}

#[inline]
pub fn is_underscore(ascii: char) -> bool {
  ascii == '\u{005F}'
}

pub fn is_comment_start(ascii: char) -> bool {
  ascii == '\u{0023}'
}

#[inline]
pub fn is_whitespace(ascii: char) -> bool {
  match ascii {
    | '\u{0009}' // \t
    | '\u{000A}' // \n
    | '\u{000B}' // vertical tab
    | '\u{000C}' // form feed
    | '\u{000D}' // \r
    | '\u{0020}' // space
    | '\u{0085}' // next line from latin1
    | '\u{200E}' // left-to-right mark
    | '\u{200F}' // right-to-left mark
    | '\u{2028}' // line seprarator
    | '\u{2029}' // paragraph seprarator
    => true,
    _ => false,
  }
}
