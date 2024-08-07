use std::collections::HashMap;
use crate::automata::nfa::Nfa;
use crate::handle::{Handle, Handled};
use crate::handle::handle_map::HandleMap;
use crate::handle::handled_vec::HandledVec;
use crate::handle::order::OrderlyHandled;
use crate::parsing::lr_parser::build::grammar_symbols::GrammarSymbolsCollection;
use crate::parsing::lr_parser::build::kernel_sets_dfa::{Item, KernelItemsSet, KernelSetsDfa};
use crate::parsing::lr_parser::rules::{ProductionRule, GrammarSymbol};

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
    ) -> Self
    {
        KernelSetsDfaBuilder::new(rules, start_rule, grammar_symbols).build()
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
}

impl<'a, Terminal, Nonterminal, Tag> KernelSetsDfaBuilder<'a, Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    fn new(
        rules: &'a HandledVec<ProductionRule<Terminal, Nonterminal, Tag>>,
        start_rule: Handle<ProductionRule<Terminal, Nonterminal, Tag>>,
        grammar_symbols: &'a GrammarSymbolsCollection<Terminal, Nonterminal>,
    ) -> Self
    {
        Self {
            grammar_symbols,
            rules,
            start_rule,
        }
    }

    fn build(self) -> KernelSetsDfa<Terminal, Nonterminal, Tag> {
        let rules_map = self.build_nonterminal_rules_map();
        let items_nfa = self.build_items_nfa(&rules_map);
        self.items_nfa_to_kernels_dfa(&items_nfa)
    }

    fn build_nonterminal_rules_map(&self)
                                   -> HandleMap<Nonterminal, Vec<Handle<ProductionRule<Terminal, Nonterminal, Tag>>>>
    {
        let mut map = HandleMap::new();
        for nonterminal in self.grammar_symbols.list_nonterminals() {
            map.insert(nonterminal, Vec::new());
        }
        for rule in self.rules.list_handles() {
            map.get_mut(self.rules[rule].lhs).expect(
                "Every nonterminal should have a map entry associated with it, as created in \
                the preceding loop"
            ).push(rule);
        }
        map
    }

    fn build_items_nfa(
        &self,
        rules_map: &HandleMap<Nonterminal, Vec<Handle<ProductionRule<Terminal, Nonterminal, Tag>>>>,
    ) -> Nfa<GrammarSymbol<Terminal, Nonterminal>, Item<Terminal, Nonterminal, Tag>>
    {
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
                let next_item = Item { rule: item.rule, dot: item.dot + 1 };
                let &next_item_state = item_states_map.get(&next_item).expect(
                    "Every item should have an NFA state associated with it"
                );
                let symbol = self.grammar_symbols
                    .get_handle(&self.rules[item.rule].rhs[item.dot]);
                nfa.link(state, next_item_state, Some(symbol));

                if let GrammarSymbol::Nonterminal(nonterminal)
                    = self.rules[item.rule].rhs[item.dot]
                {
                    let nonterminal_rules =
                        rules_map.get(nonterminal).expect(
                            "Every nonterminal should have a (maybe empty) vector of rules \
                            associated with it"
                        );
                    for &next_rule in nonterminal_rules {
                        let next_item = Item { rule: next_rule, dot: 0 };
                        let &next_item_state = item_states_map.get(&next_item)
                            .expect("Every item should have an NFA state associated with it");
                        nfa.link(state, next_item_state, None);
                    }
                }
            }
        }

        let start_item = Item { rule: self.start_rule, dot: 0 };
        let &start_rule_state = item_states_map.get(&start_item)
            .expect("Every item should have an NFA state associated with it");
        nfa.set_initial_state(start_rule_state);

        nfa
    }

    fn items_nfa_to_kernels_dfa(
        &self,
        nfa: &Nfa<GrammarSymbol<Terminal, Nonterminal>, Item<Terminal, Nonterminal, Tag>>,
    ) -> KernelSetsDfa<Terminal, Nonterminal, Tag>
    {
        nfa.compile_to_dfa(|items| {
            Some(KernelItemsSet::new(
                items
                    .into_iter()
                    .filter(|item| item.is_kernel_item(self.start_rule))
                    .copied()
            ))
        })
    }
}
