use std::iter::IntoIterator;
use std::path::Path;
use serde_derive::{Deserialize, Serialize};
use compiler_frontend_generator::{Lexeme, LexemeDescriptor, LexicalAnalyzer};
use compiler_frontend_generator::regex::Regex;

#[derive(Serialize, Deserialize)]
enum LexemeType {
    If,
    While,
    Identifier,
    Integer,
    WhiteSpace,
    SemiColon,
}

static LEXEME_DESCRIPTORS: &[LexemeDescriptor<LexemeType>] = &[
    LexemeDescriptor {
        lexeme_type: LexemeType::If,
        pattern: Regex::constant_string("if"),
    },
    LexemeDescriptor {
        lexeme_type: LexemeType::While,
        pattern: Regex::constant_string("while"),
    },
    LexemeDescriptor {
        lexeme_type: LexemeType::Identifier,
        pattern: Regex::concat([
            Regex::union([
                Regex::character_range('a', 'z'),
                Regex::character_range('A', 'Z'),
                Regex::single_char('_'),
            ].into_iter()),
            Regex::star_from(
                Regex::union([
                    Regex::character_range('a', 'z'),
                    Regex::character_range('A', 'Z'),
                    Regex::character_range('0', '9'),
                    Regex::single_char('_'),
                ].into_iter()),
            ),
        ].into_iter()),
    },
    LexemeDescriptor {
        lexeme_type: LexemeType::Integer,
        pattern: Regex::concat([
            Regex::optional(
                Regex::union([
                    Regex::single_char('+'),
                    Regex::single_char('-'),
                ].into_iter()),
            ),
            Regex::plus_from(Regex::digit())
        ].into_iter()),
    },
    LexemeDescriptor {
        lexeme_type: LexemeType::WhiteSpace,
        pattern: Regex::white_space(),
    },
    LexemeDescriptor {
        lexeme_type: LexemeType::SemiColon,
        pattern: Regex::single_char(';'),
    },
];

static SOURCE_PROGRAM_STRING: &str = "if\twhil \n \t\nwhile \t whiley 34\n-1;4 +12";

static ANALYZED_PROGRAM: Vec<Lexeme<LexemeType>> = vec![
    Lexeme {
        lexeme_type: LexemeType::If,
        string: "if".to_string(),
    },
    Lexeme {
        lexeme_type: LexemeType::WhiteSpace,
        string: "\t".to_string(),
    },
    Lexeme {
        lexeme_type: LexemeType::Identifier,
        string: "whil".to_string(),
    },
    Lexeme {
        lexeme_type: LexemeType::WhiteSpace,
        string: " \n \t\n".to_string(),
    },
    Lexeme {
        lexeme_type: LexemeType::While,
        string: "while".to_string(),
    },
    Lexeme {
        lexeme_type: LexemeType::WhiteSpace,
        string: " \t ".to_string(),
    },
    Lexeme {
        lexeme_type: LexemeType::Identifier,
        string: "whiley".to_string(),
    },
    Lexeme {
        lexeme_type: LexemeType::WhiteSpace,
        string: " ".to_string(),
    },
    Lexeme {
        lexeme_type: LexemeType::Integer,
        string: "34".to_string(),
    },
    Lexeme {
        lexeme_type: LexemeType::WhiteSpace,
        string: "\n".to_string(),
    },
    Lexeme {
        lexeme_type: LexemeType::Integer,
        string: "-1".to_string(),
    },
    Lexeme {
        lexeme_type: LexemeType::SemiColon,
        string: ";".to_string(),
    },
    Lexeme {
        lexeme_type: LexemeType::Integer,
        string: "4".to_string(),
    },
    Lexeme {
        lexeme_type: LexemeType::WhiteSpace,
        string: " ".to_string(),
    },
    Lexeme {
        lexeme_type: LexemeType::Integer,
        string: "+12".to_string(),
    },
];

fn compile_lexical_analyzer() -> LexicalAnalyzer<LexemeType> {
    LexicalAnalyzer::new(&LEXEME_DESCRIPTORS)
}

fn check_lexical_analyzer_on_string(lexical_analyzer: LexicalAnalyzer<LexemeType>) {
    let lexemes: Vec<Lexeme<LexemeType>> =
        lexical_analyzer
            .analyze_string(SOURCE_PROGRAM_STRING)
            .collect();
    assert_eq!(lexemes, ANALYZED_PROGRAM)
}

#[test]
fn test_lexical_analyzer_compilation() {
    compile_lexical_analyzer();
}

#[test]
fn test_lexical_analyzer_on_string() {
    let lexical_analyzer = compile_lexical_analyzer();
    check_lexical_analyzer_on_string(lexical_analyzer)
}

#[test]
fn test_lexical_analyzer_serialization() {
    let original_lexical_analyzer = compile_lexical_analyzer();
    let filepath =
        Path::new(&std::env::var("CARGO_MANIFEST_DIR"))
            .unwrap()
            .join("test_artifacts")
            .join("c_lexical_analyzer.json");
    original_lexical_analyzer.save_to_json_file(filepath);
    let loaded_lexical_analyzer: LexicalAnalyzer<LexemeType> =
        LexicalAnalyzer::load_from_json_file(filepath);
    check_lexical_analyzer_on_string(loaded_lexical_analyzer)
}