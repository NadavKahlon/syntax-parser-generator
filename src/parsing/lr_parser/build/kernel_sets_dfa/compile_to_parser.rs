use crate::handles::{Handle, Handled};
use crate::handles::collections::{HandledVec, HandleMap};
use crate::handles::specials::OrderlyHandled;
use crate::parsing::lr_parser::{LrParser, LrParserAction, LrParserState};
use crate::parsing::lr_parser::build::grammar_symbols::GrammarSymbolsCollection;
use crate::parsing::lr_parser::build::kernel_sets_dfa::{Item, KernelSetsDfa, KernelSetsDfaState};
use crate::parsing::lr_parser::rules::{Associativity, Binding, ProductionRule};

impl<Terminal, Nonterminal, Tag> KernelSetsDfa<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    pub fn compile_to_parser(
        &self,
        grammar_symbols: &GrammarSymbolsCollection<Terminal, Nonterminal>,
        rules: &HandledVec<ProductionRule<Terminal, Nonterminal, Tag>>,
        start_rule: Handle<ProductionRule<Terminal, Nonterminal, Tag>>,
        bindings: &HandledVec<Binding<Terminal>>,
        terminal_bindings_map: &HandleMap<Terminal, Handle<Binding<Terminal>>>,
        end_of_input_marker: Handle<Terminal>,
    ) -> LrParser<Terminal, Nonterminal, Tag> {
        LrParserCompiler::new(
            self,
            grammar_symbols,
            rules,
            start_rule,
            bindings,
            terminal_bindings_map,
            end_of_input_marker,
        )
            .compile()
    }
}

struct LrParserCompiler<'a, Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    parser: LrParser<Terminal, Nonterminal, Tag>,
    states_map: HandleMap<
        KernelSetsDfaState<Terminal, Nonterminal, Tag>,
        Handle<LrParserState<Terminal, Nonterminal, Tag>>,
    >,
    dfa: &'a KernelSetsDfa<Terminal, Nonterminal, Tag>,
    grammar_symbols: &'a GrammarSymbolsCollection<Terminal, Nonterminal>,
    rules: &'a HandledVec<ProductionRule<Terminal, Nonterminal, Tag>>,
    final_item: Item<Terminal, Nonterminal, Tag>,
    bindings: &'a HandledVec<Binding<Terminal>>,
    terminal_bindings_map: &'a HandleMap<Terminal, Handle<Binding<Terminal>>>,
    end_of_input_marker: Handle<Terminal>,
}

impl<'a, Terminal, Nonterminal, Tag> LrParserCompiler<'a, Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    fn new(
        dfa: &'a KernelSetsDfa<Terminal, Nonterminal, Tag>,
        grammar_symbols: &'a GrammarSymbolsCollection<Terminal, Nonterminal>,
        rules: &'a HandledVec<ProductionRule<Terminal, Nonterminal, Tag>>,
        start_rule: Handle<ProductionRule<Terminal, Nonterminal, Tag>>,
        bindings: &'a HandledVec<Binding<Terminal>>,
        terminal_bindings_map: &'a HandleMap<Terminal, Handle<Binding<Terminal>>>,
        end_of_input_marker: Handle<Terminal>,
    ) -> Self {
        let mut parser = LrParser::new();
        let mut states_map = HandleMap::new();
        for state in dfa.list_states() {
            states_map.insert(state, parser.new_state());
        }
        let final_item = Item {
            rule: start_rule,
            dot: rules[start_rule].rhs.len(),
        };
        Self {
            parser,
            states_map,
            dfa,
            grammar_symbols,
            rules,
            final_item,
            bindings,
            terminal_bindings_map,
            end_of_input_marker,
        }
    }

    fn get_parser_state(
        &self,
        dfa_state: Handle<KernelSetsDfaState<Terminal, Nonterminal, Tag>>,
    ) -> Handle<LrParserState<Terminal, Nonterminal, Tag>> {
        *self.states_map.get(dfa_state).expect(
            "Every state in the kernel-sets DFA should be associated with a state in the \
            target parser",
        )
    }

    fn compile(mut self) -> LrParser<Terminal, Nonterminal, Tag> {
        for (src_dfa_state, &src_parser_state) in &self.states_map.clone() {
            for terminal in self.grammar_symbols.list_terminals() {
                if let Some(action) = self.get_action(src_dfa_state, terminal) {
                    self.parser.set_action(src_parser_state, terminal, action);
                };
            }
            for nonterminal in self.grammar_symbols.list_nonterminals() {
                if let Some(tar_parser_state) = self.get_goto(src_dfa_state, nonterminal) {
                    self.parser
                        .set_goto(src_parser_state, nonterminal, tar_parser_state);
                };
            }
        }

        self.parser
            .set_end_of_input_marker(self.end_of_input_marker);
        self.parser
            .set_initial_state(self.get_parser_state(self.dfa.get_initial_state().expect(
                "The kernel-sets DFA is expected to have an initial state (associated with the \
            start rule)",
            )));
        self.parser
    }

    fn get_goto(
        &mut self,
        src_dfa_state: Handle<KernelSetsDfaState<Terminal, Nonterminal, Tag>>,
        nonterminal: Handle<Nonterminal>,
    ) -> Option<Handle<LrParserState<Terminal, Nonterminal, Tag>>> {
        if let Some(tar_dfa_state) = self.dfa.step(
            src_dfa_state,
            self.grammar_symbols.symbol_from_nonterminal(nonterminal),
        ) {
            Some(self.get_parser_state(tar_dfa_state))
        } else {
            None
        }
    }

    fn get_action(
        &mut self,
        src_dfa_state: Handle<KernelSetsDfaState<Terminal, Nonterminal, Tag>>,
        terminal: Handle<Terminal>,
    ) -> Option<LrParserAction<Terminal, Nonterminal, Tag>> {
        let mut suggested_actions = self.collect_suggested_actions(src_dfa_state, terminal);

        let mut preferred_action = suggested_actions.pop()?;
        for alternative_action in suggested_actions {
            if self.is_action_preferred(&alternative_action, &preferred_action) {
                preferred_action = alternative_action;
            }
        }
        Some(self.build_suggested_action(preferred_action))
    }

    fn collect_suggested_actions(
        &mut self,
        src_dfa_state: Handle<KernelSetsDfaState<Terminal, Nonterminal, Tag>>,
        terminal: Handle<Terminal>,
    ) -> Vec<SuggestedAction<Terminal, Nonterminal, Tag>> {
        let mut suggested_actions = vec![];
        let kernel_set = self.dfa.get_label(src_dfa_state).as_ref().expect(
            "Every state in the kernel-sets DFA should be labeled by the corresponding set",
        );

        // Suggest shift
        if let Some(tar_dfa_state) = self.dfa.step(
            src_dfa_state,
            self.grammar_symbols.symbol_from_terminal(terminal),
        ) {
            suggested_actions.push(SuggestedAction::Shift {
                terminal,
                state: self.get_parser_state(tar_dfa_state),
            });
        }

        // Suggest reduces
        for kernel_set_entry in &kernel_set.entries {
            let item_rule = &self.rules[kernel_set_entry.item.rule];
            if (kernel_set_entry.item.dot == item_rule.rhs.len())
                && (kernel_set_entry.lookaheads.contains(&terminal))
            {
                suggested_actions.push(SuggestedAction::Reduce(kernel_set_entry.item.rule));
            }
        }

        // Suggest accept
        if terminal == self.end_of_input_marker {
            for kernel_set_entry in &kernel_set.entries {
                if kernel_set_entry.item == self.final_item {
                    suggested_actions.push(SuggestedAction::Accept);
                    break;
                }
            }
        }

        suggested_actions
    }

    fn is_action_preferred(
        &self,
        action_1: &SuggestedAction<Terminal, Nonterminal, Tag>,
        action_2: &SuggestedAction<Terminal, Nonterminal, Tag>,
    ) -> bool {
        match (action_1, action_2) {
            (SuggestedAction::Accept, _) => true,
            (_, SuggestedAction::Accept) => false,
            (SuggestedAction::Shift { .. }, SuggestedAction::Shift { .. }) => {
                panic!("Shift-shift conflicts are impossible")
            }
            (SuggestedAction::Reduce(rule_1), SuggestedAction::Reduce(rule_2)) => {
                // Resolve reduce-reduce conflict in favor of the first rule
                self.rules[*rule_1].tag < self.rules[*rule_2].tag
            }
            (SuggestedAction::Shift { terminal, .. }, SuggestedAction::Reduce(rule)) => {
                self.resolve_shift_reduce_conflict(*terminal, *rule)
            }
            (SuggestedAction::Reduce(rule), SuggestedAction::Shift { terminal, .. }) => {
                !self.resolve_shift_reduce_conflict(*terminal, *rule)
            }
        }
    }

    // True for shift, false for reduce
    fn resolve_shift_reduce_conflict(
        &self,
        shifted_terminal: Handle<Terminal>,
        reduced_rule: Handle<ProductionRule<Terminal, Nonterminal, Tag>>,
    ) -> bool {
        match (
            self.terminal_bindings_map.get(shifted_terminal),
            self.rules[reduced_rule].binding,
        ) {
            // We shift by default
            (None, _) => true,
            (_, None) => true,

            (Some(&binding_1), Some(binding_2)) => {
                if binding_1 < binding_2 {
                    true
                } else if binding_1 > binding_2 {
                    false
                } else {
                    match self.bindings[binding_1].associativity {
                        Associativity::Left => false,
                        Associativity::Right => true,
                        Associativity::None => panic!(
                            "Cannot resolve a shift-reduce conflict if the bindings are the same, \
                            and it does not associate to any side"
                        ),
                    }
                }
            }
        }
    }

    fn build_suggested_action(
        &self,
        suggested_action: SuggestedAction<Terminal, Nonterminal, Tag>,
    ) -> LrParserAction<Terminal, Nonterminal, Tag> {
        match suggested_action {
            SuggestedAction::Shift { state, .. } => LrParserAction::Shift(state),
            SuggestedAction::Reduce(rule) => LrParserAction::Reduce {
                size: self.rules[rule].rhs.len(),
                nonterminal: self.rules[rule].lhs,
                tag: self.rules[rule].tag,
            },
            SuggestedAction::Accept => LrParserAction::Accept,
        }
    }
}

enum SuggestedAction<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    Shift {
        terminal: Handle<Terminal>,
        state: Handle<LrParserState<Terminal, Nonterminal, Tag>>,
    },
    Reduce(Handle<ProductionRule<Terminal, Nonterminal, Tag>>),
    Accept,
}
