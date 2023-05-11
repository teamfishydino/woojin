use std::{
  sync::{Arc, Mutex}, collections::HashMap
};
use crate::{types::WoojinValue, error::{WoojinError, WoojinErrorKind}, ast::Statements, exec};
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub(crate) struct VariableOption {
  pub is_mut: bool,
}

impl VariableOption {
  pub(crate) fn new(is_mut: Option<bool>, _is_const: Option<bool>) -> VariableOption {
    VariableOption {
      is_mut: is_mut.unwrap_or(false),
    }
  }
}

#[derive(Debug, Clone)]
pub(crate) struct WoojinVariable {
  pub value: WoojinValue,
  pub is_mut: bool
}
lazy_static! {
  pub(crate) static ref VARS: Arc<Mutex<HashMap<String, WoojinVariable>>> = {
    Arc::new(Mutex::new(HashMap::new()))
  };
}

pub(crate) fn get_var(name: &str) -> WoojinVariable {
  let vars = VARS.lock().unwrap();
  if !vars.contains_key(name) {
    WoojinError::new(format!("Variable {} is not declared", name), WoojinErrorKind::UndeclaredVariable).exit();
  }
  vars.get(name).unwrap().clone()
}

pub(crate) fn change_var(name: &str, stmt: &Statements) -> Result<(), WoojinError> {
  let var = get_var(name);
  if !var.is_mut { WoojinError::new(format!("Variable {} is not mutable", name), WoojinErrorKind::VariableNotMutable).exit(); }
  let mut vars = VARS.lock().unwrap();
  vars.insert(name.to_string(), WoojinVariable { value: exec(stmt)?.clone(), is_mut: var.is_mut });
  Ok(())
}

pub(crate) fn dec_var(name: &str, value: &WoojinValue, option: &VariableOption) -> Result<(), WoojinError> {
  let mut vars = VARS.lock().unwrap();
  if vars.contains_key(name) {
    WoojinError::new(format!("Variable {} is already declared", name), WoojinErrorKind::VariableAlreadyDeclared).exit();
  }
  vars.insert(name.to_string(), WoojinVariable {
    value: value.clone(),
    is_mut: option.is_mut
  });
  Ok(())
}

