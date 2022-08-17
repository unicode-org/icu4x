use crate::{ast, types::VariableType};
use std::collections::HashMap;

// 'vars - lifetime of variables map
// 'msgs - lifetime of messages map
// 'msgsv - lifetime of message values
// VARSV - variable value type
// MSGSV - messages value type
pub struct Scope<'vars, 'msgs, 'msgsv, VARSV, MSGSV> {
    pub variables: Option<&'vars HashMap<String, VariableType<VARSV>>>,
    pub messages: Option<&'msgs HashMap<String, &'msgsv ast::Message<MSGSV>>>,
}

impl<'vars, 'msgs, 'msgsv, VARSV, MSGSV> Scope<'vars, 'msgs, 'msgsv, VARSV, MSGSV> {
    pub fn new(
        variables: Option<&'vars HashMap<String, VariableType<VARSV>>>,
        messages: Option<&'msgs HashMap<String, &'msgsv ast::Message<MSGSV>>>,
    ) -> Self {
        Self {
            variables,
            messages,
        }
    }
}
