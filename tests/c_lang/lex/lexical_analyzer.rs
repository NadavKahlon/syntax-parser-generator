use syntax_parser_generator::lex::{Lexeme, LexemeDescriptor};
use syntax_parser_generator::lex::lexical_analyzer::LexicalAnalyzer;
use syntax_parser_generator::lex::regex::Regex;
use syntax_parser_generator::reader::Reader;
use crate::c_lang::lex::lexeme_types::CLexemeType;

pub struct CLexicalAnalyzer {
    lexer: LexicalAnalyzer<CLexemeType>,
}

impl CLexicalAnalyzer {
    pub fn analyze<'a>(&'a self, reader: &'a mut impl Reader<u8>)
                       -> impl Iterator<Item=Lexeme<CLexemeType>> + 'a
    {
        self.lexer.analyze(reader).filter(
            |lexeme| lexeme.lexeme_type != CLexemeType::WhiteSpace,
        )
    }

    pub fn new() -> Self {
        Self {
            lexer: LexicalAnalyzer::new(Self::lexeme_descriptors()),
        }
    }

    fn lexeme_descriptors() -> Vec<LexemeDescriptor<CLexemeType>> {
        // Note that the order determines priority in cases of conflict
        vec![
            // Keywords
            LexemeDescriptor::keyword(CLexemeType::If, "if"),
            LexemeDescriptor::keyword(CLexemeType::Else, "else"),
            LexemeDescriptor::keyword(CLexemeType::While, "while"),
            LexemeDescriptor::keyword(CLexemeType::Int, "int"),

            // Primitive expressions
            LexemeDescriptor::new(CLexemeType::Identifier, Self::identifier_regex()),
            LexemeDescriptor::new(CLexemeType::IntLiteral, Self::int_literal_regex()),

            // Operators
            LexemeDescriptor::new(CLexemeType::Assignment, Regex::single_char('=')),

            // Punctuation
            LexemeDescriptor::new(CLexemeType::WhiteSpace, Regex::plus_from(Regex::white_space())),
            LexemeDescriptor::special_char(CLexemeType::LeftParenthesis, '('),
            LexemeDescriptor::special_char(CLexemeType::RightParenthesis, ')'),
            LexemeDescriptor::special_char(CLexemeType::LeftBrace, '{'),
            LexemeDescriptor::special_char(CLexemeType::RightBrace, '}'),
            LexemeDescriptor::special_char(CLexemeType::Semicolon, ';'),
            LexemeDescriptor::special_char(CLexemeType::Comma, ','),
        ]
    }

    fn identifier_regex() -> Regex {
        Regex::concat(vec![
            Regex::union(vec![
                Regex::character_range('a', 'z'),
                Regex::character_range('A', 'Z'),
                Regex::single_char('_'),
            ]),
            Regex::star_from(
                Regex::union(vec![
                    Regex::character_range('a', 'z'),
                    Regex::character_range('A', 'Z'),
                    Regex::character_range('0', '9'),
                    Regex::single_char('_'),
                ]),
            ),
        ])
    }

    fn int_literal_regex() -> Regex {
        Regex::concat(vec![
            Regex::optional(
                Regex::union(vec![
                    Regex::single_char('+'),
                    Regex::single_char('-'),
                ]),
            ),
            Regex::plus_from(Regex::character_range('0', '9')),
        ])
    }
}