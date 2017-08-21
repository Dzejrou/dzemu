use util;

#[derive(Debug, PartialEq)]
pub enum Token {
    Def,
    Identifier(String),
    Number(u32)
}

#[derive(Debug)]
pub enum TokenRule {
    Def,
    Identifier,
    Number(u8)
}

impl TokenRule {
    pub fn is_valid_char(&self, c: char) -> bool {
        match *self {
            TokenRule::Def           => {
                c == 'd' || c == 'e' || c == 'f'
            }
            TokenRule::Identifier    => {
                c.is_alphanumeric() || c == '_'
            }
            TokenRule::Number(radix) => {
                c.is_digit(radix as u32)
            }
        }
    }

    pub fn is_valid_str(&self, string: &str) -> bool {
        match *self {
            TokenRule::Def        => {
                string.to_lowercase() == "def"
            }
            TokenRule::Identifier => {
                // TODO: Rename that to is_valid_identifier!
                util::is_valid_label(string, false)
            }
            _                     => {
                for c in string.chars() {
                    if !self.is_valid_char(c) {
                        return false;
                    }
                }

                true
            }
        }
    }

    fn parse_validated(&self, token: &str) -> Token {
        match *self {
            TokenRule::Number(radix) => {
                    let res = u32::from_str_radix(token, radix as u32)
                        .unwrap();

                    Token::Number(res)
            }
            TokenRule::Def           => {
                Token::Def
            }
            TokenRule::Identifier    => {
                Token::Identifier(token.to_string())
            }
        }
    }

    pub fn parse(&self, token: &str) -> Option<Token> {
        if self.is_valid_str(token) {
            Some(self.parse_validated(token))
        } else {
            None
        }
    }
}

pub struct Lexer {
    chars: Vec<char>,
    idx:   usize
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
            chars: data,
            idx:   0
        }
    }

    pub fn next_char(&mut self) -> Option<char> {
        if self.idx < self.chars.len() {
            Some(self.chars[self.idx])
        } else {
            None
        }
    }

    pub fn skip(&mut self, count: usize) {
        self.idx = self.idx.wrapping_add(count);
    }

    pub fn skip_if(&mut self, func: &Fn(char) -> bool) {
        if self.idx < self.chars.len() && func(self.chars[self.idx]) {
            self.idx = self.idx.wrapping_add(1);
        }
    }

    pub fn skip_while(&mut self, func: &Fn(char) -> bool) {
        while self.idx < self.chars.len() && func(self.chars[self.idx]) {
            self.idx = self.idx.wrapping_add(1);
        }
    }

    pub fn next_str(&mut self, rule: &TokenRule) -> Option<String> {
        let mut res = String::new();

        while let Some(c) = self.next_char() {
            if rule.is_valid_char(c) {
                res.push(self.chars[self.idx]);
                self.idx += 1;
            } else {
                break;
            }
        }

        if !res.is_empty() {
            Some(res)
        } else {
            None
        }
    }

    pub fn next_token(&mut self, rule: &TokenRule) -> Option<Token> {
        if let Some(token_str) = self.next_str(rule) {
            rule.parse(&token_str)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::Token;
    use super::TokenRule;
    use super::Lexer;

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
    fn next_str_dec() {
        let mut lexer = Lexer::new("1337 A3B0");
        let rule = TokenRule::Number(10);

        assert_eq!(lexer.next_str(&rule), Some("1337".to_string()));
        assert_eq!(lexer.next_str(&rule), None);
    }

    #[test]
    fn next_str_hexa() {
        let mut lexer = Lexer::new("A3B0 hello");
        let rule = TokenRule::Number(16);

        assert_eq!(lexer.next_str(&rule), Some("A3B0".to_string()));
        assert_eq!(lexer.next_str(&rule), None);
    }

    #[test]
    fn next_str_bin() {
        let mut lexer = Lexer::new("10123A");
        let rule = TokenRule::Number(2);

        assert_eq!(lexer.next_str(&rule), Some("101".to_string()));
        assert_eq!(lexer.next_str(&rule), None);
    }

    #[test]
    fn next_token_num() {
        let mut lexer = Lexer::new("  101  1234     ABCD");

        let rule_bin = TokenRule::Number(2);
        assert_eq!(lexer.next_token(&rule_bin), Some(Token::Number(0b101)));
        lexer.skip(1);

        let rule_dec = TokenRule::Number(10);
        assert_eq!(lexer.next_token(&rule_dec), Some(Token::Number(1234)));
        lexer.skip(1);

        let rule_hex = TokenRule::Number(16);
        assert_eq!(lexer.next_token(&rule_hex), Some(Token::Number(0xABCD)));
    }

    #[test]
    fn skip() {
        let mut lexer = Lexer::new("ABCDEFGHIJ");

        lexer.skip(2);
        assert_eq!(lexer.idx, 2);

        lexer.skip_if(&|c| c.is_numeric());
        assert_eq!(lexer.idx, 2);

        lexer.skip_if(&|c| c.is_alphanumeric());
        assert_eq!(lexer.idx, 3);

        lexer.skip_while(&|c| c == 'D' || c == 'E');
        assert_eq!(lexer.idx, 5);
    }

    #[test]
    fn tokens() {
        let mut lexer = Lexer::new("def foo");
        let rule_def = TokenRule::Def;
        let rule_id = TokenRule::Identifier;

        assert_eq!(lexer.next_token(&rule_def), Some(Token::Def));

        lexer.skip(1);
        assert_eq!(lexer.next_token(&rule_id),
                   Some(Token::Identifier("foo".to_string())));

        lexer.skip(1);
        assert_eq!(lexer.next_token(&rule_def), None);
    }
}
