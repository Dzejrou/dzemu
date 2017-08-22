pub mod rules;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Dot,
    End,
    For,
    FnDecl,
    Identifier(String),
    LParen,
    NewLine,
    OpPlus,
    RParen,
    Return,
    While,
    UInt(u32)
}

pub trait TokenRule {
    fn push(&mut self, char) -> bool;
    fn valid(&self) -> bool;
    fn get(&self) -> Option<Token>;
    fn clear(&mut self);
}
