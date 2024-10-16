mod lexeme;
pub use lexeme::{Lexeme, LexemeDescriptor};

mod lexical_analyzer;
pub use lexical_analyzer::LexicalAnalyzer;

mod lexeme_iterator;

#[cfg(test)]
mod tests;
