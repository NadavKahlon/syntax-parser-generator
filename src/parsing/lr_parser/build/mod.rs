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
    bindings: HandledVec<Binding<Terminal>>,
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
            rules: HandledVec::new(),
            start_nonterminal: None,
        }
    }

    pub fn register_binding(
        &mut self, terminals: Vec<Handle<Terminal>>, associativity: Associativity,
    ) -> Handle<Binding<Terminal>>
    {
        self.bindings.insert(Binding::new(terminals, associativity))
    }

    pub fn register_rule(
        &mut self,
        lhs: Handle<Nonterminal>,
        rhs: Vec<Symbol<Terminal, Nonterminal>>,
        binding: Option<Handle<Binding<Terminal>>>,
        tag: Handle<Tag>,
    ) {
        self.rules.insert(ProductionRule::new(lhs, rhs, tag, binding));
    }

    pub fn set_start_nonterminal(&mut self, nonterminal: Handle<Nonterminal>) {
        self.start_nonterminal = Some(nonterminal);
    }

    pub fn build(self) -> LrParser<Terminal, Nonterminal, Tag>
    {
        todo!()
    }
}

struct Item<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    rule: Handle<ProductionRule<Terminal, Nonterminal, Tag>>,
    dot: usize,
}

struct KernelItemsSet<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    entries: HandledVec<KernelItemsSetEntry<Terminal, Nonterminal, Tag>>,
}

impl<Terminal, Nonterminal, Tag> Handled for KernelItemsSet<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    type HandleCoreType = u16;
}


struct KernelItemsSetEntry<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    item: Item<Terminal, Nonterminal, Tag>,
    lookaheads: Vec<Handle<Terminal>>,
    propagations:
        Vec<(
            Handle<KernelItemsSet<Terminal, Nonterminal, Tag>>,
            Handle<KernelItemsSetEntry<Terminal, Nonterminal, Tag>>,
            Handle<Terminal>,
        )>,
}

impl<Terminal, Nonterminal, Tag> Handled for KernelItemsSetEntry<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    type HandleCoreType = u16;
}