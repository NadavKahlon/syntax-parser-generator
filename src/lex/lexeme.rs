use crate::lex::regex::Regex;

/// Describes a category of lexemes with similar syntactic meanings.
///
/// This is used as part of a lexical-analyzer's specification, as it is built to recognize
/// different types of lexemes.
pub struct LexemeDescriptor<LexemeType> {
    /// The type of lexemes being described.
    pub lexeme_type: LexemeType,

    /// A regular-expression pattern that matches the lexemes of the specified type.
    pub pattern: Regex,
}

impl<LexemeType> LexemeDescriptor<LexemeType> {
    /// Creates a [LexemeDescriptor] describing the specified `lexeme_type` with the specified
    ///`pattern`.
    pub fn new(lexeme_type: LexemeType, pattern: Regex) -> Self {
        LexemeDescriptor {
            lexeme_type,
            pattern,
        }
    }

    /// Creates a new [LexemeDescriptor] that describes a keyword.
    ///
    /// A keyword is a type of lexeme that only matches some hard-coded string (such as `if` or
    /// `int`). This function can be used to efficiently describe such keywords.
    ///
    /// # Example
    /// ```rust
    /// # use syntax_parser_generator::lex::LexemeDescriptor;
    /// enum MyLexemeType { If, While }
    /// let my_lexeme_descriptors = vec![
    ///     LexemeDescriptor::keyword(MyLexemeType::If, "if"),
    ///     LexemeDescriptor::keyword(MyLexemeType::While, "while"),
    /// ];
    /// ```
    pub fn keyword(lexeme_type: LexemeType, name: &str) -> Self {
        Self::new(lexeme_type, Regex::constant_string(name))
    }

    /// Creates a new [LexemeDescriptor] that describes a special character.
    ///
    /// A special character is a type of lexeme that only matches some hard-coded character (such as
    /// operators: `+`, `*`). This function can be used to efficiently describe such characters.
    ///
    /// # Example
    /// ```rust
    /// # use syntax_parser_generator::lex::LexemeDescriptor;
    /// enum MyLexemeType { Addition, Subtraction }
    /// let my_lexeme_descriptors = vec![
    ///     LexemeDescriptor::special_char(MyLexemeType::Addition, '+'),
    ///     LexemeDescriptor::special_char(MyLexemeType::Subtraction, '-'),
    /// ];
    /// ```
    pub fn special_char(lexeme_type: LexemeType, value: char) -> Self {
        Self::new(lexeme_type, Regex::single_char(value))
    }
}

/// A lexeme extracted from input text by a lexical analyzers.
///
/// Lexemes, also known as "tokens", are sequences of consecutive characters separated from
/// input text, and classified into categories (such as keywords, identifiers, operators), during
/// the lexical analysis phase of the syntax-parsing pipeline. They represent atomic units of
/// syntactic meaning.
#[derive(Debug, PartialEq, Eq)]
pub struct Lexeme<LexemeType> {
    /// The type (category) of the lexeme.
    pub lexeme_type: LexemeType,

    /// The original text that constituted the lexeme.
    pub contents: String,
}

impl<LexemeType> Lexeme<LexemeType> {
    /// Creates a new [Lexeme] of the given `lexeme_type` with the given `contents`.
    pub fn new(lexeme_type: LexemeType, contents: &str) -> Self {
        Self {
            lexeme_type,
            contents: String::from(contents),
        }
    }
}
