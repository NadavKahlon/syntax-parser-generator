use crate::lex::Lexeme;

use crate::lex::LexemeDescriptor;
use crate::lex::lexical_analyzer::LexicalAnalyzer;
use crate::readers::ByteArrayReader;
use crate::regex::Regex;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum TestLexemeType {
    If,
    While,
    Identifier,
    Integer,
    WhiteSpace,
    SemiColon,
}

fn lexeme_descriptors() -> Vec<LexemeDescriptor<TestLexemeType>> {
    vec![
        LexemeDescriptor::keyword(TestLexemeType::If, "if"),
        LexemeDescriptor::keyword(TestLexemeType::While, "while"),
        LexemeDescriptor::new(
            TestLexemeType::Identifier,
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
            ]),
        ),
        LexemeDescriptor::new(
            TestLexemeType::Integer,
            Regex::concat(vec![
                Regex::optional(
                    Regex::union(vec![
                        Regex::single_char('+'),
                        Regex::single_char('-'),
                    ]),
                ),
                Regex::plus_from(Regex::character_range('0', '9')),
            ]),
        ),
        LexemeDescriptor::new(
            TestLexemeType::WhiteSpace,
            Regex::plus_from(Regex::white_space()),
        ),
        LexemeDescriptor::new(
            TestLexemeType::SemiColon,
            Regex::single_char(';'),
        ),
    ]
}

fn source_program_string() -> &'static str {
    "if\twhil \n \t\nwhile \t whiley 34\n-1;4 +12"
}

fn analyzed_program() -> Vec<Lexeme<TestLexemeType>> {
    vec![
        Lexeme {
            lexeme_type: TestLexemeType::If,
            contents: "if".to_string(),
        },
        Lexeme {
            lexeme_type: TestLexemeType::WhiteSpace,
            contents: "\t".to_string(),
        },
        Lexeme {
            lexeme_type: TestLexemeType::Identifier,
            contents: "whil".to_string(),
        },
        Lexeme {
            lexeme_type: TestLexemeType::WhiteSpace,
            contents: " \n \t\n".to_string(),
        },
        Lexeme {
            lexeme_type: TestLexemeType::While,
            contents: "while".to_string(),
        },
        Lexeme {
            lexeme_type: TestLexemeType::WhiteSpace,
            contents: " \t ".to_string(),
        },
        Lexeme {
            lexeme_type: TestLexemeType::Identifier,
            contents: "whiley".to_string(),
        },
        Lexeme {
            lexeme_type: TestLexemeType::WhiteSpace,
            contents: " ".to_string(),
        },
        Lexeme {
            lexeme_type: TestLexemeType::Integer,
            contents: "34".to_string(),
        },
        Lexeme {
            lexeme_type: TestLexemeType::WhiteSpace,
            contents: "\n".to_string(),
        },
        Lexeme {
            lexeme_type: TestLexemeType::Integer,
            contents: "-1".to_string(),
        },
        Lexeme {
            lexeme_type: TestLexemeType::SemiColon,
            contents: ";".to_string(),
        },
        Lexeme {
            lexeme_type: TestLexemeType::Integer,
            contents: "4".to_string(),
        },
        Lexeme {
            lexeme_type: TestLexemeType::WhiteSpace,
            contents: " ".to_string(),
        },
        Lexeme {
            lexeme_type: TestLexemeType::Integer,
            contents: "+12".to_string(),
        },
    ]
}

#[test]
fn test_lexical_analyzer_on_string() {
    let lexical_analyzer = LexicalAnalyzer::new(lexeme_descriptors());
    let lexemes: Vec<Lexeme<TestLexemeType>> =
        lexical_analyzer
            .analyze(&mut ByteArrayReader::from_string(source_program_string().to_string()))
            .collect();
    assert_eq!(lexemes, analyzed_program())
}

#[test]
#[should_panic]
fn test_lexical_error() {
    let lexical_analyzer = LexicalAnalyzer::new(vec![
        LexemeDescriptor::new((), Regex::single_char('+'))
    ]);
    let _ = lexical_analyzer.analyze(&mut ByteArrayReader::from_string("++-+".to_string()))
        .collect::<Vec<Lexeme<()>>>();
}
