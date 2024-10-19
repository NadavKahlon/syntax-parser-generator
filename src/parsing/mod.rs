//! Build and manage syntax-directed translators based on LALR parsers.
//!
//! The second step of the syntax-parsing pipeline is called _syntax directed translation (SDT)_,
//! or _parsing_. This phase is responsible for reconstructing the hierarchical structure of the
//! lexemes collected in the lexical analysis phase - also known as the input's _syntax tree_,
//! according to a set of rules specified by a _context free grammar (CFG)_.
//!
//! # LALR Parsing
//!
//! This module provides a convenient interface for building and executing such parsers on streams
//! on lexemes, based on the bottom-up [LALR](https://en.wikipedia.org/wiki/LALR_parser) parsing
//! algorithm. Lexeme types  (e.g. keyword, identifier, integer literal) serve as the parser's
//! "terminal symbols". The contents of the lexemes is only used for translation (see the next
//! section), and is ignored by the LALR algorithm.
//!
//! # Translation
//!
//! As it is being reconstructed, the syntax tree is translated bottom-up into a more meaningful
//! representation of the input, which we call _satellite data_ (as it escorts subtrees of the
//! syntax tree). Client code is free to define the translation scheme and the target representation
//! (the type of satellite data), which is usually an abstract syntax tree (AST), or an intermediate
//! code representation (IR).
//!
//! To aid the translation process, a mutable _translation context_ can be managed by the parser.
//! This is a user-defined object associated with each execution of the parser, which is
//! consistently passed to client code responsible for translation of the input syntax tree. This
//! can be used to manage global knowledge about the parsed input, such as symbol tables or general
//! statistics.
//!
//! # See Also
//!
//! [SyntaxDirectedTranslatorBuilder], for more details on the specifications of such translation
//! engines.
//!
//! # Example
//!
//! ```rust
//! # use syntax_parser_generator::handles::specials::AutomaticallyHandled;
//! # use syntax_parser_generator::lex::Lexeme;
//! # use syntax_parser_generator::parsing::{Associativity, SyntaxDirectedTranslator, SyntaxDirectedTranslatorBuilder};
//! # #[derive(Debug, Clone, Copy)]
//! enum LexemeType { Integer, Plus, Star }
//! # impl AutomaticallyHandled for LexemeType {
//! #    type HandleCoreType = u8;
//! #    fn serial(&self) -> usize { *self as usize }
//! # }
//! #
//!
//! struct Context {
//!     integer_count: usize,
//!     op_count: usize,
//! }
//!
//! impl Context {
//!     fn new() -> Self {
//!         Self {
//!             integer_count: 0,
//!             op_count: 0,
//!         }
//!     }
//!     fn integer(&mut self, lexeme: String) -> Option<i32> {
//!         self.integer_count += 1;
//!         Some(lexeme.parse().ok()?)
//!     }
//!     fn sum(&mut self, mut satellites: Vec<Option<i32>>) -> Option<i32> {
//!         self.op_count += 1;
//!         Some(satellites[0]? + satellites[2]?)
//!     }
//!
//!     fn mult(&mut self, mut satellites: Vec<Option<i32>>) -> Option<i32> {
//!         self.op_count += 1;
//!         Some(satellites[0]? * satellites[2]?)
//!     }
//! }
//!
//! fn build_calculator() -> SyntaxDirectedTranslator<LexemeType, Context, Option<i32>> {
//!     let mut builder = SyntaxDirectedTranslatorBuilder::new();
//!
//!     builder.dub_lexeme_types(vec![
//!         (LexemeType::Integer, "INTEGER"),
//!         (LexemeType::Plus, "+"),
//!         (LexemeType::Star, "*"),
//!     ].into_iter());
//!     builder.new_nonterminal("expression");
//!     builder.set_start_nonterminal("expression");
//!
//!     builder.set_leaf_satellite_builder("INTEGER", Context::integer);
//!     builder.set_default_leaf_satellite_builder(|_, _| None);
//!
//!     builder.new_binding(
//!         vec!["*"],
//!         Associativity::Left,
//!         "multiplicative"
//!     );
//!     builder.new_binding(
//!         vec!["+"],
//!         Associativity::Left,
//!         "additive",
//!     );
//!
//!     builder.register_identity_rule("expression", "INTEGER");
//!     builder.register_bound_rule(
//!         "expression",
//!         vec!["expression", "+", "expression"],
//!         "additive",
//!         Context::sum,
//!     );
//!     builder.register_bound_rule(
//!         "expression",
//!         vec!["expression", "*", "expression"],
//!         "multiplicative",
//!         Context::mult,
//!     );
//!     builder.build()
//! }
//!
//! fn main () {
//!     let parser = build_calculator();
//!     let mut context = Context::new();
//!     let lexemes = vec![
//!         Lexeme::new(LexemeType::Integer, "-2"),
//!         Lexeme::new(LexemeType::Plus, "+"),
//!         Lexeme::new(LexemeType::Integer, "4"),
//!         Lexeme::new(LexemeType::Star, "*"),
//!         Lexeme::new(LexemeType::Integer, "3"),
//!     ];
//!     assert_eq!(parser.translate(&mut context, lexemes.into_iter()), Some(Some(10)));
//!     assert_eq!(context.integer_count, 3);
//!     assert_eq!(context.op_count, 2);
//! }
//! ```

pub use translator::build::SyntaxDirectedTranslatorBuilder;
pub use translator::sdt::SyntaxDirectedTranslator;

mod lr_parser;
pub use lr_parser::rules::Associativity;

mod translator;
