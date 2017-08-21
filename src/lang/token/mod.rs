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
                util::is_valid_identifier(string, false)
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
