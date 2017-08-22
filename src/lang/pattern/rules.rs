use lang::pattern::PatternRule;
use lang::token::Token;

pub struct ReturnStatement;

impl ReturnStatement {
    pub fn new() -> Box<PatternRule> {
        Box::new(ReturnStatement {
        
        })
    }
}

impl PatternRule for ReturnStatement {
    fn parse(&mut self, tokens: &[Token]) {
        if tokens.len() > 0 && tokens[0] == Token::Return &&
           tokens[tokens.len() - 1] == Token::NewLine {
            println!("RETURN STATEMENT: {:?}", tokens);
            panic!("Matched return statement.");
        } else {
            println!("NOT RETURN STATEMENT: {:?}", tokens);
        }
    }
}
