use std::collections::HashMap;

use crate::automata::nfa::Nfa;
use crate::handles::{Handle, Handled};
use crate::handles::collections::{HandledVec, HandleMap};
use crate::handles::specials::OrderlyHandled;
use crate::parsing::lr_parser::build::grammar_symbols::GrammarSymbolsCollection;
use crate::parsing::lr_parser::build::kernel_sets_dfa::{Item, KernelSet, KernelSetsDfa};
use crate::parsing::lr_parser::rules::{GrammarSymbol, ProductionRule};

impl<Terminal, Nonterminal, Tag> KernelSetsDfa<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    pub fn build<'a>(
        rules: &'a HandledVec<ProductionRule<Terminal, Nonterminal, Tag>>,
        start_rule: Handle<ProductionRule<Terminal, Nonterminal, Tag>>,
        grammar_symbols: &'a GrammarSymbolsCollection<Terminal, Nonterminal>,
        rules_for_nonterminals: &'a HandleMap<
            Nonterminal,
            Vec<Handle<ProductionRule<Terminal, Nonterminal, Tag>>>,
        >,
    ) -> Self {
        KernelSetsDfaBuilder {
            rules,
            start_rule,
            grammar_symbols,
            rules_for_nonterminals,
        }
            .build()
    }
}

struct KernelSetsDfaBuilder<'a, Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    grammar_symbols: &'a GrammarSymbolsCollection<Terminal, Nonterminal>,
    rules: &'a HandledVec<ProductionRule<Terminal, Nonterminal, Tag>>,
    start_rule: Handle<ProductionRule<Terminal, Nonterminal, Tag>>,
    rules_for_nonterminals:
        &'a HandleMap<Nonterminal, Vec<Handle<ProductionRule<Terminal, Nonterminal, Tag>>>>,
}

impl<'a, Terminal, Nonterminal, Tag> KernelSetsDfaBuilder<'a, Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    fn build(self) -> KernelSetsDfa<Terminal, Nonterminal, Tag> {
        let items_nfa = self.build_items_nfa();
        self.items_nfa_to_kernels_dfa(&items_nfa)
    }

    fn build_items_nfa(
        &self,
    ) -> Nfa<GrammarSymbol<Terminal, Nonterminal>, Item<Terminal, Nonterminal, Tag>> {
        let mut nfa = Nfa::new();
        let mut item_states_map = HashMap::new();

        for rule in self.rules.list_handles() {
            for dot in 0..=self.rules[rule].rhs.len() {
                let state = nfa.new_state();
                let item = Item { rule, dot };
                item_states_map.insert(item, state);
                nfa.label(state, Some(item));
            }
        }

        for (item, &state) in item_states_map.iter() {
            if item.dot < self.rules[item.rule].rhs.len() {
                let next_item = Item {
                    rule: item.rule,
                    dot: item.dot + 1,
                };
                let &next_item_state = item_states_map
                    .get(&next_item)
                    .expect("Every item should have an NFA state associated with it");
                let symbol = self
                    .grammar_symbols
                    .get_handle(&self.rules[item.rule].rhs[item.dot]);
                nfa.link(state, next_item_state, Some(symbol));

                if let GrammarSymbol::Nonterminal(nonterminal) = self.rules[item.rule].rhs[item.dot]
                {
                    let rules_for_nonterminal =
                        self.rules_for_nonterminals.get(nonterminal).expect(
                            "Every nonterminal should have a (maybe empty) vector of rules \
                            associated with it",
                        );
                    for &next_rule in rules_for_nonterminal {
                        let next_item = Item {
                            rule: next_rule,
                            dot: 0,
                        };
                        let &next_item_state = item_states_map
                            .get(&next_item)
                            .expect("Every item should have an NFA state associated with it");
                        nfa.link(state, next_item_state, None);
                    }
                }
            }
        }

        let start_item = Item {
            rule: self.start_rule,
            dot: 0,
        };
        let &start_rule_state = item_states_map
            .get(&start_item)
            .expect("Every item should have an NFA state associated with it");
        nfa.set_initial_state(start_rule_state);

        nfa
    }

    fn items_nfa_to_kernels_dfa(
        &self,
        nfa: &Nfa<GrammarSymbol<Terminal, Nonterminal>, Item<Terminal, Nonterminal, Tag>>,
    ) -> KernelSetsDfa<Terminal, Nonterminal, Tag> {
        nfa.compile_to_dfa(|items| {
            Some(KernelSet::new(
                items
                    .into_iter()
                    .filter(|item| self.is_kernel_item(item))
                    .copied(),
            ))
        })
    }

    fn is_kernel_item(&self, item: &Item<Terminal, Nonterminal, Tag>) -> bool {
        // My addition to the algorithm: empty rules are also kernel items
        (item.dot != 0) || (item.rule == self.start_rule) || self.rules[item.rule].rhs.is_empty()
    }
}
