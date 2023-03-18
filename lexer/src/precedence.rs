use std::fmt::{Debug, Display};

pub enum Precedence {
    LOWEST,
    EQUALS,
    // ==
    LESSGREATER,
    // > or <
    SUM,
    // +
    PRODUCT,
    // *
    PREFIX,
    // -X or !X
    CALL, // myFunction(X)
}

impl Precedence {
    pub fn get_precedence(&self) -> u8 {
        return match self {
            Precedence::LOWEST => 1,
            Precedence::EQUALS => 2,
            Precedence::LESSGREATER => 3,
            Precedence::SUM => 4,
            Precedence::PRODUCT => 5,
            Precedence::PREFIX => 6,
            Precedence::CALL => 7,
        };
    }
}

impl Debug for Precedence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Precedence::LOWEST => write!(f, "LOWEST ({})", self.get_precedence()),
            Precedence::EQUALS => write!(f, "EQUALS ({})", self.get_precedence()),
            Precedence::LESSGREATER => write!(f, "LESSGREATER ({})", self.get_precedence()),
            Precedence::SUM => write!(f, "SUM ({})", self.get_precedence()),
            Precedence::PRODUCT => write!(f, "PRODUCT ({})", self.get_precedence()),
            Precedence::PREFIX => write!(f, "PREFIX ({})", self.get_precedence()),
            Precedence::CALL => write!(f, "CALL ({})", self.get_precedence()),
        };
    }
}