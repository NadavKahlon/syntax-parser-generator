use syntax_parser_generator::handle::auto::AutomaticallyHandled;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CLexemeType {
    If,
    Else,
    While,
    Int,
    Identifier,
    IntLiteral,
    WhiteSpace,
    Assignment,
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,
}

impl AutomaticallyHandled for CLexemeType {
    type HandleCoreType = u16;

    fn serial(&self) -> usize {
        *self as usize
    }
}
