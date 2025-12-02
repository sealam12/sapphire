#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace, LeftBracket, RightBracket,
    Comma, Dot, DoubleDot, Minus, Plus, Semicolon, Slash, Star,
    QuestionMark, Colon, Pipe, DoublePipe, Amp, DoubleAmp,

    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals.
    Identifier, String, Number,

    // Keywords.
    And, Class, Else, False, Fn, For, If, In, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    EOF
}