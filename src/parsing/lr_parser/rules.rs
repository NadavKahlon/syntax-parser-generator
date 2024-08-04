use crate::handle::{Handle, Handled};
use crate::handle::order::OrderlyHandled;

pub struct ProductionRule<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: Handled,
{
    lhs: Handle<Nonterminal>,
    rhs: Vec<Symbol<Terminal, Nonterminal>>,
    tag: Handle<Tag>,
    binding: Option<Handle<Binding>>,
}

impl<Terminal, Nonterminal, Tag> ProductionRule<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: Handled,
{
    pub fn new(
        lhs: Handle<Nonterminal>, rhs: Vec<Symbol<Terminal, Nonterminal>>, tag: Handle<Tag>,
        binding: Option<Handle<Binding>>,
    ) -> Self {
        Self { lhs, rhs, tag, binding }
    }
}

pub enum Symbol<Terminal: Handled, Nonterminal: Handled> {
    Terminal(Handle<Terminal>),
    Nonterminal(Handle<Nonterminal>),
}

pub enum Associativity {
    Left,
    Right,
    None,
}

pub struct Binding {
    associativity: Associativity,
}

impl Binding {
    pub fn new(associativity: Associativity) -> Self {
        Self { associativity }
    }
}

impl Handled for Binding { type HandleCoreType = u8; }
impl OrderlyHandled for Binding {}
