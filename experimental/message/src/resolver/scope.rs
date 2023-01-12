use crate::MessageFormat;
use crate::{ast, types::VariableType};
use std::collections::HashMap;

// 'vars - lifetime of variables map
// 'msgs - lifetime of messages map
// 'msgsv - lifetime of message values
// VARSV - variable value type
// MSGSV - messages value type
pub struct Scope<'b, 'mf, 'vars, VARSV> {
    pub mf: &'mf MessageFormat<'b>,
    pub variables: Option<&'vars HashMap<String, VariableType<VARSV>>>,
}

impl<'b, 'mf, 'vars, VARSV> Scope<'b, 'mf, 'vars, VARSV> {
    pub fn new(
        mf: &'mf MessageFormat<'b>,
        variables: Option<&'vars HashMap<String, VariableType<VARSV>>>,
    ) -> Self {
        Self { mf, variables }
    }
}
