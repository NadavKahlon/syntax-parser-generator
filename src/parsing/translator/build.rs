use std::collections::HashMap;
use crate::handles::{Handle, Handled};
use crate::handles::collections::{HandledVec, HandleMap};
use crate::handles::specials::AutomaticallyHandled;
use crate::parsing::lr_parser::build::LrParserBuilder;
use crate::parsing::lr_parser::rules::{Associativity, Binding, GrammarSymbol};
use crate::parsing::translator::handlers::{LeafSatelliteBuilder, SatelliteReducer};
use crate::parsing::translator::sdt::SyntaxDirectedTranslator;

/// An interface for specifying and compiling a [SyntaxDirectedTranslator].
///
/// An instance of this type serve as a "contractor" for building a LALR parser. It exports an API
/// to communicate the specifications of the parser, and when you're done - its
/// [build](SyntaxDirectedTranslatorBuilder::build) can be used to compile a matching instance of
/// [SyntaxDirectedTranslator].
///
/// # Features
///
/// ## Bindings
///
/// A group of terminal symbols (i.e. lexeme types) can be associated with a _binding_, which can
/// then be attached to the grammar's production rules. Bindings are used to resolve shift-reduce
/// conflicts: when deciding between shifting by a bound terminal or reducing by a bound rule,
/// the one with higher priority binding is selected. If both have the same binding - we reduce if
/// it is left-associated, and shift if it is right-associated.
///
/// This can be used to implement associativity and precedence of operators. For example - one
/// left-associated higher-priority binding for multiplication and division, and another
/// left-associated lower-priority binding for addition and subtraction.
///
/// ## Dubs
///
/// To make the parser's programmatic specification cleaner, the notion of _dubs_ was introduced:
/// instead of referring to a terminal, nonterminal, or a binding, by some complex Rust object,
/// these are referred to by their "dubs" - character strings that identify them.
///
/// ## Translation Scheme Specifications
///
/// The compiled parser will be capable of translating a given sequence of lexemes into some
/// higher-level representation (AST, IR sequence, etc.). We refer to this higher level
/// representation as _satellite data_ (as it escorts subtrees of the input's syntax tree). Client
/// code is free to determine this target representation and the translation logic.
///
/// This translation logic is carried out recursively: a subtree of the input syntax tree is
/// translated into satellite data as a function of the translations of its own subtrees. We call
/// such translation functions _satellite reducers_, as they execute after each reduction of
/// grammar symbols by the underlying LR parser. At the leaves, lexemes are directly translated to
/// satellite data using dedicated functions we call _leaf satellite builders_.
///
/// All of these translation functions receive a mutable reference to the _translation context_ as
/// an additional argument, which is a user-defined object that's used to hold global knowledge
/// about the translated input, such as symbol tables and general statistics.
///
/// All of this translation logic is of course user-defined, providing client code with great
/// flexibility when building custom syntax-directed translators.
///
/// # Example
///
/// Check out the example at [parsing](crate::parsing).
///
/// # Conflict Resolution Policy
///
/// * Reduce-reduce conflicts cannot be resolved.
/// * Shift-reduce conflicts are resolved with "shift" by default, unless otherwise specified by
///     bindings.
pub struct SyntaxDirectedTranslatorBuilder<LexemeType, Context, Satellite>
where
    LexemeType: AutomaticallyHandled,
{
    grammar_symbol_dub_map: HashMap<String, GrammarSymbol<LexemeType, Nonterminal>>,
    nonterminals: HandledVec<Nonterminal>,
    lr_parser_builder: LrParserBuilder<LexemeType, Nonterminal, SatelliteReducer<Context, Satellite>>,
    bindings_dub_map: HashMap<String, Handle<Binding<LexemeType>>>,
    leaf_satellite_builder_map: HandleMap<LexemeType, LeafSatelliteBuilder<Context, Satellite>>,
    default_leaf_satellite_builder: Option<LeafSatelliteBuilder<Context, Satellite>>,
    satellite_reducers: HandledVec<SatelliteReducer<Context, Satellite>>,
}

impl<LexemeType, Context, Satellite> SyntaxDirectedTranslatorBuilder<LexemeType, Context, Satellite>
where
    LexemeType: AutomaticallyHandled,
{
    /// Create a blank [SyntaxDirectedTranslatorBuilder], with no registered specifications.
    pub fn new() -> Self {
        Self {
            grammar_symbol_dub_map: HashMap::new(),
            nonterminals: HandledVec::new(),
            lr_parser_builder: LrParserBuilder::new(),
            bindings_dub_map: HashMap::new(),
            leaf_satellite_builder_map: HandleMap::new(),
            default_leaf_satellite_builder: None,
            satellite_reducers: HandledVec::new(),
        }
    }

    /// Dub with a category of lexemes.
    ///
    /// Remember: lexeme types serve as terminal symbols for the underlying LALR parser. This
    /// function, in effect, dubs a terminal symbol in the grammar.
    ///
    /// # Panics
    ///
    /// If some other grammar symbol is already identified by this dub.
    pub fn dub_lexeme_type(&mut self, lexeme_type: LexemeType, dub: &str) {
        if self.grammar_symbol_dub_map.contains_key(dub) {
            panic!(
                "Tried to dub a lexeme type {:?}, which is already used to dub another \
                grammar symbol", dub,
            )
        }
        self.grammar_symbol_dub_map
            .insert(String::from(dub), GrammarSymbol::Terminal(lexeme_type.handle()));
    }

    /// Dub multiple categories of lexemes at once.
    ///
    /// # Panics
    ///
    /// If a new dub is already used to dub another grammar symbol.
    pub fn dub_lexeme_types<'a>(&mut self, lexeme_type_dubs: impl Iterator<Item=(LexemeType, &'a str)>) {
        for (lexeme_type, dub) in lexeme_type_dubs {
            self.dub_lexeme_type(lexeme_type, dub);
        }
    }

    /// Create a new nonterminal grammar symbol, associated with the given dub.
    ///
    /// # Panics
    ///
    /// If some other grammar symbol is already identified by this dub.
    pub fn new_nonterminal(&mut self, dub: &str) {
        if self.grammar_symbol_dub_map.contains_key(dub) {
            panic!(
                "Tried to create a new nonterminal dubbed {:?}, which is already used to dub \
                another grammar symbol", dub,
            )
        }
        let nonterminal = self.nonterminals.insert(Nonterminal);
        self.grammar_symbol_dub_map.insert(String::from(dub), GrammarSymbol::Nonterminal(nonterminal));
    }

    /// Create multiple nonterminals at once, associated with the given dubs.
    ///
    /// # Panics
    ///
    /// If a new dub is already used to dub another grammar symbol.
    pub fn new_nonterminals<'a>(&mut self, dubs: impl Iterator<Item=&'a str>) {
        for dub in dubs {
            self.new_nonterminal(dub);
        }
    }

    /// Set the start nonterminal of the underlying grammar.
    ///
    /// The nonterminal is identified by its `dub`.
    ///
    /// # Panics
    ///
    /// If no nonterminal is associated with this dub.
    pub fn set_start_nonterminal(&mut self, dub: &str) {
        let nonterminal = match self.grammar_symbol_dub_map.get(dub) {
            Some(GrammarSymbol::Nonterminal(nonterminal)) => *nonterminal,
            _ => panic!(
                "Tried to set start nonterminal to a non-existing nonterminal dubbed {:?}",
                dub,
            ),
        };
        self.lr_parser_builder.set_start_nonterminal(nonterminal);
    }

    /// Register a new binding.
    ///
    /// Note that the order of registration determines the priority of the bindings: bindings
    /// registered earlier have higher priority over bindings registered later.
    ///
    /// # Arguments
    /// * `bound_lexeme_types_dubs` - the dubs of the terminal grammar symbols (lexeme categories)
    ///     that are bound to this binding.
    /// * `associativity` - the binding's associativity.
    /// * `binding_dub` - a dub, used to identify this binding.
    ///
    /// # Panics
    ///
    /// If the binding's dub is already used to dub another binding, or if no terminal symbol is
    /// dubbed with one of the `bound_lexeme_types_dubs`.
    pub fn new_binding(
        &mut self,
        bound_lexeme_types_dubs: Vec<&str>,
        associativity: Associativity,
        binding_dub: &str,
    ) {
        if self.bindings_dub_map.contains_key(binding_dub) {
            panic!(
                "Tried to create a new binding dubbed {:?}, which is already used to dub an \
                existing binding", binding_dub,
            )
        }
        let terminals = bound_lexeme_types_dubs.iter()
            .map(|&lexeme_type_dub| match self.grammar_symbol_dub_map.get(lexeme_type_dub) {
                Some(GrammarSymbol::Terminal(terminal)) => *terminal,
                _ => panic!(
                    "Tried to create a binding dubbed {:?} , bound to a non-existing \
                        lexeme type dubbed {:?}", binding_dub, lexeme_type_dub,
                ),
            }).collect();
        self.bindings_dub_map.insert(
            String::from(binding_dub),
            self.lr_parser_builder.register_binding(terminals, associativity),
        );
    }

    /// Set the function that'll transform lexemes of a certain type into satellite data.
    ///
    /// When a lexeme of the category dubbed `lexeme_type_dub` will be encountered, `builder` will
    /// be called with the lexeme's contents (alongside a reference to the translation context), and
    /// the output will serve as the satellite data attached to this "terminal symbol".
    ///
    /// # Panics
    ///
    /// If `lexeme_type_dub` isn't a known lexeme-type dub.
    pub fn set_leaf_satellite_builder<F>(&mut self, lexeme_type_dub: &str, builder: F)
    where
        F: Fn(&mut Context, String) -> Satellite + 'static,
    {
        let lexeme_type =
            match self.grammar_symbol_dub_map.get(lexeme_type_dub) {
                Some(GrammarSymbol::Terminal(lexeme_type)) => *lexeme_type,
                _ => panic!(
                    "Tried to set a leaf satellite builder for a non-existing lexeme type dubbed \
                    {:?}", lexeme_type_dub,
                )
            };
        self.leaf_satellite_builder_map.insert(lexeme_type, Box::new(builder));
    }

    /// Set a leaf-satellite builder to process lexeme types for which no builder was set with
    /// [SyntaxDirectedTranslatorBuilder::set_leaf_satellite_builder].
    pub fn set_default_leaf_satellite_builder<F>(&mut self, builder: F)
    where
        F: Fn(&mut Context, String) -> Satellite + 'static,
    {
        self.default_leaf_satellite_builder = Some(Box::new(builder));
    }

    /// Add a production rule to the underlying context-free grammar.
    ///
    /// # Arguments
    ///
    /// * `lhs` - the dub of the nonterminal in the left-hand side of the production.
    /// * `rhs` - a sequence of grammar-symbols dubs (terminals and nonterminals), that appear in
    ///     the right-hand side of the production.
    /// * `satellite_reducer` - when reducing by the new rule, this function will be called with the
    ///     satellite data of the right-hand side grammar symbols (and the translation context), and
    ///     its output will serve as the satellite data attached to the left-hand side nonterminal.
    ///
    /// # Panics
    ///
    /// If any the given dubs isn't known as a matching dub.
    pub fn register_rule<F>(
        &mut self,
        lhs: &str,
        rhs: Vec<&str>,
        satellite_reducer: F,
    ) where
        F: Fn(&mut Context, Vec<Satellite>) -> Satellite + 'static,
    {
        self.register_rule_raw(lhs, rhs, None, satellite_reducer);
    }

    /// Register a production rule with an associated binding.
    ///
    /// The associated binding is identified by its dub - `binding_dub`. The rest of the arguments
    /// are identical to [SyntaxDirectedTranslatorBuilder::register_rule].
    ///
    /// # Panics
    ///
    /// If any the given dubs isn't known as a matching dub.
    pub fn register_bound_rule<F>(
        &mut self,
        lhs: &str,
        rhs: Vec<&str>,
        binding_dub: &str,
        satellite_reducer: F,
    ) where
        F: Fn(&mut Context, Vec<Satellite>) -> Satellite + 'static,
    {
        self.register_rule_raw(lhs, rhs, Some(binding_dub), satellite_reducer);
    }

    /// Register a production rule for which the satellite reducer copies the satellite data from
    /// its 1-length RHS to its LHS.
    ///
    /// This is commonly used for productions, such as `expression -> integer_literal`.
    ///
    /// # Arguments
    ///
    /// * `lhs` - the dub of the nonterminal in the left-hand side of the production.
    /// * `rhs` - the dub of the only grammar-symbol in the right-hand side of the production.
    ///
    /// # Panics
    ///
    /// If any the given dubs isn't known as a matching dub.
    pub fn register_identity_rule(&mut self, lhs: &str, rhs: &str) {
        self.register_rule(
            lhs,
            vec![rhs],
            |_context, mut satellites| satellites.pop().expect(
                "Tried to reduce satellites using `identity_satellite_reducer`, but the \
                provided RHS satellite list is empty (it should contain a single satellite for the \
                single item in the RHS)"
            ),
        );
    }

    /// Register a production rule whose RHS is empty.
    ///
    /// This is commonly used with nonterminals that represent lists that can be empty. For
    /// example `statements_list -> _nothing_`.
    ///
    /// # Arguments
    ///
    /// * `lhs` - the dub of the nonterminal in the left-hand side of the production.
    /// * `default_satellite_builder` - when reducing by this rule, this will be called (with the
    ///     translation context), and its output will serve as the satellite data attached to the
    ///     left-hand side nonterminal.
    ///
    /// # Panics
    ///
    /// If `lhs` is not a known dub for a nonterminal.
    pub fn register_empty_rule<F>(&mut self, lhs: &str, default_satellite_builder: F)
    where
        F: Fn(&mut Context) -> Satellite + 'static,
    {
        self.register_rule(
            lhs,
            vec![],
            move |context, _satellites| default_satellite_builder(context),
        )
    }

    fn register_rule_raw<F>(
        &mut self,
        lhs_dub: &str,
        rhs_dubs: Vec<&str>,
        binding_dub: Option<&str>,
        satellite_reducer: F,
    ) where
        F: Fn(&mut Context, Vec<Satellite>) -> Satellite + 'static,
    {
        let lhs = match self.grammar_symbol_dub_map.get(lhs_dub) {
            Some(GrammarSymbol::Nonterminal(nonterminal)) => *nonterminal,
            _ => panic!(
                "Tried to register a production rule whose LHS is a non-existing nonterminal \
                dubbed {:?}", lhs_dub,
            ),
        };
        let rhs =
            rhs_dubs.iter().map(|&dub| match self.grammar_symbol_dub_map.get(dub) {
                Some(&grammar_symbol) => grammar_symbol,
                _ => panic!(
                    "Tried to register a production rule whose RHS contains non-existing grammar \
                symbol dubbed {:?}", dub,
                )
            }).collect();
        let binding = binding_dub.map(|actual_binding_dub| {
            match self.bindings_dub_map.get(actual_binding_dub) {
                Some(&binding) => binding,
                None => panic!(
                    "Tried to register a production rule bound to a non-existing binding dubbed \
                    {:?}", actual_binding_dub,
                ),
            }
        });
        let tag = self.satellite_reducers.insert(Box::new(satellite_reducer));
        self.lr_parser_builder.register_rule(lhs, rhs, binding, tag);
    }

    /// Compile the set specifications into a functioning [SyntaxDirectedTranslator].
    pub fn build(self) -> SyntaxDirectedTranslator<LexemeType, Context, Satellite> {
        SyntaxDirectedTranslator {
            lr_parser: self.lr_parser_builder.build(),
            default_leaf_satellite_builder: self.default_leaf_satellite_builder,
            leaf_satellite_builder_map: self.leaf_satellite_builder_map,
            satellite_reducers: self.satellite_reducers,
        }
    }
}

// Blank, don't really need to carry any info, Handle API is only used for counting registrations
pub struct Nonterminal;
impl Handled for Nonterminal { type HandleCoreType = u8; }
