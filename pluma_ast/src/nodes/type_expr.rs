use super::*;
use crate::common::*;
use crate::value_type::ValueType;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct TypeExprNode {
  pub pos: Position,
  pub kind: TypeExprKind,
  pub typ: ValueType,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub enum TypeExprKind {
  // e.g. String or Dict<Int, String>
  Single(TypeIdentifierNode),
  // e.g. String -> Bool
  Func(Box<TypeExprNode>, Box<TypeExprNode>),
  // e.g. (String, Bool)
  UnlabeledTuple(Vec<TypeExprNode>),
  // e.g. (a: String, b: Bool)
  LabeledTuple(Vec<(IdentifierNode, TypeExprNode)>),
  // e.g. ()
  EmptyTuple,
  // e.g. (String) or (String -> Bool)
  Grouping(Box<TypeExprNode>),
}
