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

pub enum Symbol<Terminal: Handled, Nonterminal: Handled> {
    Terminal(Handle<Terminal>),
    Nonterminal(Handle<Nonterminal>),
}

pub enum Associativity {
    Left,
    Right,
}

struct Binding {
    associativity: Option<Associativity>,
}

impl Handled for Binding { type HandleCoreType = u8; }
impl OrderlyHandled for Binding {}
