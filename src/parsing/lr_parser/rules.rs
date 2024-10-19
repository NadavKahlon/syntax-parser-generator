use derive_where::derive_where;

use crate::handles::{Handle, Handled};
use crate::handles::specials::OrderlyHandled;

#[derive_where(Debug)]
pub struct ProductionRule<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: Handled,
{
    pub(super) lhs: Handle<Nonterminal>,
    pub(super) rhs: Vec<GrammarSymbol<Terminal, Nonterminal>>,
    pub(super) tag: Handle<Tag>,
    pub(super) binding: Option<Handle<Binding<Terminal>>>,
}

impl<Terminal, Nonterminal, Tag> ProductionRule<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: Handled,
{
    pub fn new(
        lhs: Handle<Nonterminal>,
        rhs: Vec<GrammarSymbol<Terminal, Nonterminal>>,
        tag: Handle<Tag>,
        binding: Option<Handle<Binding<Terminal>>>,
    ) -> Self {
        Self {
            lhs,
            rhs,
            tag,
            binding,
        }
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

#[derive_where(Debug, Clone, Copy)]
pub enum GrammarSymbol<Terminal: Handled, Nonterminal: Handled> {
    Terminal(Handle<Terminal>),
    Nonterminal(Handle<Nonterminal>),
}

impl<Terminal: Handled, Nonterminal: Handled> Handled for GrammarSymbol<Terminal, Nonterminal> {
    type HandleCoreType = u16;
}

/// Represents the associativity of a binding of a grammar's terminals and production rules.
///
/// See [syntaxDirectedTranslatorBuilder](crate::parsing::SyntaxDirectedTranslatorBuilder) for more
/// details.
pub enum Associativity {
    /// Represents left-associated binding.
    ///
    /// For example - the addition operator's binding is left-associated.
    Left,

    /// Represents right-associated binding.
    ///
    /// For example - the unary minus operator's binding is right-associated.
    Right,

    /// Represents bindings that cannot be associated to either left or right.
    None,
}

pub struct Binding<Terminal: Handled> {
    pub(super) terminals: Vec<Handle<Terminal>>,
    pub(super) associativity: Associativity,
}

impl<Terminal> Binding<Terminal>
where
    Terminal: Handled,
{
    pub fn new(terminals: Vec<Handle<Terminal>>, associativity: Associativity) -> Self {
        Self {
            terminals,
            associativity,
        }
    }
}

impl<Terminal: Handled> Handled for Binding<Terminal> {
    type HandleCoreType = u8;
}
impl<Terminal: Handled> OrderlyHandled for Binding<Terminal> {}
