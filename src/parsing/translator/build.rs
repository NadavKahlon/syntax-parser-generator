use crate::handle::{Handle, Handled};
use crate::handle::handled_vec::HandledVec;
use crate::parsing::lr_parser::build::LrParserBuilder;
use crate::parsing::lr_parser::rules::{Associativity, Binding, Symbol};
use crate::parsing::translator::reduction_handler::ReductionHandler;
use crate::parsing::translator::SyntaxDirectedTranslator;

pub struct SyntaxDirectedTranslatorBuilder<Terminal, Satellite, F>
where
    Terminal: Handled,
    F: Fn(Vec<&Satellite>) -> Satellite,
{
    nonterminals: HandledVec<Nonterminal>,
    reduction_handlers: HandledVec<ReductionHandler<Satellite, F>>,
    lr_parser_builder: LrParserBuilder<Terminal, Nonterminal, ReductionHandler<Satellite, F>>,
}

impl<Terminal, Satellite, F> SyntaxDirectedTranslatorBuilder<Terminal, Satellite, F>
where
    Terminal: Handled,
    F: Fn(Vec<&Satellite>) -> Satellite,
{
    pub fn new() -> Self {
        Self {
            nonterminals: HandledVec::new(),
            reduction_handlers: HandledVec::new(),
            lr_parser_builder: LrParserBuilder::new(),
        }
    }

    pub fn new_nonterminal(&mut self) -> Handle<Nonterminal> {
        self.nonterminals.insert(Nonterminal)
    }

    pub fn register_binding(&mut self, associativity: Associativity) -> Handle<Binding> {
        self.lr_parser_builder.register_binding(associativity)
    }

    pub fn register_rule(
        &mut self,
        lhs: Handle<Nonterminal>,
        rhs: Vec<Symbol<Terminal, Nonterminal>>,
        binding: Option<Handle<Binding>>,
        handler: F,
    ) {
        let tag =
            self.reduction_handlers.insert(ReductionHandler::new(handler));
        self.lr_parser_builder.register_rule(lhs, rhs, binding, tag);
    }

    pub fn build(self) -> SyntaxDirectedTranslator<Terminal, Satellite, F> {
        let Self { reduction_handlers, lr_parser_builder, ..} = self;
        SyntaxDirectedTranslator {
            lr_parser: lr_parser_builder.build(),
            reduction_handlers,
        }
    }
}

// Blank, don't really need to carry any info, Handle API is only used for counting registrations
pub struct Nonterminal;
impl Handled for Nonterminal { type HandleCoreType = u8; }

