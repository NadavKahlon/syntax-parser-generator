use crate::handle::{Handle, Handled};
use crate::handle::handled_vec::HandledVec;
use crate::handle::order::OrderlyHandled;
use crate::parsing::lr_parser::rules::ProductionRule;

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