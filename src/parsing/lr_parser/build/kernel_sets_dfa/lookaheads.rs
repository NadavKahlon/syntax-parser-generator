use std::collections::HashSet;
use derive_where::derive_where;
use crate::handle::{Handle, Handled};
use crate::handle::handle_map::HandleMap;
use crate::handle::handled_vec::HandledVec;
use crate::handle::order::OrderlyHandled;
use crate::parsing::lr_parser::build::grammar_symbols::GrammarSymbolsCollection;
use crate::parsing::lr_parser::build::kernel_sets_dfa::{Item, KernelSetEntry, KernelSetsDfa, KernelSetsDfaState};
use crate::parsing::lr_parser::build::kernel_sets_dfa::firsts::FirstsMap;
use crate::parsing::lr_parser::rules::{GrammarSymbol, ProductionRule};

impl<Terminal, Nonterminal, Tag> KernelSetsDfa<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    pub fn generate_lookaheads(
        &mut self,
        grammar_symbols: &GrammarSymbolsCollection<Terminal, Nonterminal>,
        rules: &HandledVec<ProductionRule<Terminal, Nonterminal, Tag>>,
        rules_for_nonterminals: &HandleMap<
            Nonterminal, Vec<Handle<ProductionRule<Terminal, Nonterminal, Tag>>>
        >,
    ) {
        LookaheadsGenerator::new(grammar_symbols, self, rules, rules_for_nonterminals).generate();
    }
}

struct LookaheadsGenerator<'a, Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    dfa: &'a mut KernelSetsDfa<Terminal, Nonterminal, Tag>,
    rules: &'a HandledVec<ProductionRule<Terminal, Nonterminal, Tag>>,
    rules_for_nonterminals:
        &'a HandleMap<Nonterminal, Vec<Handle<ProductionRule<Terminal, Nonterminal, Tag>>>>,
    firsts_map: FirstsMap<'a, Terminal, Nonterminal, Tag>,
    grammar_symbols: &'a GrammarSymbolsCollection<Terminal, Nonterminal>,
}

impl<'a, Terminal, Nonterminal, Tag> LookaheadsGenerator<'a, Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    fn new(
        grammar_symbols: &'a GrammarSymbolsCollection<Terminal, Nonterminal>,
        dfa: &'a mut KernelSetsDfa<Terminal, Nonterminal, Tag>,
        rules: &'a HandledVec<ProductionRule<Terminal, Nonterminal, Tag>>,
        rules_for_nonterminals: &'a HandleMap<
            Nonterminal, Vec<Handle<ProductionRule<Terminal, Nonterminal, Tag>>>
        >,
    ) -> Self {
        Self {
            dfa,
            rules,
            rules_for_nonterminals,
            grammar_symbols,
            firsts_map: FirstsMap::new(rules, rules_for_nonterminals),
        }
    }
    fn generate(&mut self) {
        self.firsts_map.build();
        for state in self.dfa.list_states() {
            self.record_lookaheads_from(state);
        }
        self.close_lookahead_propagation();
    }

    fn record_lookaheads_from(
        &mut self, state: Handle<KernelSetsDfaState<Terminal, Nonterminal, Tag>>,
    ) {
        let kernel_set = self.dfa.kernel_set(state);
        let mut orders = Vec::new();
        let mock_terminal
            = Handle::mock(&self.grammar_symbols.list_terminals().collect());

        for kernel_set_entry_handle in kernel_set.entries.list_handles() {
            let kernel_set_entry = &kernel_set.entries[kernel_set_entry_handle];
            let mock_item = Lr1Item {
                rule: kernel_set_entry.item.rule,
                dot: kernel_set_entry.item.dot,
                lookahead: mock_terminal,
            };
            let mock_closure = self.lr1_item_closure(mock_item);
            for mock_closure_item in mock_closure {
                if let Some(&transition_symbol)
                    = self.rules[mock_closure_item.rule].rhs.get(mock_closure_item.dot)
                {
                    let target_item = Item {
                        rule: kernel_set_entry.item.rule,
                        dot: kernel_set_entry.item.dot + 1,
                    };
                    let (tar_state, tar_entry) =
                        self.locate_target_entry(state, transition_symbol, target_item);
                    if mock_closure_item.lookahead != mock_terminal {
                        orders.push(LookaheadRecordingOrder::Spontaneous {
                            tar_state,
                            tar_entry,
                            lookahead: mock_closure_item.lookahead,
                        });
                    } else {
                        orders.push(LookaheadRecordingOrder::Propagates {
                            src_state: state,
                            src_entry: kernel_set_entry_handle,
                            tar_state,
                            tar_entry,
                        });
                    }
                }
            }
        }
        for order in orders {
            self.handle_lookahead_record_order(order);
        }
    }

    fn close_lookahead_propagation(&mut self) {
        // TODO optimize: there are many list copied here to circumvent Rust's borrow checking
        let mut changed = true;
        while changed {
            changed = false;

            let states: Vec<Handle<KernelSetsDfaState<Terminal, Nonterminal, Tag>>> =
                self.dfa.list_states().collect();
            for state in states {

                let entries_handles: Vec<Handle<KernelSetEntry<Terminal, Nonterminal, Tag>>> =
                    self.dfa.kernel_set(state).entries.list_handles().collect();
                for entry_handle in entries_handles {

                    let entry = &self.dfa.kernel_set(state).entries[entry_handle];
                    let lookaheads = entry.lookaheads.clone();
                    let propagations = entry.propagations.clone();

                    for (tar_state, tar_entry) in
                        propagations
                    {
                        for &lookahead in &lookaheads {
                            changed |= self.dfa
                                .mut_kernel_set(tar_state)
                                .entries[tar_entry]
                                .lookaheads
                                .insert(lookahead);
                        };
                    }
                }
            }
        }
    }

    fn lr1_item_closure(&self, src_lr1_item: Lr1Item<Terminal, Nonterminal, Tag>)
                        -> HashSet<Lr1Item<Terminal, Nonterminal, Tag>>
    {
        let mut closure = HashSet::new();
        let mut lr1_items_to_process = vec![src_lr1_item];

        while let Some(lr1_item) = lr1_items_to_process.pop() {
            if closure.insert(lr1_item) {
                let rhs = &self.rules[lr1_item.rule].rhs;
                if let Some(&GrammarSymbol::Nonterminal(nonterminal))
                    = rhs.get(lr1_item.dot)
                {
                    let suffix: Vec<GrammarSymbol<Terminal, Nonterminal>> =
                        rhs[(lr1_item.dot + 1)..]
                            .iter()
                            .copied()
                            .chain(vec![GrammarSymbol::Terminal(lr1_item.lookahead)].into_iter())
                            .collect();
                    let first_terminals: Vec<Handle<Terminal>>
                        = self.firsts_map.terminal_firsts_for_string(&suffix).collect();

                    let next_rules = self
                        .rules_for_nonterminals
                        .get(nonterminal)
                        .expect(
                            "All nonterminals should have a (maybe empty) list of rules \
                            associated with them"
                        );
                    for &next_rule in next_rules {
                        for &first_terminal in &first_terminals {
                            lr1_items_to_process.push(Lr1Item {
                                rule: next_rule,
                                dot: 0,
                                lookahead: first_terminal,
                            });
                        }
                    }
                }
            }
        }
        closure
    }

    pub fn locate_target_entry(
        &self,
        src_state: Handle<KernelSetsDfaState<Terminal, Nonterminal, Tag>>,
        transition_symbol: GrammarSymbol<Terminal, Nonterminal>,
        target_item: Item<Terminal, Nonterminal, Tag>,
    ) -> (
        Handle<KernelSetsDfaState<Terminal, Nonterminal, Tag>>,
        Handle<KernelSetEntry<Terminal, Nonterminal, Tag>>,
    ) {
        let transition_symbol_handle =
            self.grammar_symbols.get_handle(&transition_symbol);
        let target_state = self.dfa.step(src_state, transition_symbol_handle)
            .expect(
                "Since the target item was found by stepping from an item in the source \
                item's closure, there should exist such transition from the source item's state"
            );
        let set = self.dfa.get_label(target_state).as_ref().expect(
            "Every state should be labeled by the corresponding kernel items set"
        );
        for entry_handle in set.entries.list_handles() {
            let entry = &set.entries[entry_handle];
            if entry.item == target_item {
                return (target_state, entry_handle);
            }
        }
        panic!(
            "Could not find the target item in the target state, though it was built by stepping \
            from an item in source item's closure"
        )
    }

    fn handle_lookahead_record_order(
        &mut self,
        order: LookaheadRecordingOrder<Terminal, Nonterminal, Tag>,
    ) {
        match order {
            LookaheadRecordingOrder::Propagates {
                src_state,
                src_entry,
                tar_state,
                tar_entry
            } => {
                self.dfa.mut_kernel_set(src_state).entries[src_entry].propagations.push(
                    (tar_state, tar_entry)
                );
            }
            LookaheadRecordingOrder::Spontaneous {
                tar_state,
                tar_entry,
                lookahead
            } => {
                self.dfa.mut_kernel_set(tar_state).entries[tar_entry].lookaheads.insert(lookahead);
            }
        }
    }
}

#[derive_where(Hash, Clone, Copy, PartialEq, Eq)]
struct Lr1Item<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    rule: Handle<ProductionRule<Terminal, Nonterminal, Tag>>,
    dot: usize,
    lookahead: Handle<Terminal>,
}


pub enum LookaheadRecordingOrder<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    Propagates {
        src_state: Handle<KernelSetsDfaState<Terminal, Nonterminal, Tag>>,
        src_entry: Handle<KernelSetEntry<Terminal, Nonterminal, Tag>>,
        tar_state: Handle<KernelSetsDfaState<Terminal, Nonterminal, Tag>>,
        tar_entry: Handle<KernelSetEntry<Terminal, Nonterminal, Tag>>,
    },
    Spontaneous {
        tar_state: Handle<KernelSetsDfaState<Terminal, Nonterminal, Tag>>,
        tar_entry: Handle<KernelSetEntry<Terminal, Nonterminal, Tag>>,
        lookahead: Handle<Terminal>,
    },
}
