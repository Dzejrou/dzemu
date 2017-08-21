#[derive(Debug, PartialEq)]
pub enum Token {
    Number(u32)
}

#[derive(Debug)]
pub enum TokenRule {
    Number(u8)
}

impl TokenRule {
    pub fn is_valid_char(&self, c: char) -> bool {
        match *self {
            TokenRule::Number(radix) => c.is_digit(radix as u32)
        }
    }

    pub fn is_valid_str(&self, string: &str) -> bool {
        for c in string.chars() {
            if !self.is_valid_char(c) {
                return false;
            }
        }

        true
    }

    pub fn parse(&self, token_str: &str) -> Option<Token> {
        match *self {
            TokenRule::Number(radix) => {
                if !self.is_valid_str(token_str) {
                    None
                } else {
                    let res = u32::from_str_radix(token_str, radix as u32)
                        .unwrap();

                    Some(Token::Number(res))
                }
            }
        }
    }
}

pub struct Lexer {
    chars: Vec<char>,
    idx:   usize
}

impl Lexer {
    pub fn new(data: &str) -> Lexer {
        // TODO: Is whitespace needed?
        let data: Vec<char> = data.chars()
            .filter(|c| !c.is_whitespace()).collect();

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
        let mut lexer = Lexer::new("0101 1234 ABCD");

        lexer.skip(4);
        // let rule_bin = TokenRule::Number(2);
        // assert_eq!(lexer.next_token(&rule_bin), Some(Token::Number(0b101)));

        let rule_dec = TokenRule::Number(10);
        assert_eq!(lexer.next_token(&rule_dec), Some(Token::Number(1234)));

        let rule_hex = TokenRule::Number(16);
        assert_eq!(lexer.next_token(&rule_hex), Some(Token::Number(0xABCD)));
    }
}
