use crate::handle::Handled;
use crate::handle::order::OrderlyHandled;
use crate::parsing::lr_parser::LrParser;
use crate::parsing::lr_parser::rules::ProductionRule;

impl<Terminal, Nonterminal, Tag> LrParser<Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    fn build(_rules: Vec<ProductionRule<Terminal, Nonterminal, Tag>>) -> Self
    {
        todo!()
    }
}