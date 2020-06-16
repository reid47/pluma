use super::*;
use crate::common::*;

#[derive(Debug)]
pub struct PatternNode {
  pub pos: Position,
  pub kind: PatternKind,
}

#[derive(Debug)]
pub enum PatternKind {
  // e.g. let x =
  Identifier(IdentifierNode),
  // e.g. let Person (x, y) =
  Constructor(IdentifierNode, Box<PatternNode>),
  // e.g. let (x, y) =
  Tuple(Vec<PatternNode>),
  // e.g. '_' in let (x, _) =
  Underscore,
  // e.g. '1' in match x | 1 => "yes" | _ => "no"
  Literal(LiteralNode),
  // e.g. match str | "$(thing)?" => "yes" | _ => "no"
  Interpolation(Vec<ExprNode>),
}