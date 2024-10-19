//! Build and manage lexical analyzers.
//!
//! The first step of the syntax-parsing pipeline is called _lexical-analysis_. During this phase,
//! The input text is separated into consecutive sequences of characters that have some atomic
//! syntactic meaning, known as [lexemes](Lexeme) (or tokens).
//!
//! Lexemes are usually classified into categories, or _lexeme types_, that identify "groups" of
//! lexemes that have similar syntactic meaning: identifier, integer literal, operator, white
//! space, etc. Each category is specified by a [LexemeDescriptor], which defines the
//! [regex](regex::Regex) pattern that matches lexemes of that type.
//!
//! Finally, the computational unit responsible for extracting the lexemes that a given input text
//! consists of is known as a [lexical analyzer](LexicalAnalyzer), and is compiled from a set of
//! [LexemeDescriptor]s.
//!
//! # Example
//! ```rust
//! # use syntax_parser_generator::lex::*;
//! # use syntax_parser_generator::readers::ByteArrayReader;
//! # use syntax_parser_generator::lex::Regex;
//! # #[derive(Debug, Clone, Eq, Hash, PartialEq)]
//! # enum MyLexemeType { Integer, Addition, NotANumber }
//! let lexical_analyzer = LexicalAnalyzer::new(vec![
//!
//!     // Integer literals
//!     LexemeDescriptor::new(
//!         MyLexemeType::Integer,
//!         Regex::concat(vec![
//!             Regex::optional(
//!                 Regex::union(vec![Regex::single_char('+'), Regex::single_char('-')])
//!             ),
//!             Regex::plus_from(Regex::character_range('0', '9')),
//!         ])
//!     ),
//!
//!     // The addition operator
//!     LexemeDescriptor::special_char(MyLexemeType::Addition, '+'),
//!
//!     // Invalid numbers
//!     LexemeDescriptor::keyword(MyLexemeType::NotANumber, "NaN"),
//! ]);
//!
//! // Use the lexical analyzer to parse structured input text
//! let input_text = &mut ByteArrayReader::from_string(String::from("-2+NaN+-45"));
//! let extracted_lexemes = lexical_analyzer.analyze(input_text);
//!
//! // Validate the parsed output
//! let actual_lexemes = vec![
//!     Lexeme::new(MyLexemeType::Integer, "-2"),
//!     Lexeme::new(MyLexemeType::Addition, "+"),
//!     Lexeme::new(MyLexemeType::NotANumber, "NaN"),
//!     Lexeme::new(MyLexemeType::Addition, "+"),
//!     Lexeme::new(MyLexemeType::Integer, "-45"),
//! ];
//! assert_eq!(extracted_lexemes.collect::<Vec<Lexeme<MyLexemeType>>>(), actual_lexemes);
//! ```

pub use lexeme::{Lexeme, LexemeDescriptor};
pub use lexical_analyzer::LexicalAnalyzer;
pub use regex::Regex;

mod regex;
mod lexeme;
mod lexical_analyzer;
mod lexeme_iterator;

#[cfg(test)]
mod tests;
