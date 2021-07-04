pub fn is_close_brace(ascii: char) -> bool {
  ascii == '\u{007D}'
}

pub fn is_colon(ascii: char) -> bool {
  ascii == '\u{003A}'
}

pub fn is_double_quote(ascii: char) -> bool {
  ascii == '\u{0022}'
}

pub fn is_equals(ascii: char) -> bool {
  ascii == '\u{003D}'
}

pub fn is_group(ascii: char) -> bool {
  match ascii {
    '{' | '}' => true,
    _ => false,
  }
}

pub fn is_tag(ascii: char) -> bool {
  match ascii {
    '<' | '>' => true,
    _ => false,
  }
}

pub fn is_id_continue(ascii: char) -> bool {
  is_ident(ascii) || is_number(ascii) || ascii > '\x7f'
}

pub fn is_id_start(ascii: char) -> bool {
  is_ident(ascii) || ascii > '\x7f'
}

pub fn is_ident(ascii: char) -> bool {
  ascii.is_alphabetic()
}

pub fn is_left_angle_bracket(ascii: char) -> bool {
  ascii == '\u{003C}'
}

pub fn is_number(ascii: char) -> bool {
  ascii.is_digit(10)
}

pub fn is_open_brace(ascii: char) -> bool {
  ascii == '\u{007B}'
}

pub fn is_quote(ascii: char) -> bool {
  is_double_quote(ascii)
}

pub fn is_right_angle_bracket(ascii: char) -> bool {
  ascii == '\u{003E}'
}

pub fn is_slash(ascii: char) -> bool {
  ascii == '\u{002F}'
}

pub fn is_symbol(ascii: char) -> bool {
  match ascii {
    ':' | '!' | '=' | '-' | '/' => true,
    _ => false,
  }
}

pub fn is_whitespace(ascii: char) -> bool {
  match ascii {
    | '\u{0009}' // \t
    | '\u{000A}' // \n
    | '\u{000B}' // vertical tab
    | '\u{000C}' // form feed
    | '\u{000D}' // \r
    | '\u{0020}' // space
    | '\u{0085}' // NEXT LINE from latin1
    | '\u{200E}' // LEFT-TO-RIGHT MARK
    | '\u{200F}' // RIGHT-TO-LEFT MARK
    | '\u{2028}' // LINE SEPARATOR
    | '\u{2029}' // PARAGRAPH SEPARATOR
    => true,
    _ => false,
  }
}
