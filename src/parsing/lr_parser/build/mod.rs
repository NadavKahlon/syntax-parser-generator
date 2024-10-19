use std::collections::HashSet;

use crate::handles::{Handle, Handled};
use crate::handles::collections::{HandledVec, HandleMap};
use crate::handles::specials::OrderlyHandled;
use crate::parsing::lr_parser::build::grammar_symbols::GrammarSymbolsCollection;
use crate::parsing::lr_parser::build::kernel_sets_dfa::KernelSetsDfa;
use crate::parsing::lr_parser::LrParser;
use crate::parsing::lr_parser::rules::{Associativity, Binding, GrammarSymbol, ProductionRule};

mod grammar_symbols;
mod kernel_sets_dfa;

pub struct LrParserBuilder<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    bindings: HandledVec<Binding<Terminal>>,
    terminal_bindings_map: HandleMap<Terminal, Handle<Binding<Terminal>>>,
    rules: HandledVec<ProductionRule<Terminal, Nonterminal, Tag>>,
    start_nonterminal: Option<Handle<Nonterminal>>,
}

impl<Terminal, Nonterminal, Tag> LrParserBuilder<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    pub fn new() -> Self {
        Self {
            bindings: HandledVec::new(),
            terminal_bindings_map: HandleMap::new(),
            rules: HandledVec::new(),
            start_nonterminal: None,
        }
    }

    pub fn register_binding(
        &mut self,
        terminals: Vec<Handle<Terminal>>,
        associativity: Associativity,
    ) -> Handle<Binding<Terminal>> {
        let binding = self.bindings.insert(Binding::new(terminals, associativity));
        for &terminal in &self.bindings[binding].terminals {
            if !self.terminal_bindings_map.get(terminal).is_none() {
                panic!("Multiple bindings defined for terminal {:?}", terminal);
            }
            self.terminal_bindings_map.insert(terminal, binding);
        }
        binding
    }

    pub fn register_rule(
        &mut self,
        lhs: Handle<Nonterminal>,
        rhs: Vec<GrammarSymbol<Terminal, Nonterminal>>,
        binding: Option<Handle<Binding<Terminal>>>,
        tag: Handle<Tag>,
    ) {
        self.rules
            .insert(ProductionRule::new(lhs, rhs, tag, binding));
    }

    pub fn set_start_nonterminal(&mut self, nonterminal: Handle<Nonterminal>) {
        self.start_nonterminal = Some(nonterminal);
    }

    fn list_known_handles(
        &self,
    ) -> (
        Vec<Handle<Terminal>>,
        Vec<Handle<Nonterminal>>,
        Vec<Handle<Tag>>,
    ) {
        let mut terminals: HashSet<Handle<Terminal>> = HashSet::new();
        let mut nonterminals: HashSet<Handle<Nonterminal>> = HashSet::new();
        let mut tags: HashSet<Handle<Tag>> = HashSet::new();

        for binding in &self.bindings {
            terminals.extend(&binding.terminals);
        }

        for rule in &self.rules {
            nonterminals.insert(rule.lhs);
            for symbol in &rule.rhs {
                match symbol {
                    GrammarSymbol::Terminal(terminal) => {
                        terminals.insert(*terminal);
                    }
                    GrammarSymbol::Nonterminal(nonterminal) => {
                        nonterminals.insert(*nonterminal);
                    }
                }
            }
            tags.insert(rule.tag);
        }

        if let Some(start_nonterminal) = self.start_nonterminal {
            nonterminals.insert(start_nonterminal);
        }

        (
            terminals.into_iter().collect(),
            nonterminals.into_iter().collect(),
            tags.into_iter().collect(),
        )
    }

    fn index_rules_by_nonterminals(
        &self,
        grammar_symbols: &GrammarSymbolsCollection<Terminal, Nonterminal>,
    ) -> HandleMap<Nonterminal, Vec<Handle<ProductionRule<Terminal, Nonterminal, Tag>>>> {
        let mut map = HandleMap::new();
        for nonterminal in grammar_symbols.list_nonterminals() {
            map.insert(nonterminal, Vec::new());
        }
        for rule in self.rules.list_handles() {
            map.get_mut(self.rules[rule].lhs)
                .expect(
                    "Every nonterminal should have a map entry associated with it, as created in \
                the preceding loop",
                )
                .push(rule);
        }
        map
    }

    pub fn build(mut self) -> LrParser<Terminal, Nonterminal, Tag> {
        let (mut terminals, mut nonterminals, mut tags) = self.list_known_handles();

        let actual_start_nonterminal = Handle::mock(&nonterminals);
        let end_of_input_marker = Handle::mock(&terminals);
        let start_rule_tag = Handle::mock(&tags);
        nonterminals.push(actual_start_nonterminal);
        terminals.push(end_of_input_marker);
        tags.push(start_rule_tag);

        let grammar_symbols = GrammarSymbolsCollection::new(&terminals, &nonterminals);

        let specified_start_nonterminal = self
            .start_nonterminal
            .expect("Cannot build an LR-parser when no start-nonterminal was specified");
        let start_rule = self.rules.insert(ProductionRule::new(
            actual_start_nonterminal,
            vec![GrammarSymbol::Nonterminal(specified_start_nonterminal)],
            start_rule_tag,
            None,
        ));

        let rules_for_nonterminals = self.index_rules_by_nonterminals(&grammar_symbols);

        let mut kernel_sets_dfa = KernelSetsDfa::build(
            &self.rules,
            start_rule,
            &grammar_symbols,
            &rules_for_nonterminals,
        );
        kernel_sets_dfa.generate_lookaheads(
            &grammar_symbols,
            &self.rules,
            start_rule,
            &rules_for_nonterminals,
            end_of_input_marker,
        );
        kernel_sets_dfa.compile_to_parser(
            &grammar_symbols,
            &self.rules,
            start_rule,
            &self.bindings,
            &self.terminal_bindings_map,
            end_of_input_marker,
        )
    }
}
