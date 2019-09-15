// TODO: use this to only derive in tests
// #[cfg_attr(test, derive(Debug))]
#[derive(Debug)]
pub enum Token {
  Arrow(usize, usize),
  BinaryDigits(usize, usize),
  Colon(usize, usize),
  ColonEquals(usize, usize),
  Comma(usize, usize),
  Comment(usize, usize),
  DecimalDigits(usize, usize),
  Dot(usize, usize),
  DoubleArrow(usize, usize),
  DoubleColon(usize, usize),
  Equals(usize, usize),
  HexDigits(usize, usize),
  Identifier(usize, usize),
  InterpolationEnd(usize, usize),
  InterpolationStart(usize, usize),
  KeywordAs(usize, usize),
  KeywordDef(usize, usize),
  KeywordLet(usize, usize),
  KeywordMatch(usize, usize),
  KeywordType(usize, usize),
  KeywordUse(usize, usize),
  LeftBrace(usize, usize),
  LeftBracket(usize, usize),
  LeftParen(usize, usize),
  LineBreak(usize, usize),
  Minus(usize, usize),
  OctalDigits(usize, usize),
  Pipe(usize, usize),
  RightBrace(usize, usize),
  RightBracket(usize, usize),
  RightParen(usize, usize),
  StringLiteral(usize, usize),
  Unexpected(usize, usize),
}

pub fn get_token_location(token: &Token) -> (usize, usize) {
  match token {
    Token::Arrow(start, end) => (*start, *end),
    Token::BinaryDigits(start, end) => (*start, *end),
    Token::Colon(start, end) => (*start, *end),
    Token::ColonEquals(start, end) => (*start, *end),
    Token::Comma(start, end) => (*start, *end),
    Token::Comment(start, end) => (*start, *end),
    Token::DecimalDigits(start, end) => (*start, *end),
    Token::Dot(start, end) => (*start, *end),
    Token::DoubleArrow(start, end) => (*start, *end),
    Token::DoubleColon(start, end) => (*start, *end),
    Token::Equals(start, end) => (*start, *end),
    Token::HexDigits(start, end) => (*start, *end),
    Token::Identifier(start, end) => (*start, *end),
    Token::InterpolationEnd(start, end) => (*start, *end),
    Token::InterpolationStart(start, end) => (*start, *end),
    Token::KeywordAs(start, end) => (*start, *end),
    Token::KeywordDef(start, end) => (*start, *end),
    Token::KeywordLet(start, end) => (*start, *end),
    Token::KeywordMatch(start, end) => (*start, *end),
    Token::KeywordType(start, end) => (*start, *end),
    Token::KeywordUse(start, end) => (*start, *end),
    Token::LeftBrace(start, end) => (*start, *end),
    Token::LeftBracket(start, end) => (*start, *end),
    Token::LeftParen(start, end) => (*start, *end),
    Token::LineBreak(start, end) => (*start, *end),
    Token::Minus(start, end) => (*start, *end),
    Token::OctalDigits(start, end) => (*start, *end),
    Token::Pipe(start, end) => (*start, *end),
    Token::RightBrace(start, end) => (*start, *end),
    Token::RightBracket(start, end) => (*start, *end),
    Token::RightParen(start, end) => (*start, *end),
    Token::StringLiteral(start, end) => (*start, *end),
    Token::Unexpected(start, end) => (*start, *end),
  }
}