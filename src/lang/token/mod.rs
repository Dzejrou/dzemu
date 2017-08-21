pub mod rules;

#[derive(Debug, PartialEq)]
pub enum Token {
    FnDecl,
    Identifier(String),
    UInt(u32)
}

pub trait TokenRule {
    fn push(&mut self, char) -> bool;
    fn valid(&self) -> bool;
    fn get(&self) -> Option<Token>;
    fn clear(&mut self);
}
