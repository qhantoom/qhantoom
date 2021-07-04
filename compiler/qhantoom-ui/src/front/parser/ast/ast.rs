pub use self::AttrKind::*;
pub use self::ElmtKind::*;
pub use self::PropKind::*;
pub use self::TagKind::*;

use crate::html::computer::interpreter::Interpreter;
use crate::html::transformer::transpiler::Transpiler;
use crate::CompilerResult;

use std::fmt;

use regex::{Captures, Regex};
use rsass::compile_scss;

#[derive(Clone, Debug, PartialEq)]
pub enum ElmtKind {
  Tag(TagKind),
  Txt(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Elmt {
  pub kind: Box<ElmtKind>,
  pub attrs: Vec<Box<AttrKind>>,
  pub children: Vec<Box<Elmt>>,
}

impl fmt::Display for Elmt {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Elmt {
  pub fn new(
    kind: Box<ElmtKind>,
    attrs: Vec<Box<AttrKind>>,
    children: Vec<Box<Elmt>>,
  ) -> Elmt {
    Self {
      kind,
      attrs,
      children,
    }
  }

  pub fn add_children(&mut self, children: Vec<Box<Elmt>>) -> &mut Self {
    self.children = children;
    self
  }

  pub fn add_kind(&mut self, kind: Box<ElmtKind>) -> &mut Self {
    self.kind = kind;
    self
  }

  pub fn key(&self) -> std::option::Option<&String> {
    for attr in &self.attrs {
      if let Key(ref key) = **attr {
        return Some(key);
      }
    }

    None
  }

  pub fn interpret(
    &mut self,
    interpreter: &mut Interpreter,
  ) -> CompilerResult<Box<Elmt>> {
    match *self.kind {
      ElmtKind::Tag(Script) => match *self.children[0].kind {
        ElmtKind::Txt(ref source) => {
          crate::lang::computer::interpretify(
            &source.trim(),
            &mut interpreter.interpreter_lang,
          )?;
        }
        _ => {}
      },
      ElmtKind::Tag(_) => {
        let children = self
          .children
          .iter()
          .map(|c| c.to_owned().interpret(interpreter))
          .collect::<CompilerResult<Vec<Box<Elmt>>>>()?;

        self.add_children(children);
      }
      ElmtKind::Txt(ref content) => {
        let re = Regex::new(r"\{(.*?)\}").unwrap();

        let result = re.replace(&content, |cap: &Captures| {
          let content_interpreted = crate::lang::computer::interpretify(
            &cap[1],
            &mut interpreter.interpreter_lang,
          )
          .unwrap();

          format!("{}", &content_interpreted.text())
        });

        self.add_kind(Box::new(ElmtKind::Txt(result.to_string())));
      }
    };

    Ok(Box::new(self.to_owned()))
  }

  pub fn text(&self) -> String {
    match *self.kind {
      Tag(ref tag) => {
        let kw = tag.to_string();

        // TODO: use tag_void vec instead
        if self.children.is_empty() {
          return format!("<{} />", kw);
        }

        let attrs = self
          .attrs
          .iter()
          .map(|child| child.text())
          .collect::<Vec<_>>()
          .join(" ");

        let children = self
          .children
          .iter()
          .map(|child| child.text())
          .collect::<Vec<_>>()
          .join("\n");

        format!("<{} {}>\n{}\n</{}>", kw, attrs, children, kw)
      }
      Txt(ref value) => format!("{}", value),
    }
  }

  pub fn transpile(
    &mut self,
    transpiler: &mut Transpiler,
  ) -> CompilerResult<Box<Elmt>> {
    match *self.kind {
      ElmtKind::Tag(Script) => match *self.children[0].kind {
        ElmtKind::Txt(ref source) => {
          let source_transpiled =
            crate::html::transformer::transformify(&source.trim())?;

          self.add_children(vec![Box::new(Elmt::new(
            Box::new(ElmtKind::Txt(source_transpiled.into())),
            vec![],
            vec![],
          ))]);
        }
        _ => {}
      },
      ElmtKind::Tag(Style) => match *self.children[0].kind {
        ElmtKind::Txt(ref source) => {
          let source_transpiled =
            compile_scss(&source.as_bytes(), Default::default())
              .and_then(|s| Ok(String::from_utf8(s)?))
              .map_err(|e| {
                eprintln!("{}", e);
                "rsass failed"
              })
              .unwrap();

          self.add_children(vec![Box::new(Elmt::new(
            Box::new(ElmtKind::Txt(source_transpiled.into())),
            vec![],
            vec![],
          ))]);
        }
        _ => {}
      },
      ElmtKind::Tag(_) => {}
      _ => {}
    };

    Ok(Box::new(self.to_owned()))
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
  pub elmts: Vec<Box<Elmt>>,
}

impl fmt::Display for Program {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl Program {
  pub fn new(elmts: Vec<Box<Elmt>>) -> Program {
    Self { elmts }
  }

  pub fn add_elmts(&mut self, elmts: Vec<Box<Elmt>>) -> &mut Self {
    self.elmts = elmts;
    self
  }

  pub fn interpret(
    &mut self,
    interpreter: &mut Interpreter,
  ) -> CompilerResult<Box<Program>> {
    let elmts = self
      .elmts
      .iter()
      .map(|elmt| elmt.to_owned().interpret(interpreter))
      .collect::<CompilerResult<Vec<Box<Elmt>>>>()?;

    Ok(Box::new(self.add_elmts(elmts).to_owned()))
  }

  fn text(&self) -> String {
    let elmts = self
      .elmts
      .iter()
      .map(|elmt| elmt.text())
      .collect::<Vec<String>>()
      .join("");

    format!("{}", elmts)
  }

  pub fn transpile(
    &mut self,
    transpiler: &mut Transpiler,
  ) -> CompilerResult<String> {
    // let elmts = self
    //   .elmts
    //   .iter()
    //   .map(|e| e.kind.transpile(self))
    //   .collect::<CompilerResult<Vec<String>>>()?
    //   .join("");

    Ok(format!(""))
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AttrKind {
  Key(Box<String>),
  Prop(Box<String>, Box<PropKind>),
  Styles(Box<String>, Box<String>),
}

impl fmt::Display for AttrKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl AttrKind {
  pub fn text(&self) -> String {
    match *self {
      Self::Key(ref value) => format!("{}", *value),
      Self::Prop(ref prop, ref value) => format!("{}=\"{}\"", *prop, *value,),
      Self::Styles(ref prop, ref value) => format!("{}:{}", *prop, *value),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PropKind {
  Bool(Box<String>),
  Str(Box<String>),
}

impl fmt::Display for PropKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.text())
  }
}

impl PropKind {
  pub fn text(&self) -> String {
    match *self {
      Self::Bool(ref value) => format!("{}", *value),
      Self::Str(ref value) => format!("{}", *value),
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TagKind {
  A,
  Abbr,
  Address,
  Area,
  Article,
  Aside,
  Audio,
  B,
  Base,
  Bdi,
  Bdo,
  Blockquote,
  Body,
  Br,
  Button,
  Canvas,
  Caption,
  Cite,
  Code,
  Col,
  Colgroup,
  Data,
  Datalist,
  Dd,
  Del,
  Details,
  Dfn,
  Dialog,
  Div,
  Dl,
  Dt,
  Em,
  Embed,
  Fieldset,
  Figcaption,
  Figure,
  Footer,
  Form,
  H1,
  H2,
  H3,
  H4,
  H5,
  H6,
  Head,
  Header,
  Hr,
  Html,
  I,
  Iframe,
  Img,
  Input,
  Ins,
  Kdb,
  Label,
  Legend,
  Li,
  Link,
  Main,
  Map,
  Mark,
  Meta,
  Meter,
  Nav,
  Noscript,
  Object,
  Ol,
  Optgroup,
  Option,
  Output,
  P,
  Param,
  Picture,
  Pre,
  Progress,
  Q,
  Rp,
  Rt,
  Ruby,
  S,
  Samp,
  Script,
  Section,
  Select,
  Small,
  Source,
  Span,
  Strong,
  Style,
  Sub,
  Summary,
  Sup,
  Svg,
  Table,
  Tbody,
  Td,
  Template,
  Textarea,
  Tfoot,
  Th,
  Thead,
  Time,
  Title,
  Tr,
  Track,
  Tt,
  U,
  Ul,
  Var,
  Video,
  Wbr,
}

impl From<TagKind> for String {
  fn from(tag: TagKind) -> String {
    tag.to_string()
  }
}

impl TagKind {
  pub fn keywords(name: &str) -> ElmtKind {
    match name {
      "a" => Tag(A),
      "abbr" => Tag(Abbr),
      "address" => Tag(Address),
      "area" => Tag(Area),
      "article" => Tag(Article),
      "aside" => Tag(Aside),
      "audio" => Tag(Audio),
      "b" => Tag(B),
      "base" => Tag(Base),
      "bdi" => Tag(Bdi),
      "bdo" => Tag(Bdo),
      "blockquote" => Tag(Blockquote),
      "body" => Tag(Body),
      "br" => Tag(Br),
      "button" => Tag(Button),
      "canvas" => Tag(Canvas),
      "caption" => Tag(Caption),
      "cite" => Tag(Cite),
      "code" => Tag(Code),
      "col" => Tag(Col),
      "colgroup" => Tag(Colgroup),
      "data" => Tag(Data),
      "dd" => Tag(Dd),
      "del" => Tag(Del),
      "details" => Tag(Details),
      "dfn" => Tag(Dfn),
      "dialog" => Tag(Dialog),
      "div" => Tag(Div),
      "dl" => Tag(Dl),
      "dt" => Tag(Dt),
      "em" => Tag(Em),
      "embed" => Tag(Embed),
      "fieldset" => Tag(Fieldset),
      "figcaption" => Tag(Figcaption),
      "figure" => Tag(Figure),
      "footer" => Tag(Footer),
      "form" => Tag(Form),
      "h1" => Tag(H1),
      "h2" => Tag(H2),
      "h3" => Tag(H3),
      "h4" => Tag(H4),
      "h5" => Tag(H5),
      "h6" => Tag(H6),
      "head" => Tag(Head),
      "header" => Tag(Header),
      "hr" => Tag(Hr),
      "html" => Tag(Html),
      "i" => Tag(I),
      "iframe" => Tag(Iframe),
      "img" => Tag(Img),
      "input" => Tag(Input),
      "ins" => Tag(Ins),
      "kdb" => Tag(Kdb),
      "label" => Tag(Label),
      "legend" => Tag(Legend),
      "li" => Tag(Li),
      "link" => Tag(Link),
      "main" => Tag(Main),
      "map" => Tag(Map),
      "mark" => Tag(Mark),
      "meta" => Tag(Meta),
      "meter" => Tag(Meter),
      "nav" => Tag(Nav),
      "noscript" => Tag(Noscript),
      "object" => Tag(Object),
      "ol" => Tag(Ol),
      "optgroup" => Tag(Optgroup),
      "option" => Tag(Option),
      "output" => Tag(Output),
      "p" => Tag(P),
      "param" => Tag(Param),
      "picture" => Tag(Picture),
      "pre" => Tag(Pre),
      "progress" => Tag(Progress),
      "q" => Tag(Q),
      "rp" => Tag(Rp),
      "rt" => Tag(Rt),
      "ruby" => Tag(Ruby),
      "s" => Tag(S),
      "samp" => Tag(Samp),
      "script" => Tag(Script),
      "section" => Tag(Section),
      "select" => Tag(Select),
      "small" => Tag(Small),
      "source" => Tag(Source),
      "span" => Tag(Span),
      "strong" => Tag(Strong),
      "style" => Tag(Style),
      "sub" => Tag(Sub),
      "summary" => Tag(Summary),
      "sup" => Tag(Sup),
      "svg" => Tag(Svg),
      "table" => Tag(Table),
      "tbody" => Tag(Tbody),
      "td" => Tag(Td),
      "template" => Tag(Template),
      "textarea" => Tag(Textarea),
      "tfoot" => Tag(Tfoot),
      "th" => Tag(Th),
      "thead" => Tag(Thead),
      "time" => Tag(Time),
      "title" => Tag(Title),
      "tr" => Tag(Tr),
      "track" => Tag(Track),
      "tt" => Tag(Tt),
      "u" => Tag(U),
      "ul" => Tag(Ul),
      "var" => Tag(Var),
      "video" => Tag(Video),
      "wbr" => Tag(Wbr),
      _ => Txt(name.into()),
    }
  }
}

macro_rules! kws {
  { $type:tt { $($kind:ident: $value:expr,)* } } => {
    impl std::fmt::Display for $type {
      fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
          $($kind => write!(f, "{}", $value),)*
        }
      }
    }
  }
}

kws! {
  TagKind {
    A: "a",
    Abbr: "abbr",
    Address: "address",
    Area: "area",
    Article: "article",
    Aside: "aside",
    Audio: "audio",
    B: "b",
    Base: "base",
    Bdi: "bdi",
    Bdo: "bdo",
    Blockquote: "blockquote",
    Body: "body",
    Br: "br",
    Button: "button",
    Canvas: "canvas",
    Caption: "caption",
    Cite: "cite",
    Code: "code",
    Col: "col",
    Colgroup: "colgroup",
    Data: "data",
    Datalist: "datalist",
    Dd: "dd",
    Del: "del",
    Details: "details",
    Dfn: "dfn",
    Dialog: "dialog",
    Div: "div",
    Dl: "dl",
    Dt: "dt",
    Em: "em",
    Embed: "embed",
    Fieldset: "fieldset",
    Figcaption: "figcaption",
    Figure: "figure",
    Footer: "footer",
    Form: "form",
    H1: "h1",
    H2: "h2",
    H3: "h3",
    H4: "h4",
    H5: "h5",
    H6: "h6",
    Head: "head",
    Header: "header",
    Hr: "hr",
    Html: "html",
    I: "i",
    Iframe: "iframe",
    Img: "img",
    Input: "input",
    Ins: "ins",
    Kdb: "kdb",
    Label: "label",
    Legend: "legend",
    Li: "li",
    Link: "link",
    Main: "main",
    Map: "map",
    Mark: "mark",
    Meta: "meta",
    Meter: "meter",
    Nav: "nav",
    Noscript: "noscript",
    Object: "object",
    Ol: "ol",
    Optgroup: "optgroup",
    Option: "option",
    Output: "output",
    P: "p",
    Param: "param",
    Picture: "picture",
    Pre: "pre",
    Progress: "progress",
    Q: "q",
    Rp: "rp",
    Rt: "rt",
    Ruby: "ruby",
    S: "s",
    Samp: "samp",
    Script: "script",
    Section: "section",
    Select: "select",
    Small: "small",
    Source: "source",
    Span: "span",
    Strong: "strong",
    Style: "style",
    Sub: "sub",
    Summary: "summary",
    Sup: "sup",
    Svg: "svg",
    Table: "table",
    Tbody: "tbody",
    Td: "td",
    Template: "template",
    Textarea: "textarea",
    Tfoot: "tfoot",
    Th: "th",
    Thead: "thead",
    Time: "time",
    Title: "title",
    Tr: "tr",
    Track: "track",
    Tt: "tt",
    U: "u",
    Ul: "ul",
    Var: "var",
    Video: "video",
    Wbr: "wbr",
  }
}
