use util;
use lang::token::Token;
use lang::token::TokenRule;

const BUFFER_CAPACITY: usize = 20;

#[derive(Debug)]
pub struct Keyword {
    buffer:  String,
    keyword: String,
    token:   Token,
    chars:   Vec<char>
}

impl Keyword {
    pub fn new(keyword: String, token: Token) -> Box<TokenRule> {
        let chars: Vec<char> = keyword.chars().collect();

        Box::new(Keyword {
            buffer: String::with_capacity(BUFFER_CAPACITY),
            keyword,
            token,
            chars
        })
    }
}

impl TokenRule for Keyword {
    fn push(&mut self, c: char) -> bool {
        if self.buffer.len() < self.chars.len() {
            if c == self.chars[self.buffer.len()] {
                self.buffer.push(c);
                self.chars.push(c);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn valid(&self) -> bool {
        self.buffer == self.keyword
    }

    fn get(&self) -> Option<Token> {
        if self.valid() {
            Some(self.token.clone())
        } else {
            None
        }
    }

    fn clear(&mut self) {
        self.buffer.clear()
    }
}

#[derive(Debug)]
pub struct Identifier {
    buffer:    String,
    blacklist: Vec<String>
}

impl Identifier {
    pub fn new(blacklist: Vec<String>) -> Box<TokenRule> {
        Box::new(Identifier {
            buffer:    String::with_capacity(BUFFER_CAPACITY),
            blacklist
        })
    }
}

impl TokenRule for Identifier {
    fn push(&mut self, c: char) -> bool {
        if util::is_valid_identifier_char(c) {
            self.buffer.push(c);
            true
        } else {
            false
        }
    }

    fn valid(&self) -> bool {
        !self.blacklist.contains(&self.buffer) &&
        util::is_valid_identifier(&self.buffer, false)
    }

    fn get(&self) -> Option<Token> {
        if self.valid() {
            Some(Token::Identifier(self.buffer.clone()))
        } else {
            None
        }
    }

    fn clear(&mut self) {
        self.buffer.clear()
    }
}

#[derive(Debug)]
pub struct UInt {
    buffer: String,
    radix:  u32
}

impl UInt {
    pub fn new(radix: u32) -> Box<TokenRule> {
        Box::new(UInt {
            buffer: String::with_capacity(BUFFER_CAPACITY),
            radix
        })
    }
}

impl TokenRule for UInt {
    fn push(&mut self, c: char) -> bool {
        if c.is_digit(self.radix) {
            self.buffer.push(c);
            true
        } else {
            false
        }
    }

    fn valid(&self) -> bool {
        if self.buffer != "" {
            for c in self.buffer.chars() {
                if !c.is_digit(self.radix) {
                    return false;
                }
            }

            true
        } else {
            false
        }
    }

    fn get(&self) -> Option<Token> {
        if self.valid() {
            Some(Token::UInt(
                    u32::from_str_radix(&self.buffer, self.radix)
                        .unwrap()
            ))
        } else {
            None
        }
    }

    fn clear(&mut self) {
        self.buffer.clear()
    }
}
