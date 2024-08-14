use crate::handle::{Handled};
use crate::handle::handled_vec::HandledVec;
use crate::parsing::lr_parser::LrParser;
use crate::parsing::translator::build::Nonterminal;
use crate::parsing::translator::atomic_translator::AtomicTranslator;

pub mod build;
mod atomic_translator;
pub mod execution;
#[cfg(test)]
mod tests;

pub struct SyntaxDirectedTranslator<Terminal, Satellite>
where
    Terminal: Handled,
{
    lr_parser: LrParser<Terminal, Nonterminal, AtomicTranslator<Satellite>>,
    atomic_translators: HandledVec<AtomicTranslator<Satellite>>,
}
