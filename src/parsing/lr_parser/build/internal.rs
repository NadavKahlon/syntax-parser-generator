use crate::handle::{Handle, Handled};
use crate::handle::handled_vec::HandledVec;
use crate::handle::order::OrderlyHandled;
use crate::parsing::lr_parser::LrParser;
use crate::parsing::lr_parser::rules::{Binding, ProductionRule};

pub struct InternalLrParserBuilder<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    terminals: Vec<Handle<Terminal>>,
    nonterminals: Vec<Handle<Nonterminal>>,
    rules: HandledVec<ProductionRule<Terminal, Nonterminal, Tag>>,
    start_rule: Handle<ProductionRule<Terminal, Nonterminal, Tag>>,
    end_of_input_marker: Handle<Terminal>,
    bindings: HandledVec<Binding<Terminal>>,
}

impl<Terminal, Nonterminal, Tag> InternalLrParserBuilder<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    pub fn new(
        terminals: Vec<Handle<Terminal>>,
        nonterminals: Vec<Handle<Nonterminal>>,
        rules: HandledVec<ProductionRule<Terminal, Nonterminal, Tag>>,
        start_rule: Handle<ProductionRule<Terminal, Nonterminal, Tag>>,
        end_of_input_marker: Handle<Terminal>,
        bindings: HandledVec<Binding<Terminal>>,
    ) -> Self {
        Self { terminals, nonterminals, rules, start_rule, end_of_input_marker, bindings }
    }

    pub fn build(self) -> LrParser<Terminal, Nonterminal, Tag> {
        todo!()
    }
}
