use crate::handle::{Handle, Handled};
use crate::handle::order::OrderlyHandled;

pub struct ProductionRule<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: Handled,
{
    pub(super) lhs: Handle<Nonterminal>,
    pub(super) rhs: Vec<Symbol<Terminal, Nonterminal>>,
    pub(super) tag: Handle<Tag>,
    binding: Option<Handle<Binding<Terminal>>>,
}

impl<Terminal, Nonterminal, Tag> ProductionRule<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: Handled,
{
    pub fn new(
        lhs: Handle<Nonterminal>, rhs: Vec<Symbol<Terminal, Nonterminal>>, tag: Handle<Tag>,
        binding: Option<Handle<Binding<Terminal>>>,
    ) -> Self {
        Self { lhs, rhs, tag, binding }
    }
}

impl<Terminal, Nonterminal, Tag> Handled for ProductionRule<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: Handled,
{
    type HandleCoreType = Tag::HandleCoreType;
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

pub struct Binding<Terminal: Handled> {
    pub(super) terminals: Vec<Handle<Terminal>>,
    associativity: Associativity,
}

impl<Terminal> Binding<Terminal>
where
    Terminal: Handled,
{
    pub fn new(terminals: Vec<Handle<Terminal>>, associativity: Associativity) -> Self {
        Self { terminals, associativity }
    }
}

impl<Terminal: Handled> Handled for Binding<Terminal> { type HandleCoreType = u8; }
impl<Terminal: Handled> OrderlyHandled for Binding<Terminal> {}
