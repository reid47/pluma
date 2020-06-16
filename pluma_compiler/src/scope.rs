use crate::analysis_error::{AnalysisError, AnalysisErrorKind};
use pluma_ast::*;
use pluma_diagnostics::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Binding {
  pub typ: ValueType,
  pub ref_count: usize,
  pub pos: (usize, usize),
  pub kind: BindingKind,
}

#[derive(Debug)]
pub struct TypeBinding {
  pub ref_count: usize,
  pub pos: (usize, usize),
  pub kind: TypeBindingKind,
  pub methods: HashMap<Vec<String>, ValueType>,
}

#[derive(Debug, PartialEq)]
pub enum BindingKind {
  Let,
  Def,
  Param,
  EnumVariant,
  StructConstructor,
  Field,
}

#[derive(Debug)]
pub enum TypeBindingKind {
  Enum,
  Struct { fields: HashMap<String, Binding> },
  Alias,
  Trait { fields: HashMap<String, Binding> },
  IntrinsicType,
}

#[derive(Debug)]
struct ScopeLevel {
  pub bindings: HashMap<String, Binding>,
}

#[derive(Debug)]
pub struct Scope {
  levels: Vec<ScopeLevel>,
  type_bindings: HashMap<ValueType, TypeBinding>,
}

impl Scope {
  pub fn new() -> Self {
    Scope {
      levels: Vec::new(),
      type_bindings: HashMap::new(),
    }
  }

  pub fn enter(&mut self) {
    self.levels.push(ScopeLevel {
      bindings: HashMap::new(),
    });
  }

  pub fn exit(&mut self) -> Result<(), Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();

    if let Some(exited_level) = self.levels.pop() {
      for (name, binding) in exited_level.bindings {
        if binding.ref_count == 0 {
          diagnostics.push(
            Diagnostic::warning(AnalysisError {
              pos: binding.pos,
              kind: AnalysisErrorKind::UnusedVariable(name),
            })
            .with_pos(binding.pos),
          )
        }
      }
    }

    if diagnostics.len() > 0 {
      return Err(diagnostics);
    }

    Ok(())
  }

  pub fn add_binding(
    &mut self,
    kind: BindingKind,
    name: String,
    typ: ValueType,
    pos: (usize, usize),
  ) {
    let current_level = self.levels.last_mut().expect("no current scope");

    current_level.bindings.insert(
      name,
      Binding {
        typ,
        ref_count: 0,
        pos,
        kind,
      },
    );
  }

  pub fn add_type_binding(&mut self, typ: ValueType, kind: TypeBindingKind, pos: (usize, usize)) {
    self.type_bindings.insert(
      typ,
      TypeBinding {
        ref_count: 0,
        pos,
        kind,
        methods: HashMap::new(),
      },
    );
  }

  pub fn add_type_method(
    &mut self,
    typ: ValueType,
    method_parts: Vec<String>,
    param_types: Vec<ValueType>,
    return_type: ValueType,
    typ_pos: (usize, usize),
  ) -> Result<(), Diagnostic> {
    let binding = match self.get_type_binding(&typ) {
      Some(binding) => binding,
      None => {
        return Err(
          Diagnostic::error(AnalysisError {
            pos: typ_pos,
            kind: AnalysisErrorKind::UndefinedTypeInMethodDef(typ.clone()),
          })
          .with_pos(typ_pos),
        )
      }
    };

    let method_type = ValueType::Func(param_types, Box::new(return_type));

    binding.methods.insert(method_parts, method_type);

    // if let Some(param_types_to_return_types) = binding.methods.get_mut(&method_parts) {
    //   param_types_to_return_types.insert(param_types, return_type);
    // } else {
    //   let mut h = HashMap::new();
    //   h.insert(param_types, return_type);
    //   binding.methods.insert(method_parts, h);
    // }

    Ok(())
  }

  pub fn get_binding(&mut self, name: &String) -> Option<&Binding> {
    for level in self.levels.iter_mut().rev() {
      if let Some(binding) = level.bindings.get_mut(name) {
        binding.ref_count += 1;

        return Some(binding);
      }
    }

    None
  }

  pub fn get_type_binding(&mut self, typ: &ValueType) -> Option<&mut TypeBinding> {
    if let Some(binding) = self.type_bindings.get_mut(typ) {
      binding.ref_count += 1;

      return Some(binding);
    }

    None
  }
}