use crate::handle::Handled;
use crate::handle::handled_vec::HandledVec;
use crate::parsing::lr_parser::LrParser;
use crate::parsing::translator::build::Nonterminal;
use crate::parsing::translator::reduction_handler::ReductionHandler;

pub mod build;
mod reduction_handler;

pub struct SyntaxDirectedTranslator<Terminal, Satellite, F>
where
    Terminal: Handled,
    F: Fn(Vec<&Satellite>) -> Satellite,
{
    lr_parser: LrParser<Terminal, Nonterminal, ReductionHandler<Satellite, F>>,
    reduction_handlers: HandledVec<ReductionHandler<Satellite, F>>,
}

impl<Terminal, Satellite, F> SyntaxDirectedTranslator<Terminal, Satellite, F>
where
    Terminal: Handled,
    F: Fn(Vec<&Satellite>) -> Satellite,
{
}