use util;
use lang::token::Token;
use lang::token::TokenRule;

const BUFFER_CAPACITY: usize = 20;

pub struct FnDecl {
    buffer: String
}

impl FnDecl {
    pub fn new() -> Box<TokenRule> {
        Box::new(FnDecl {
            buffer: String::with_capacity(BUFFER_CAPACITY)
        })
    }
}

impl TokenRule for FnDecl {
    fn push(&mut self, c: char) -> bool {
        match self.buffer.len() {
            0 => {
                if c == 'd' {
                    self.buffer.push(c);
                    true
                } else {
                    false
                }
            }
            1 => {
                if c == 'e' {
                    self.buffer.push(c);
                    true
                } else {
                    false
                }
            }
            2 => {
                if c == 'f' {
                    self.buffer.push(c);
                    true
                } else {
                    false
                }
            }
            _ => false
        }
    }

    fn valid(&self) -> bool {
        self.buffer == "def"
    }

    fn get(&self) -> Option<Token> {
        if self.valid() {
            Some(Token::FnDecl)
        } else {
            None
        }
    }

    fn clear(&mut self) {
        self.buffer.clear()
    }
}

pub struct Identifier {
    buffer: String
}

impl Identifier {
    pub fn new() -> Box<TokenRule> {
        Box::new(Identifier {
            buffer: String::with_capacity(BUFFER_CAPACITY)
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
