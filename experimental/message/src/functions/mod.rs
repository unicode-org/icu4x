use crate::types::VariableType;

pub struct Number;

impl Number {
    pub fn format(input: &VariableType<String>) -> VariableType<String> {
        VariableType::String("Hello from function".to_string())
    }
}
