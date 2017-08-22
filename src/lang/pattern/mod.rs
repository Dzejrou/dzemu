use lang::token::Token;

pub mod rules;

pub trait PatternRule {
    // TODO: Return type.
    fn parse(&mut self, &[Token]);
}
