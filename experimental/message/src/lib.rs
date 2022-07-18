pub mod ast;
pub mod parser;

pub struct MessageFormat {}

impl MessageFormat {
    pub fn new() -> Self {
        Self {}
    }

    pub fn format<'s>(&self, _msg: ast::Message<&'s str>) -> String {
        panic!();
    }
}
