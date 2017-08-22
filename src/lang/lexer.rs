use lang::token::Token;
use lang::token::TokenRule;
use lang::token::rules::*;

pub struct Lexer {
    chars:  Vec<char>,
    rules:  Vec<Box<TokenRule>>,
    tokens: Vec<Token>,
    idx:    usize
}

impl Lexer {
    pub fn new(data: &str) -> Lexer {
        let data: Vec<char> = data
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .chars()
            .collect();

        Lexer {
            chars:  data,
            rules:  Vec::new(),
            tokens: Vec::new(),
            idx:    0
        }
    }

    pub fn add_rule(&mut self, rule: Box<TokenRule>) -> &mut Lexer{
        self.rules.push(rule);
        self
    }

    pub fn add_rules(&mut self, rules: &mut Vec<Box<TokenRule>>) {
        self.rules.append(rules);
    }

    pub fn add_str(&mut self, string: &str) {
        self.chars.append(&mut string.chars().collect());
    }

    pub fn tokenize(&mut self) {
        let len = self.chars.len();
        let mut token_found = false;

        while self.idx < len {
            let c = self.chars[self.idx];
            for rule in self.rules.iter_mut() {
                if !rule.push(c) {
                    // First invalid char.
                    if let Some(token) = rule.get() {
                        token_found = true;
                        self.tokens.push(token);
                        break;
                    } else {
                        rule.clear();
                    }
                }
            }

            if token_found {
                for rule in self.rules.iter_mut() {
                    rule.clear();
                }

                //self.skip_while(&char::is_whitespace);
                token_found = false;
            } else {
                self.idx = self.idx.wrapping_add(1);
            }
        }

        // In case of leftovers.
        for rule in self.rules.iter_mut() {
            if rule.valid() {
                self.tokens.push(rule.get().unwrap());
            }
        }
    }

    pub fn next_char(&mut self) -> Option<char> {
        if self.valid_idx() {
            Some(self.chars[self.idx])
        } else {
            None
        }
    }

    fn valid_idx(&self) -> bool {
        self.idx < self.chars.len()
    }

    pub fn skip(&mut self, count: usize) {
        self.idx = self.idx.wrapping_add(count);
    }

    pub fn skip_if(&mut self, func: &Fn(char) -> bool) {
        if self.valid_idx() && func(self.chars[self.idx]) {
            self.idx = self.idx.wrapping_add(1);
        }
    }

    pub fn skip_while(&mut self, func: &Fn(char) -> bool) {
        while self.valid_idx() && func(self.chars[self.idx]) {
            self.idx = self.idx.wrapping_add(1);
        }
    }

    pub fn test(&self, func: &Fn(char) -> bool) -> bool {
        if self.valid_idx() {
            func(self.chars[self.idx])
        } else {
            false
        }
    }

    pub fn test_eq(&self, c: char) -> bool {
        if self.valid_idx() {
            self.chars[self.idx] == c
        } else {
            false
        }
    }

    pub fn peek(&self, off: usize) -> Option<char> {
        let idx = self.idx.wrapping_add(off);

        if idx < self.chars.len() {
            Some(self.chars[idx])
        } else {
            None
        }
    }

    pub fn next(&mut self, mut rule: Box<TokenRule>) -> Option<Token> {
        let idx_bck = self.idx;
        while let Some(c) = self.next_char() {
            if rule.push(c) {
                self.idx += 1;
            } else {
                break;
            }
        }

        if rule.valid() {
            rule.get()
        } else {
            self.idx = idx_bck;
            None
        }
    }
}

#[cfg(test)]
mod test {
    use lang::token::Token;
    use lang::token::TokenRule;
    use lang::token::rules::*;
    use lang::lexer::Lexer;

    #[test]
    fn next_char() {
        let mut lexer = Lexer::new("ACBD");

        assert_eq!(lexer.next_char(), Some('A'));

        lexer.skip(1);
        assert_eq!(lexer.next_char(), Some('C'));

        lexer.skip(2);
        assert_eq!(lexer.next_char(), Some('D'));

        lexer.skip(1);
        assert_eq!(lexer.next_char(), None);
    }

    #[test]
    fn next_uint() {
        let mut lexer = Lexer::new("  101  1234     ABCD");

        let rule_bin = UInt::new(2);
        assert_eq!(lexer.next(rule_bin), Some(Token::UInt(0b101)));
        lexer.skip(1);

        let rule_dec = UInt::new(10);
        assert_eq!(lexer.next(rule_dec), Some(Token::UInt(1234)));
        lexer.skip(1);

        let rule_hex = UInt::new(16);
        assert_eq!(lexer.next(rule_hex), Some(Token::UInt(0xABCD)));
    }

    #[test]
    fn skip() {
        let mut lexer = Lexer::new("ABCDEF");

        lexer.skip(2);
        assert_eq!(lexer.idx, 2);

        lexer.skip_if(&|c| c.is_numeric());
        assert_eq!(lexer.idx, 2);

        lexer.skip_if(&|c| c.is_alphanumeric());
        assert_eq!(lexer.idx, 3);

        lexer.skip_while(&|c| c == 'D' || c == 'E');
        assert_eq!(lexer.idx, 5);

        lexer.skip(1);
        assert_eq!(lexer.idx, 6);
    }

    #[test]
    fn test() {
        let lexer = Lexer::new("A");

        assert!(lexer.test(&|c| c == 'A'));
        assert!(!lexer.test(&|_| false));
        assert!(lexer.test_eq('A'))
    }

    #[test]
    fn peek() {
        let lexer = Lexer::new("ABCD");

        assert_eq!(lexer.peek(3), Some('D'));
        assert_eq!(lexer.peek(4), None);
    }

    #[test]
    fn tokens() {
        let mut lexer = Lexer::new("def foo");
        let rule_fn = Keyword::new("def", Token::FnDecl);
        let rule_id = Identifier::new(vec!["def".to_string()]);

        assert_eq!(lexer.next(rule_fn), Some(Token::FnDecl));

        lexer.skip(1);
        assert_eq!(lexer.next(rule_id),
                   Some(Token::Identifier("foo".to_string())));

        lexer.skip(1);
        let rule_fn = Keyword::new("def", Token::FnDecl);
        assert_eq!(lexer.next(rule_fn), None);
    }

    #[test]
    fn tokenize() {
        let mut lexer = Lexer::new("");
        lexer.add_str("def foo(parameter)\n");
        lexer.add_str("    return parameter + 3\n");
        lexer.add_str("end\n");

        let keywords: Vec<String> = vec![
            "def".to_string(), "return".to_string(),
            "end".to_string()
        ];

        lexer.add_rule(Identifier::new(keywords))
            .add_rule(Keyword::new("def", Token::FnDecl))
            .add_rule(Keyword::new("return", Token::Return))
            .add_rule(Keyword::new("end", Token::End))
            .add_rule(Keyword::new("(", Token::LParen))
            .add_rule(Keyword::new(")", Token::RParen))
            .add_rule(Keyword::new("+", Token::OpPlus))
            .add_rule(Keyword::new("\n", Token::NewLine))
            .add_rule(UInt::new(10));

        // TODO: This is a problem, picks up ff etc.
        //       Maybe add a prefix for it and make it HexUInt?
        // .add_rule(UInt::new(16));
        lexer.tokenize();

        let foo = "foo".to_string();
        let par = "parameter".to_string();

        println!("");
        println!("{:?}", lexer.tokens);
        let target = vec![
            Token::FnDecl, Token::Identifier(foo.clone()),
            Token::LParen, Token::Identifier(par.clone()),
            Token::RParen, Token::NewLine, Token::Return,
            Token::Identifier(par.clone()),Token::OpPlus,
            Token::UInt(3), Token::NewLine, Token::End,
            Token::NewLine
        ];
        println!("{:?}", target);
        println!("");

        assert_eq!(lexer.tokens, target);
    }
}
