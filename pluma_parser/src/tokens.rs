use std::fmt;

#[derive(Copy, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Token {
  Arrow(usize, usize),
  BinaryDigits(usize, usize),
  Colon(usize, usize),
  Comma(usize, usize),
  Comment(usize, usize),
  DecimalDigits(usize, usize),
  Dot(usize, usize),
  DoubleArrow(usize, usize),
  DoubleDot(usize, usize),
  DoubleColon(usize, usize),
  Equals(usize, usize),
  ForwardSlash(usize, usize),
  HexDigits(usize, usize),
  Identifier(usize, usize),
  IdentifierSpecialOther(usize, usize),
  IdentifierSpecialParam(usize, usize),
  ImportPath(usize, usize),
  InterpolationEnd(usize, usize),
  InterpolationStart(usize, usize),
  KeywordAlias(usize, usize),
  KeywordAs(usize, usize),
  KeywordBreak(usize, usize),
  KeywordConst(usize, usize),
  KeywordDef(usize, usize),
  KeywordEnum(usize, usize),
  KeywordInternal(usize, usize),
  KeywordIntrinsicDef(usize, usize),
  KeywordIntrinsicType(usize, usize),
  KeywordLet(usize, usize),
  KeywordMatch(usize, usize),
  KeywordMut(usize, usize),
  KeywordPrivate(usize, usize),
  KeywordStruct(usize, usize),
  KeywordTrait(usize, usize),
  KeywordUse(usize, usize),
  KeywordWhere(usize, usize),
  LeftAngle(usize, usize),
  LeftBrace(usize, usize),
  LeftBracket(usize, usize),
  LeftParen(usize, usize),
  LineBreak(usize, usize),
  OctalDigits(usize, usize),
  Operator(usize, usize),
  Pipe(usize, usize),
  RightAngle(usize, usize),
  RightBrace(usize, usize),
  RightBracket(usize, usize),
  RightParen(usize, usize),
  StringLiteral(usize, usize),
  Underscore(usize, usize),
  Unexpected(usize, usize),
}

impl Token {
  pub fn get_position(&self) -> (usize, usize) {
    match self {
      &Token::Arrow(start, end) => (start, end),
      &Token::BinaryDigits(start, end) => (start, end),
      &Token::Colon(start, end) => (start, end),
      &Token::Comma(start, end) => (start, end),
      &Token::Comment(start, end) => (start, end),
      &Token::DecimalDigits(start, end) => (start, end),
      &Token::Dot(start, end) => (start, end),
      &Token::DoubleArrow(start, end) => (start, end),
      &Token::DoubleDot(start, end) => (start, end),
      &Token::DoubleColon(start, end) => (start, end),
      &Token::Equals(start, end) => (start, end),
      &Token::ForwardSlash(start, end) => (start, end),
      &Token::HexDigits(start, end) => (start, end),
      &Token::Identifier(start, end) => (start, end),
      &Token::IdentifierSpecialOther(start, end) => (start, end),
      &Token::IdentifierSpecialParam(start, end) => (start, end),
      &Token::ImportPath(start, end) => (start, end),
      &Token::InterpolationEnd(start, end) => (start, end),
      &Token::InterpolationStart(start, end) => (start, end),
      &Token::KeywordAlias(start, end) => (start, end),
      &Token::KeywordAs(start, end) => (start, end),
      &Token::KeywordBreak(start, end) => (start, end),
      &Token::KeywordConst(start, end) => (start, end),
      &Token::KeywordDef(start, end) => (start, end),
      &Token::KeywordEnum(start, end) => (start, end),
      &Token::KeywordInternal(start, end) => (start, end),
      &Token::KeywordIntrinsicDef(start, end) => (start, end),
      &Token::KeywordIntrinsicType(start, end) => (start, end),
      &Token::KeywordLet(start, end) => (start, end),
      &Token::KeywordMatch(start, end) => (start, end),
      &Token::KeywordMut(start, end) => (start, end),
      &Token::KeywordPrivate(start, end) => (start, end),
      &Token::KeywordStruct(start, end) => (start, end),
      &Token::KeywordTrait(start, end) => (start, end),
      &Token::KeywordUse(start, end) => (start, end),
      &Token::KeywordWhere(start, end) => (start, end),
      &Token::LeftAngle(start, end) => (start, end),
      &Token::LeftBrace(start, end) => (start, end),
      &Token::LeftBracket(start, end) => (start, end),
      &Token::LeftParen(start, end) => (start, end),
      &Token::LineBreak(start, end) => (start, end),
      &Token::OctalDigits(start, end) => (start, end),
      &Token::Operator(start, end) => (start, end),
      &Token::Pipe(start, end) => (start, end),
      &Token::RightAngle(start, end) => (start, end),
      &Token::RightBrace(start, end) => (start, end),
      &Token::RightBracket(start, end) => (start, end),
      &Token::RightParen(start, end) => (start, end),
      &Token::StringLiteral(start, end) => (start, end),
      &Token::Underscore(start, end) => (start, end),
      &Token::Unexpected(start, end) => (start, end),
    }
  }
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let as_string = match self {
      &Token::Arrow(..) => "a '->'",
      &Token::BinaryDigits(..) => "binary digits",
      &Token::Colon(..) => "a ':'",
      &Token::Comma(..) => "a ','",
      &Token::Comment(..) => "a comment",
      &Token::DecimalDigits(..) => "digits",
      &Token::Dot(..) => "a '.'",
      &Token::DoubleArrow(..) => "a '=>'",
      &Token::DoubleDot(..) => "a '..'",
      &Token::DoubleColon(..) => "a '::'",
      &Token::Equals(..) => "a '='",
      &Token::ForwardSlash(..) => "a '/'",
      &Token::HexDigits(..) => "hex digits",
      &Token::Identifier(..) => "an identifier",
      &Token::IdentifierSpecialOther(..) => "an identifier starting with '$'",
      &Token::IdentifierSpecialParam(..) => "an identifier starting with '$'",
      &Token::ImportPath(..) => "an import path",
      &Token::InterpolationEnd(..) => "a ')'",
      &Token::InterpolationStart(..) => "a '$('",
      &Token::KeywordAlias(..) => "keyword 'alias'",
      &Token::KeywordAs(..) => "keyword 'as'",
      &Token::KeywordBreak(..) => "keyword 'break'",
      &Token::KeywordConst(..) => "keyword 'const'",
      &Token::KeywordDef(..) => "keyword 'def'",
      &Token::KeywordEnum(..) => "keyword 'enum'",
      &Token::KeywordInternal(..) => "keyword 'internal'",
      &Token::KeywordIntrinsicDef(..) => "keyword 'intrinsic_def'",
      &Token::KeywordIntrinsicType(..) => "keyword 'intrinsic_type'",
      &Token::KeywordLet(..) => "keyword 'let'",
      &Token::KeywordMatch(..) => "keyword 'match'",
      &Token::KeywordMut(..) => "keyword 'mut'",
      &Token::KeywordPrivate(..) => "keyword 'private'",
      &Token::KeywordStruct(..) => "keyword 'struct'",
      &Token::KeywordTrait(..) => "keyword 'trait'",
      &Token::KeywordUse(..) => "keyword 'use'",
      &Token::KeywordWhere(..) => "keyword 'where'",
      &Token::LeftAngle(..) => "a '<'",
      &Token::LeftBrace(..) => "a '{'",
      &Token::LeftBracket(..) => "a '['",
      &Token::LeftParen(..) => "a '('",
      &Token::LineBreak(..) => "a line break",
      &Token::OctalDigits(..) => "octal digits",
      &Token::Operator(..) => "an operator",
      &Token::Pipe(..) => "a '|'",
      &Token::RightAngle(..) => "a '>'",
      &Token::RightBrace(..) => "a '}'",
      &Token::RightBracket(..) => "a ']'",
      &Token::RightParen(..) => "a ')'",
      &Token::StringLiteral(..) => "a string",
      &Token::Underscore(..) => "a '_'",
      &Token::Unexpected(..) => "unknown",
    };

    write!(f, "{}", as_string)
  }
}
