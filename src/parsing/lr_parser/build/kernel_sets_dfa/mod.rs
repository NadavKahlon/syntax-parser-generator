pub mod build;

use derive_where::derive_where;
use crate::automata::dfa::{Dfa, DfaState};
use crate::handle::{Handle, Handled};
use crate::handle::handled_vec::HandledVec;
use crate::handle::order::OrderlyHandled;
use crate::parsing::lr_parser::rules::{GrammarSymbol, ProductionRule};

pub type KernelSetsDfaState<Terminal, Nonterminal, Tag> =
DfaState<GrammarSymbol<Terminal, Nonterminal>, KernelItemsSet<Terminal, Nonterminal, Tag>>;

pub type KernelSetsDfa<Terminal, Nonterminal, Tag> =
Dfa<GrammarSymbol<Terminal, Nonterminal>, KernelItemsSet<Terminal, Nonterminal, Tag>>;

#[derive_where(Hash, Clone, Copy, PartialEq, Eq)]
pub struct Item<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    pub rule: Handle<ProductionRule<Terminal, Nonterminal, Tag>>,
    pub dot: usize,
}

impl<Terminal, Nonterminal, Tag> Item<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    pub fn is_kernel_item(&self, start_rule: Handle<ProductionRule<Terminal, Nonterminal, Tag>>)
                          -> bool
    {
        (self.dot != 0) || (self.rule == start_rule)
    }
}

pub struct KernelItemsSet<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    entries: HandledVec<KernelItemsSetEntry<Terminal, Nonterminal, Tag>>,
}

impl<Terminal, Nonterminal, Tag> KernelItemsSet<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    pub fn new(kernel_items: impl Iterator<Item=Item<Terminal, Nonterminal, Tag>>) -> Self {
        Self {
            entries: HandledVec::from_iter(kernel_items.map(KernelItemsSetEntry::new)),
        }
    }
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
            Handle<KernelSetsDfaState<Terminal, Nonterminal, Tag>>,
            Handle<KernelItemsSetEntry<Terminal, Nonterminal, Tag>>,
            Handle<Terminal>,
        )>,
}

impl<Terminal, Nonterminal, Tag> KernelItemsSetEntry<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    fn new(kernel_item: Item<Terminal, Nonterminal, Tag>) -> Self {
        Self {
            item: kernel_item,
            lookaheads: Vec::new(),
            propagations: Vec::new(),
        }
    }
}

impl<Terminal, Nonterminal, Tag> Handled for KernelItemsSetEntry<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    type HandleCoreType = u16;
}
