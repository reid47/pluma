use crate::types::ValueType;
use std::fmt;
use uuid::Uuid;

pub type Position = (usize, usize);
pub type SignaturePart = (Box<IdentifierNode>, Box<TypeExprNode>);
pub type Signature = Vec<SignaturePart>;
pub type GenericTypeConstraints = Vec<(IdentifierNode, TypeExprNode)>;

#[derive(Debug)]
pub struct CallNode {
  pub pos: Position,
  pub callee: Box<ExprNode>,
  pub args: Vec<ExprNode>,
  pub typ: Option<ValueType>,
}

#[derive(Debug)]
pub struct DefNode {
  pub pos: Position,
  pub kind: DefKind,
  pub return_type: Option<TypeExprNode>,
  pub generic_type_constraints: GenericTypeConstraints,
  pub params: Vec<IdentifierNode>,
  pub body: Vec<StatementNode>,
}

#[derive(Debug)]
pub struct IntrinsicDefNode {
  pub pos: Position,
  pub kind: DefKind,
  pub return_type: Option<TypeExprNode>,
  pub generic_type_constraints: GenericTypeConstraints,
}

#[derive(Debug)]
pub enum DefKind {
  // def hi(A, B) -> Ret { ... }
  Function {
    signature: Signature,
  },
  // def (Receiver).hi() -> Ret { ... }
  Method {
    receiver: Box<TypeIdentifierNode>,
    signature: Signature,
  },
  // def (A) ++ (B) -> Ret { ... }
  BinaryOperator {
    left: Box<TypeExprNode>,
    op: Box<OperatorNode>,
    right: Box<TypeExprNode>,
  },
  // def ~(A) -> Ret { ... }
  UnaryOperator {
    op: Box<OperatorNode>,
    right: Box<TypeExprNode>,
  },
}

#[derive(Debug)]
pub struct EnumVariantNode {
  pub pos: Position,
  pub kind: EnumVariantKind,
}

#[derive(Debug)]
pub enum EnumVariantKind {
  Identifier(IdentifierNode),
  Call(CallNode),
}

#[derive(Debug)]
pub struct ExprNode {
  pub pos: Position,
  pub kind: ExprKind,
  pub typ: Option<ValueType>,
}

#[derive(Debug)]
pub enum ExprKind {
  Assignment {
    left: Box<IdentifierNode>,
    right: Box<ExprNode>,
  },
  BinaryOperation {
    left: Box<ExprNode>,
    op: Box<OperatorNode>,
    right: Box<ExprNode>,
  },
  Block {
    params: Vec<IdentifierNode>,
    body: Vec<StatementNode>,
  },
  Call(CallNode),
  Chain {
    receiver: Box<ExprNode>,
    prop: Box<ExprNode>,
  },
  Dict(Vec<(ExprNode, ExprNode)>),
  EmptyTuple,
  Grouping(Box<ExprNode>),
  Identifier(IdentifierNode),
  MultiPartIdentifier(Vec<IdentifierNode>),
  Interpolation(Vec<ExprNode>),
  List(Vec<ExprNode>),
  Literal(LiteralNode),
  Match(MatchNode),
  Tuple(Vec<ExprNode>),
  UnaryOperation {
    op: Box<OperatorNode>,
    right: Box<ExprNode>,
  },
  Underscore,
}

#[derive(Debug, Clone)]
pub struct IdentifierNode {
  pub pos: Position,
  pub name: String,
  pub typ: Option<ValueType>,
}

#[derive(Debug)]
pub struct LetNode {
  pub pos: Position,
  pub pattern: PatternNode,
  pub value: ExprNode,
}

#[derive(Debug)]
pub struct LiteralNode {
  pub pos: Position,
  pub kind: LiteralKind,
  pub typ: Option<ValueType>,
}

#[derive(Debug)]
pub enum LiteralKind {
  FloatDecimal(f64),
  IntDecimal(i128),
  IntOctal(i128),
  IntHex(i128),
  IntBinary(i128),
  Str(String),
}

#[derive(Debug)]
pub struct MatchNode {
  pub pos: Position,
  pub subject: Box<ExprNode>,
  pub cases: Vec<MatchCaseNode>,
}

#[derive(Debug)]
pub struct MatchCaseNode {
  pub pos: Position,
  pub pattern: PatternNode,
  pub body: ExprNode,
}

#[derive(Debug)]
pub struct ModuleNode {
  pub pos: Position,
  pub body: Vec<TopLevelStatementNode>,
}

#[derive(Debug)]
pub struct OperatorNode {
  pub pos: Position,
  pub name: String,
}

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

#[derive(Debug)]
pub struct ReturnNode {
  pub pos: Position,
  pub value: ExprNode,
}

#[derive(Debug)]
pub struct StatementNode {
  pub pos: Position,
  pub kind: StatementKind,
}

#[derive(Debug)]
pub enum StatementKind {
  Let(LetNode),
  Expr(ExprNode),
  Return(ReturnNode),
}

#[derive(Debug)]
pub struct TopLevelStatementNode {
  pub pos: Position,
  pub kind: TopLevelStatementKind,
}

#[derive(Debug)]
pub enum TopLevelStatementKind {
  Let(LetNode),
  TypeDef(TypeDefNode),
  IntrinsicTypeDef(IntrinsicTypeDefNode),
  Def(DefNode),
  IntrinsicDef(IntrinsicDefNode),
  Expr(ExprNode),
  PrivateMarker,
}

#[derive(Debug)]
pub struct TypeExprNode {
  pub pos: Position,
  pub kind: TypeExprKind,
}

#[derive(Debug)]
pub enum TypeExprKind {
  // e.g. String or Dict<Int, String>
  Single(TypeIdentifierNode),
  // e.g. String -> Bool
  Func(Box<TypeExprNode>, Box<TypeExprNode>),
  // e.g. (String, Bool)
  Tuple(Vec<TypeExprNode>),
  // e.g. ()
  EmptyTuple,
  // e.g. (String) or (String -> Bool)
  Grouping(Box<TypeExprNode>),
}

#[derive(Debug)]
pub struct TypeDefNode {
  pub pos: Position,
  pub kind: TypeDefKind,
  pub name: Box<TypeIdentifierNode>,
  pub generic_type_constraints: GenericTypeConstraints,
}

#[derive(Debug)]
pub struct IntrinsicTypeDefNode {
  pub pos: Position,
  pub name: Box<IdentifierNode>,
  pub generic_type_constraints: GenericTypeConstraints,
}

#[derive(Debug)]
pub enum TypeDefKind {
  // alias StringList List<String>
  Alias {
    of: TypeExprNode,
  },
  // enum Color | Red | Green | Blue
  Enum {
    variants: Vec<EnumVariantNode>,
  },
  // struct Person (name :: String, age :: Int)
  Struct {
    fields: Vec<(IdentifierNode, TypeExprNode)>,
  },
  // trait Named .name :: String .getName() -> String
  Trait {
    fields: Vec<(IdentifierNode, TypeExprNode)>,
    methods: Vec<(Signature, TypeExprNode)>,
  },
}

#[derive(Debug)]
pub struct TypeIdentifierNode {
  pub pos: Position,
  pub name: Box<IdentifierNode>,
  pub generics: Vec<TypeExprNode>,
}

#[derive(Debug, Clone)]
pub struct UseNode {
  pub pos: Position,
  pub module_name: String,
  pub qualifier: Box<IdentifierNode>,
}
