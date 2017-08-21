pub enum Token {
    Number(u32)
}

pub enum TokenRule {
    Number(u8)
}

impl TokenRule {
    pub fn is_valid_char(&self, c: char) -> bool {
        match *self {
            TokenRule::Number(base) => c.is_digit(base as u32)
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
            TokenRule::Number(_) => {
                if !self.is_valid_str(token_str) {
                    None
                } else {
                    let res = token_str.parse()
                        .expect(&format!("Not a number: {}", token_str));

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
        let data: Vec<char> = data.chars().collect();

        Lexer {
            chars: data,
            idx:   0
        }
    }

    fn next_char(&mut self) -> Option<char> {
        if self.idx < self.chars.len() {
            Some(self.chars[self.idx])
        } else {
            None
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
    use super::TokenRule;
    use super::Lexer;

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
}
