pub mod build;
mod lookaheads;
mod compile_to_parser;
mod firsts;

use std::collections::HashSet;
use derive_where::derive_where;
use crate::automata::dfa::{Dfa, DfaState};
use crate::handles::{Handle, Handled};
use crate::handles::collections::HandledVec;
use crate::handles::specials::OrderlyHandled;
use crate::parsing::lr_parser::rules::{GrammarSymbol, ProductionRule};

pub type KernelSetsDfaState<Terminal, Nonterminal, Tag> =
DfaState<GrammarSymbol<Terminal, Nonterminal>, KernelSet<Terminal, Nonterminal, Tag>>;

pub type KernelSetsDfa<Terminal, Nonterminal, Tag> =
Dfa<GrammarSymbol<Terminal, Nonterminal>, KernelSet<Terminal, Nonterminal, Tag>>;

impl<Terminal, Nonterminal, Tag> KernelSetsDfa<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    pub fn kernel_set(&self, state: Handle<KernelSetsDfaState<Terminal, Nonterminal, Tag>>)
                      -> &KernelSet<Terminal, Nonterminal, Tag>
    {
        self.get_label(state).as_ref().expect(
            "All states of a KernelSetsDfa should be labeled by the corresponding KernelSet"
        )
    }

    pub fn mut_kernel_set(&mut self, state: Handle<KernelSetsDfaState<Terminal, Nonterminal, Tag>>)
                          -> &mut KernelSet<Terminal, Nonterminal, Tag>
    {
        self.get_label_mut(state).expect(
            "All states of a KernelSetsDfa should be labeled by the corresponding KernelSet"
        )
    }
}

#[derive_where(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct Item<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    pub rule: Handle<ProductionRule<Terminal, Nonterminal, Tag>>,
    pub dot: usize,
}

#[derive_where(Debug)]
pub struct KernelSet<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    pub entries: HandledVec<KernelSetEntry<Terminal, Nonterminal, Tag>>,
}

impl<Terminal, Nonterminal, Tag> KernelSet<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    pub fn new(kernel_items: impl Iterator<Item=Item<Terminal, Nonterminal, Tag>>) -> Self {
        Self {
            entries: HandledVec::from_iter(kernel_items.map(KernelSetEntry::new)),
        }
    }
}

#[derive_where(Debug)]
pub struct KernelSetEntry<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    pub item: Item<Terminal, Nonterminal, Tag>,
    pub lookaheads: HashSet<Handle<Terminal>>,
    pub propagations:
        Vec<(
            Handle<KernelSetsDfaState<Terminal, Nonterminal, Tag>>,
            Handle<KernelSetEntry<Terminal, Nonterminal, Tag>>,
        )>,
}

impl<Terminal, Nonterminal, Tag> KernelSetEntry<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    fn new(kernel_item: Item<Terminal, Nonterminal, Tag>) -> Self {
        Self {
            item: kernel_item,
            lookaheads: HashSet::new(),
            propagations: Vec::new(),
        }
    }
}

impl<Terminal, Nonterminal, Tag> Handled for KernelSetEntry<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    type HandleCoreType = u16;
}
