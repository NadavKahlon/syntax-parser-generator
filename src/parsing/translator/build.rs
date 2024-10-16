use std::collections::HashMap;
use crate::handles::{Handle, Handled};
use crate::handles::collections::{HandledVec, HandleMap};
use crate::handles::specials::AutomaticallyHandled;
use crate::parsing::lr_parser::build::LrParserBuilder;
use crate::parsing::lr_parser::rules::{Associativity, Binding, GrammarSymbol};
use crate::parsing::translator::handlers::{LeafSatelliteBuilder, SatelliteReducer};
use crate::parsing::translator::sdt::SyntaxDirectedTranslator;

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

    pub fn dub_lexeme_types<'a>(&mut self, lexeme_type_dubs: impl Iterator<Item=(LexemeType, &'a str)>) {
        for (lexeme_type, dub) in lexeme_type_dubs {
            self.dub_lexeme_type(lexeme_type, dub);
        }
    }

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

    pub fn new_nonterminals<'a>(&mut self, dubs: impl Iterator<Item=&'a str>) {
        for dub in dubs {
            self.new_nonterminal(dub);
        }
    }

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

    pub fn set_default_leaf_satellite_builder<F>(&mut self, builder: F)
    where
        F: Fn(&mut Context, String) -> Satellite + 'static,
    {
        self.default_leaf_satellite_builder = Some(Box::new(builder));
    }

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
