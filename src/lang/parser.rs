use std::collections::VecDeque;

use lang::token::Token;
use lang::pattern::PatternRule;
use lang::pattern::rules::*;

struct Parser {
    tokens: VecDeque<Token>,
    rules:  Vec<Box<PatternRule>>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Box<Parser> {
        let mut deque: VecDeque<Token> = VecDeque::new();
        for token in tokens.iter() {
            deque.push_back(token.clone());
        }

        Box::new(Parser {
            tokens: deque,
            rules:  Vec::new()
        })
    }

    pub fn add_rule(&mut self, rule: Box<PatternRule>) {
        self.rules.push(rule);
    }

    pub fn parse(&mut self) {
        // TODO: Downwards or upwards traversal?
        let mut current: Vec<Token> = Vec::new();

        while !self.tokens.is_empty() {
            current.push(self.tokens.pop_front().unwrap());

            for rule in self.rules.iter_mut() {
                rule.parse(&current);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use lang::token::Token;
    use lang::pattern::PatternRule;
    use lang::pattern::rules::*;
    use lang::parser::Parser;

    #[test]
    fn parse_return() {
        let tokens = vec![Token::Return, Token::UInt(1), Token::NewLine];
        let mut parser = Parser::new(tokens);
        parser.add_rule(ReturnStatement::new());
        parser.parse();

        assert!(false);
    }
}
