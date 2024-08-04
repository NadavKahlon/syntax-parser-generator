use crate::handle::{Handle, Handled};
use crate::handle::handled_vec::HandledVec;
use crate::handle::order::OrderlyHandled;
use crate::parsing::lr_parser::LrParser;
use crate::parsing::lr_parser::rules::{Associativity, Binding, ProductionRule, Symbol};

pub struct LrParserBuilder<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    bindings: HandledVec<Binding>,
    rules: Vec<ProductionRule<Terminal, Nonterminal, Tag>>,
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
            rules: Vec::new(),
        }
    }

    pub fn register_binding(&mut self, associativity: Associativity) -> Handle<Binding> {
        self.bindings.insert(Binding::new(associativity))
    }

    pub fn register_rule(
        &mut self,
        lhs: Handle<Nonterminal>,
        rhs: Vec<Symbol<Terminal, Nonterminal>>,
        binding: Option<Handle<Binding>>,
        tag: Handle<Tag>,
    ) {
        self.rules.push(ProductionRule::new(lhs, rhs, tag, binding))
    }

    pub fn build(self) -> LrParser<Terminal, Nonterminal, Tag>
    {
        todo!()
    }
}