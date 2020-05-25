use crate::visitor::Visitor;
use pluma_ast::nodes::*;

pub trait Traverse {
  fn traverse<V: Visitor>(&mut self, _visitor: &mut V) {}
}

impl Traverse for CallNode {
  fn traverse<V: Visitor>(&mut self, visitor: &mut V) {
    visitor.enter_call(self);

    for arg in &mut self.args {
      arg.traverse(visitor);
    }

    self.callee.traverse(visitor);

    visitor.leave_call(self);
  }
}

impl Traverse for DefNode {
  fn traverse<V: Visitor>(&mut self, visitor: &mut V) {
    visitor.enter_def(self);

    match &mut self.kind {
      DefKind::BinaryOperator { left, op, right } => {
        right.traverse(visitor);
        left.traverse(visitor);
        op.traverse(visitor);
      }

      DefKind::Function { signature } => {
        for (ident, type_expr) in signature {
          ident.traverse(visitor);
          type_expr.traverse(visitor);
        }
      }

      DefKind::Method {
        receiver,
        signature,
      } => {
        for (ident, type_expr) in signature {
          ident.traverse(visitor);
          type_expr.traverse(visitor);
        }

        receiver.traverse(visitor);
      }

      DefKind::UnaryOperator { op, right } => {
        right.traverse(visitor);
        op.traverse(visitor);
      }
    }

    visitor.leave_def(self);
  }
}

impl Traverse for ExprNode {
  fn traverse<V: Visitor>(&mut self, visitor: &mut V) {
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
      ExprKind::Call(call) => call.traverse(visitor),
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
      ExprKind::EmptyTuple => {}
      ExprKind::MultiPartIdentifier(parts) => {
        for part in parts {
          part.traverse(visitor);
        }
      }
      ExprKind::FieldAccess { receiver, field } => {
        receiver.traverse(visitor);
        field.traverse(visitor);
      }
      ExprKind::MethodAccess {
        receiver,
        method_parts,
      } => {
        receiver.traverse(visitor);
        for part in method_parts {
          part.traverse(visitor);
        }
      }
      other_kind => todo!("traverse {:#?}", other_kind),
    }

    visitor.leave_expr(self);
  }
}

impl Traverse for IdentifierNode {
  fn traverse<V: Visitor>(&mut self, visitor: &mut V) {
    visitor.enter_identifier(self);

    visitor.leave_identifier(self);
  }
}

impl Traverse for IntrinsicDefNode {
  fn traverse<V: Visitor>(&mut self, visitor: &mut V) {
    visitor.enter_intrinsic_def_node(self);

    visitor.leave_intrinsic_def_node(self);
  }
}

impl Traverse for IntrinsicTypeDefNode {
  fn traverse<V: Visitor>(&mut self, visitor: &mut V) {
    visitor.enter_intrinsic_type_def(self);

    visitor.leave_intrinsic_type_def(self);
  }
}

impl Traverse for LetNode {
  fn traverse<V: Visitor>(&mut self, visitor: &mut V) {
    visitor.enter_let(self);

    self.pattern.traverse(visitor);
    self.value.traverse(visitor);

    visitor.leave_let(self);
  }
}

impl Traverse for LiteralNode {
  fn traverse<V: Visitor>(&mut self, visitor: &mut V) {
    visitor.enter_literal(self);

    visitor.leave_literal(self);
  }
}

impl Traverse for MatchNode {
  // todo
}

impl Traverse for MatchCaseNode {
  // todo
}

impl Traverse for ModuleNode {
  fn traverse<V: Visitor>(&mut self, visitor: &mut V) {
    visitor.enter_module(self);

    for node in &mut self.body {
      node.traverse(visitor);
    }

    visitor.leave_module(self);
  }
}

impl Traverse for OperatorNode {
  // todo
}

impl Traverse for PatternNode {
  fn traverse<V: Visitor>(&mut self, visitor: &mut V) {
    visitor.enter_pattern(self);

    // ?

    visitor.leave_pattern(self);
  }
}

impl Traverse for ReturnNode {
  fn traverse<V: Visitor>(&mut self, visitor: &mut V) {
    visitor.enter_return(self);

    self.value.traverse(visitor);

    visitor.leave_return(self);
  }
}

impl Traverse for StatementNode {
  fn traverse<V: Visitor>(&mut self, visitor: &mut V) {
    visitor.enter_statement(self);

    match &mut self.kind {
      StatementKind::Let(node) => node.traverse(visitor),
      StatementKind::Expr(node) => node.traverse(visitor),
      StatementKind::Return(node) => node.traverse(visitor),
    };

    visitor.leave_statement(self);
  }
}

impl Traverse for TopLevelStatementNode {
  fn traverse<V: Visitor>(&mut self, visitor: &mut V) {
    visitor.enter_top_level_statement(self);

    match &mut self.kind {
      TopLevelStatementKind::Let(node) => node.traverse(visitor),
      TopLevelStatementKind::TypeDef(node) => node.traverse(visitor),
      TopLevelStatementKind::Def(node) => node.traverse(visitor),
      TopLevelStatementKind::Expr(node) => node.traverse(visitor),
      TopLevelStatementKind::IntrinsicDef(node) => node.traverse(visitor),
      TopLevelStatementKind::IntrinsicTypeDef(node) => node.traverse(visitor),
      TopLevelStatementKind::PrivateMarker => {}
    };

    visitor.leave_top_level_statement(self);
  }
}

impl Traverse for TypeExprNode {
  // todo
}

impl Traverse for TypeDefNode {
  fn traverse<V: Visitor>(&mut self, visitor: &mut V) {
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

impl Traverse for TypeIdentifierNode {
  fn traverse<V: Visitor>(&mut self, visitor: &mut V) {
    visitor.enter_type_identifier(self);

    visitor.leave_type_identifier(self);
  }
}
