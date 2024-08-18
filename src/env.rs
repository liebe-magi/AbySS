use crate::ast::{LineInfo, Type};
use crate::eval::EvalError;
use std::collections::HashMap;

pub struct VarInfo {
    pub value: Value,
    pub var_type: Type,
    pub is_morph: bool,
}

pub struct Environment {
    vars: HashMap<String, VarInfo>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            vars: HashMap::new(),
        }
    }

    pub fn set_var(&mut self, name: String, value: Value, var_type: Type, is_morph: bool) {
        self.vars.insert(
            name,
            VarInfo {
                value,
                var_type,
                is_morph,
            },
        );
    }

    pub fn get_var(&self, name: &str) -> Option<&VarInfo> {
        self.vars.get(name)
    }

    pub fn update_var(
        &mut self,
        name: &str,
        value: Value,
        var_type: Type,
        line_info: Option<LineInfo>, // LineInfoを追加
    ) -> Result<(), EvalError> {
        if let Some(var_info) = self.vars.get_mut(name) {
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
            Ok(())
        } else {
            Err(EvalError::UndefinedVariable(name.to_string(), line_info))
        }
    }
}

pub enum Value {
    Omen(bool),
    Arcana(i64),
    Aether(f64),
    Rune(String),
}

#[derive(Debug)]
pub struct SymbolInfo {
    pub var_type: Type,
    pub is_morph: bool,
}

pub type SymbolTable = HashMap<String, SymbolInfo>;
