use pluma_ast::*;

pub trait Visitor {
  fn enter_block(&mut self, _node: &BlockNode) {}

  fn leave_block(&mut self, _node: &BlockNode) {}

  fn enter_call(&mut self, _node: &CallNode) {}

  fn leave_call(&mut self, _node: &CallNode) {}

  fn enter_const(&mut self, _node: &ConstNode) {}

  fn leave_const(&mut self, _node: &ConstNode) {}

  fn enter_def(&mut self, _node: &DefNode) {}

  fn leave_def(&mut self, _node: &DefNode) {}

  fn enter_expr(&mut self, _node: &ExprNode) {}

  fn leave_expr(&mut self, _node: &ExprNode) {}

  fn enter_identifier(&mut self, _node: &IdentifierNode) {}

  fn leave_identifier(&mut self, _node: &IdentifierNode) {}

  fn enter_intrinsic_def(&mut self, _node: &IntrinsicDefNode) {}

  fn leave_intrinsic_def(&mut self, _node: &IntrinsicDefNode) {}

  fn enter_intrinsic_type_def(&mut self, _node: &IntrinsicTypeDefNode) {}

  fn leave_intrinsic_type_def(&mut self, _node: &IntrinsicTypeDefNode) {}

  fn enter_let(&mut self, _node: &LetNode) {}

  fn leave_let(&mut self, _node: &LetNode) {}

  fn enter_literal(&mut self, _node: &LiteralNode) {}

  fn leave_literal(&mut self, _node: &LiteralNode) {}

  fn enter_match(&mut self, _node: &MatchNode) {}

  fn leave_match(&mut self, _node: &MatchNode) {}

  fn enter_match_case(&mut self, _node: &MatchCaseNode) {}

  fn leave_match_case(&mut self, _node: &MatchCaseNode) {}

  fn enter_module(&mut self, _node: &ModuleNode) {}

  fn leave_module(&mut self, _node: &ModuleNode) {}

  fn enter_operator(&mut self, _node: &OperatorNode) {}

  fn leave_operator(&mut self, _node: &OperatorNode) {}

  fn enter_pattern(&mut self, _node: &PatternNode) {}

  fn leave_pattern(&mut self, _node: &PatternNode) {}

  fn enter_statement(&mut self, _node: &StatementNode) {}

  fn leave_statement(&mut self, _node: &StatementNode) {}

  fn enter_top_level_statement(&mut self, _node: &TopLevelStatementNode) {}

  fn leave_top_level_statement(&mut self, _node: &TopLevelStatementNode) {}

  fn enter_type_expr(&mut self, _node: &TypeExprNode) {}

  fn leave_type_expr(&mut self, _node: &TypeExprNode) {}

  fn enter_type_def(&mut self, _node: &TypeDefNode) {}

  fn leave_type_def(&mut self, _node: &TypeDefNode) {}

  fn enter_type_identifier(&mut self, _node: &TypeIdentifierNode) {}

  fn leave_type_identifier(&mut self, _node: &TypeIdentifierNode) {}
}
