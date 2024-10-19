# Syntax Parser Generator

An independent Rust library for generating parsers of syntactically-structured text.

As such, it can generate 2 types of engines - for the 2 phases of syntax parsing, which
naturally fit on top of each other:

* [_Lexical analyzers_](https://en.wikipedia.org/wiki/Lexical_analysis): for tokenizing input
    text by regular expressions.
* [_Syntax-directed translators_](https://en.wikipedia.org/wiki/Syntax-directed_translation):
    for reconstructing the input's syntax-tree by context-free grammars (using the
    [LALR](https://en.wikipedia.org/wiki/LALR_parser) algorithm), and translating it into some
    user-defined representation, such as an abstract syntax-tree (AST) or a sequence of
    intermediate code representation (IR).

Check out the `lex` and `parsing` modules, respectively, for these purposes.

## Motivation

This project was built for fun - to practice Rust, and to test my knowledge in compilation.
Note that the crate is independent: its entire API and logic is designed and implemented
in-house.

Nevertheless, feel free to utilize this project to build your own parsers! You are also 
invited to contribute, hit-me-up if you wish to :)

## Documentation

* In `docs.rs`: https://docs.rs/syntax-parser-generator/latest/
* In `crates.io`: https://crates.io/crates/syntax-parser-generator/

## Example

```rust
enum LexemeType { Plus, Star, Integer }

fn build_lexer() -> LexicalAnalyzer<LexemeType> {
    LexicalAnalyzer::new(vec![
        LexemeDescriptor::special_char(LexemeType::Plus, '+'),
        LexemeDescriptor::special_char(LexemeType::Star, '*'),
        LexemeDescriptor::new(
            LexemeType::Integer,
            Regex::plus_from(Regex::character_range('0', '9')),
        ),
    ])
}

struct ParsingContext {
    integer_count: usize,
    op_count: usize,
}
impl ParsingContext {
    fn new() -> Self {
        Self {
            integer_count: 0,
            op_count: 0,
        }
    }
    fn integer(&mut self, lexeme: String) -> Option<i32> {
        self.integer_count += 1;
        Some(lexeme.parse().ok()?)
    }
    fn sum(&mut self, satellites: Vec<Option<i32>>) -> Option<i32> {
        self.op_count += 1;
        Some(satellites[0]? + satellites[2]?)
    }
    fn mult(&mut self, satellites: Vec<Option<i32>>) -> Option<i32> {
        self.op_count += 1;
        Some(satellites[0]? * satellites[2]?)
    }
}

fn build_parser() -> SyntaxDirectedTranslator<LexemeType, ParsingContext, Option<i32>> {
    let mut builder = SyntaxDirectedTranslatorBuilder::new();

    builder.dub_lexeme_types(vec![
        (LexemeType::Integer, "INTEGER"),
        (LexemeType::Plus, "+"),
        (LexemeType::Star, "*"),
    ].into_iter());
    builder.new_nonterminal("expression");
    builder.set_start_nonterminal("expression");

    builder.new_binding(
        vec!["*"],
        Associativity::Left,
        "multiplicative",
    );
    builder.new_binding(
        vec!["+"],
        Associativity::Left,
        "additive",
    );

    builder.set_leaf_satellite_builder("INTEGER", ParsingContext::integer);
    builder.set_default_leaf_satellite_builder(|_, _| None);

    builder.register_identity_rule("expression", "INTEGER");
    builder.register_bound_rule(
        "expression",
        vec!["expression", "+", "expression"],
        "additive",
        ParsingContext::sum,
    );
    builder.register_bound_rule(
        "expression",
        vec!["expression", "*", "expression"],
        "multiplicative",
        ParsingContext::mult,
    );
    builder.build()
}

fn main() {
    let lexer = build_lexer();
    let parser = build_parser();
    let mut context = ParsingContext::new();

    let mut input = ByteArrayReader::from_string_slice("12+4*5+8");
    assert_eq!(parser.translate(&mut context, lexer.analyze(&mut input)), Some(Some(40)));
    assert_eq!(context.integer_count, 4);
    assert_eq!(context.op_count, 3);
}
```
