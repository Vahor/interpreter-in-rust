use std::fmt::Debug;

pub enum Precedence {
    LOWEST,
    /// ==
    EQUALS,
    /// > or <
    LESSGREATER,
    /// +
    SUM,
    /// *
    PRODUCT,
    /// -X or !X
    PREFIX,
    /// myFunction(X)
    CALL,
    /// array\[index]
    INDEX,
}

impl Precedence {
    pub fn value(&self) -> u8 {
        return match self {
            Precedence::LOWEST => 1,
            Precedence::EQUALS => 2,
            Precedence::LESSGREATER => 3,
            Precedence::SUM => 4,
            Precedence::PRODUCT => 5,
            Precedence::PREFIX => 6,
            Precedence::CALL => 7,
            Precedence::INDEX => 8,
        };
    }
}

impl Debug for Precedence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Precedence::LOWEST => write!(f, "LOWEST ({})", self.value()),
            Precedence::EQUALS => write!(f, "EQUALS ({})", self.value()),
            Precedence::LESSGREATER => write!(f, "LESSGREATER ({})", self.value()),
            Precedence::SUM => write!(f, "SUM ({})", self.value()),
            Precedence::PRODUCT => write!(f, "PRODUCT ({})", self.value()),
            Precedence::PREFIX => write!(f, "PREFIX ({})", self.value()),
            Precedence::CALL => write!(f, "CALL ({})", self.value()),
            Precedence::INDEX => write!(f, "INDEX ({})", self.value()),
        };
    }
}