use crate::handle::{Handle, Handled};
use crate::handle::handled_vec::HandledVec;
use crate::parsing::lr_parser::build::LrParserBuilder;
use crate::parsing::lr_parser::rules::{Associativity, Binding, GrammarSymbol};
use crate::parsing::translator::atomic_translator::AtomicTranslator;
use crate::parsing::translator::SyntaxDirectedTranslator;

pub struct SyntaxDirectedTranslatorBuilder<Terminal, Satellite>
where
    Terminal: Handled,
{
    nonterminals: HandledVec<Nonterminal>,
    atomic_translators: HandledVec<AtomicTranslator<Satellite>>,
    lr_parser_builder: LrParserBuilder<Terminal, Nonterminal, AtomicTranslator<Satellite>>,
}

impl<Terminal, Satellite> SyntaxDirectedTranslatorBuilder<Terminal, Satellite>
where
    Terminal: Handled,
{
    pub fn new() -> Self {
        Self {
            nonterminals: HandledVec::new(),
            atomic_translators: HandledVec::new(),
            lr_parser_builder: LrParserBuilder::new(),
        }
    }

    pub fn new_nonterminal(&mut self) -> Handle<Nonterminal> {
        self.nonterminals.insert(Nonterminal)
    }

    pub fn register_binding(
        &mut self, terminals: Vec<Handle<Terminal>>, associativity: Associativity
    ) -> Handle<Binding<Terminal>>
    {
        self.lr_parser_builder.register_binding(terminals, associativity)
    }

    pub fn register_rule(
        &mut self,
        lhs: Handle<Nonterminal>,
        rhs: Vec<GrammarSymbol<Terminal, Nonterminal>>,
        handler: Box<dyn Fn(Vec<Satellite>) -> Option<Satellite>>,
    ) {
        self.register_rule_raw(lhs, rhs, handler, None);
    }

    pub fn register_bound_rule(
        &mut self,
        lhs: Handle<crate::parsing::translator::build::Nonterminal>,
        rhs: Vec<GrammarSymbol<Terminal, crate::parsing::translator::build::Nonterminal>>,
        handler: Box<dyn Fn(Vec<Satellite>) -> Option<Satellite>>,
        binding: Handle<Binding<Terminal>>,
    ) {
        self.register_rule_raw(lhs, rhs, handler, Some(binding))
    }

    pub fn set_start_nonterminal(&mut self, nonterminal: Handle<Nonterminal>) {
        self.lr_parser_builder.set_start_nonterminal(nonterminal)
    }

    pub fn build(self) -> SyntaxDirectedTranslator<Terminal, Satellite> {
        let Self {
            atomic_translators,
            lr_parser_builder,
            ..
        } = self;
        SyntaxDirectedTranslator {
            lr_parser: lr_parser_builder.build(),
            atomic_translators,
        }
    }

    fn register_rule_raw(
        &mut self,
        lhs: Handle<Nonterminal>,
        rhs: Vec<GrammarSymbol<Terminal, Nonterminal>>,
        handler: Box<dyn Fn(Vec<Satellite>) -> Option<Satellite>>,
        optional_binding: Option<Handle<Binding<Terminal>>>,
    ) {
        let tag =
            self.atomic_translators.insert(AtomicTranslator::new(handler));
        self.lr_parser_builder.register_rule(lhs, rhs, optional_binding, tag);
    }
}

// Blank, don't really need to carry any info, Handle API is only used for counting registrations
pub struct Nonterminal;
impl Handled for Nonterminal { type HandleCoreType = u8; }
