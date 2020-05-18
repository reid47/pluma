use crate::ast::*;
use crate::visitor_mut::VisitorMut;

pub trait TraverseMut {
  fn traverse<V: VisitorMut>(&mut self, visitor: &mut V) {}
}

impl TraverseMut for DefNode {
  // todo
}

impl TraverseMut for ExprNode {
  fn traverse<V: VisitorMut>(&mut self, visitor: &mut V) {
    visitor.enter_expr(self);

    match &mut self.kind {
      ExprKind::Assignment { left, right } => {
        right.traverse(visitor);
        left.traverse(visitor);
      }
      ExprKind::Block { params, body } => {
        for param in params {
          param.traverse(visitor);
        }

        for stmt in body {
          stmt.traverse(visitor);
        }
      }
      ExprKind::Call { callee, args } => {
        for arg in args {
          arg.traverse(visitor);
        }

        callee.traverse(visitor);
      }
      ExprKind::Literal(literal) => literal.traverse(visitor),
      ExprKind::Identifier(ident) => ident.traverse(visitor),
      ExprKind::Interpolation(parts) => {
        for part in parts {
          part.traverse(visitor);
        }
      }
      ExprKind::Tuple(entries) => {
        for entry in entries {
          entry.traverse(visitor);
        }
      }
      other_kind => todo!("traverse {:#?}", other_kind),
    }

    visitor.leave_expr(self);
  }
}

impl TraverseMut for IdentifierNode {
  fn traverse<V: VisitorMut>(&mut self, visitor: &mut V) {
    visitor.enter_identifier(self);

    visitor.leave_identifier(self);
  }
}

impl TraverseMut for LetNode {
  fn traverse<V: VisitorMut>(&mut self, visitor: &mut V) {
    visitor.enter_let(self);

    self.pattern.traverse(visitor);
    self.value.traverse(visitor);

    visitor.leave_let(self);
  }
}

impl TraverseMut for LiteralNode {
  fn traverse<V: VisitorMut>(&mut self, visitor: &mut V) {
    visitor.enter_literal(self);

    visitor.leave_literal(self);
  }
}

impl TraverseMut for MatchNode {
  // todo
}

impl TraverseMut for MatchCaseNode {
  // todo
}

impl TraverseMut for ModuleNode {
  fn traverse<V: VisitorMut>(&mut self, visitor: &mut V) {
    visitor.enter_module(self);

    for node in &mut self.body {
      node.traverse(visitor);
    }

    visitor.leave_module(self);
  }
}

impl TraverseMut for OperatorNode {
  // todo
}

impl TraverseMut for PatternNode {
  fn traverse<V: VisitorMut>(&mut self, visitor: &mut V) {
    visitor.enter_pattern(self);

    // ?

    visitor.leave_pattern(self);
  }
}

impl TraverseMut for ReturnNode {
  fn traverse<V: VisitorMut>(&mut self, visitor: &mut V) {
    visitor.enter_return(self);

    self.value.traverse(visitor);

    visitor.leave_return(self);
  }
}

impl TraverseMut for StatementNode {
  fn traverse<V: VisitorMut>(&mut self, visitor: &mut V) {
    visitor.enter_statement(self);

    match &mut self.kind {
      StatementKind::Let(node) => node.traverse(visitor),
      StatementKind::Expr(node) => node.traverse(visitor),
      StatementKind::Return(node) => node.traverse(visitor),
    };

    visitor.leave_statement(self);
  }
}

impl TraverseMut for TopLevelStatementNode {
  fn traverse<V: VisitorMut>(&mut self, visitor: &mut V) {
    visitor.enter_top_level_statement(self);

    match &mut self.kind {
      TopLevelStatementKind::Let(node) => node.traverse(visitor),
      TopLevelStatementKind::TypeDef(node) => node.traverse(visitor),
      TopLevelStatementKind::Def(node) => node.traverse(visitor),
      TopLevelStatementKind::Expr(node) => node.traverse(visitor),
    };

    visitor.leave_top_level_statement(self);
  }
}

impl TraverseMut for TypeExprNode {
  // todo
}

impl TraverseMut for TypeDefNode {
  fn traverse<V: VisitorMut>(&mut self, visitor: &mut V) {
    visitor.enter_type_def(self);

    // match &self.kind {
    //   TypeDefKind::Enum { variants } => {
    //     for variant in variants {
    //       visitor.visit_type(&variant);
    //     }
    //   }
    //   _ => todo!("not yet implemented"),
    // }

    visitor.leave_type_def(self);
  }
}