pub mod build;
mod lookaheads;
mod compile_to_parser;
mod firsts;

use std::collections::HashSet;
use derive_where::derive_where;
use crate::automata::dfa::{Dfa, DfaState};
use crate::handle::{Handle, Handled};
use crate::handle::handled_vec::HandledVec;
use crate::handle::order::OrderlyHandled;
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

pub struct KernelSetEntry<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    item: Item<Terminal, Nonterminal, Tag>,
    lookaheads: HashSet<Handle<Terminal>>,
    propagations:
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
