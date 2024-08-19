use crate::ast::{LineInfo, Type};
use crate::eval::EvalError;
use std::collections::HashMap;

#[derive(Debug)]
pub struct VarInfo {
    pub value: Value,
    pub var_type: Type,
    pub is_morph: bool,
}

#[derive(Debug)]
pub struct Environment {
    scopes: Vec<HashMap<String, VarInfo>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn set_var(&mut self, name: String, value: Value, var_type: Type, is_morph: bool) {
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.insert(
                name,
                VarInfo {
                    value,
                    var_type,
                    is_morph,
                },
            );
        }
    }

    pub fn get_var(&self, name: &str) -> Option<&VarInfo> {
        for scope in self.scopes.iter().rev() {
            if let Some(var_info) = scope.get(name) {
                return Some(var_info);
            }
        }
        None
    }

    pub fn update_var(
        &mut self,
        name: &str,
        value: Value,
        var_type: Type,
        line_info: Option<LineInfo>, // LineInfoを追加
    ) -> Result<(), EvalError> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(var_info) = scope.get_mut(name) {
                // 変数が可変でない場合、エラーを返す
                if !var_info.is_morph {
                    return Err(EvalError::InvalidOperation(
                        format!("Cannot reassign to immutable variable {}", name),
                        line_info,
                    ));
                }

                // 変数の型が異なる場合、エラーを返す
                if var_info.var_type != var_type {
                    return Err(EvalError::InvalidOperation(
                        format!(
                            "Type mismatch: cannot assign {:?} to variable {} of type {:?}",
                            var_type, name, var_info.var_type
                        ),
                        line_info,
                    ));
                }

                // 書き換えが許可され、型も一致している場合、値を更新
                var_info.value = value;
                return Ok(());
            }
        }
        Err(EvalError::UndefinedVariable(name.to_string(), line_info))
    }
}

#[derive(Debug)]
pub enum Value {
    Omen(bool),
    Arcana(i64),
    Aether(f64),
    Rune(String),
}
