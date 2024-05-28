pub enum CommentKind {
    // Line comment looks like this
    Line,

    /* This is a block comment */
    Block,
}

pub enum Arithmetic {
    // +
    Add,

    // -
    Subtract,

    // *
    Multiply,

    // divide(/)
    Divide,
}

pub enum Binary {
    Or,
    And,
    Not,
    Xor,
}

pub enum DataTypes {
    // TODO:IMPLEMENT THIS
}

pub enum Delimeters {
    // (...)
    Paranthesis,

    // {...}
    Braces,

    // [...]
    Bracket,
}
