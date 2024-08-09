use crate::handle::Handled;
use crate::handle::order::OrderlyHandled;
use crate::parsing::lr_parser::build::kernel_sets_dfa::KernelSetsDfa;
use crate::parsing::lr_parser::LrParser;

impl<Terminal, Nonterminal, Tag> KernelSetsDfa<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    pub fn compile_to_parser(&self) -> LrParser<Terminal, Nonterminal, Tag> {
        todo!()
    }
}
