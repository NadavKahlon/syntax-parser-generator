use crate::handle::{Handle, Handled};
use crate::parsing::lr_parser::execute::{FinalDecision, LrParserDecision, LrParserExecution};
use crate::parsing::translator::atomic_translator::AtomicTranslator;
use crate::parsing::translator::build::Nonterminal;
use crate::parsing::translator::SyntaxDirectedTranslator;

impl<Terminal, Satellite> SyntaxDirectedTranslator<Terminal, Satellite>
where
    Terminal: Handled,
{
    pub fn translate(
        &self, stream: impl Iterator<Item=(Handle<Terminal>, Satellite)>,
    ) -> Option<Satellite>
    {
        let mut execution =
            SyntaxDirectedTranslatorExecution::new(self);
        for (terminal, satellite) in stream {
            execution.feed(terminal, satellite)?;
        }
        execution.finalize()
    }
}

struct SyntaxDirectedTranslatorExecution<'a, Terminal, Satellite>
where
    Terminal: Handled,
{
    translator: &'a SyntaxDirectedTranslator<Terminal, Satellite>,
    satellite_stack: Vec<Satellite>,
    lr_parser_execution: LrParserExecution<'a, Terminal, Nonterminal, AtomicTranslator<Satellite>>,
}

impl<'a, Terminal, Satellite> SyntaxDirectedTranslatorExecution<'a, Terminal, Satellite>
where
    Terminal: Handled,
{
    fn new(translator: &'a SyntaxDirectedTranslator<Terminal, Satellite>) -> Self
    {
        Self {
            translator,
            satellite_stack: Vec::new(),
            lr_parser_execution: translator.lr_parser.new_execution(),
        }
    }

    fn feed(&mut self, terminal: Handle<Terminal>, satellite: Satellite) -> Option<()> {
        loop {
            match self.lr_parser_execution.decide(terminal)? {
                LrParserDecision::Reduce { size, tag } => {
                    self.handle_reduce(size, tag)?
                }
                LrParserDecision::Shift => {
                    self.satellite_stack.push(satellite);
                    break
                }
            }
        }
        Some(())
    }

    fn finalize(mut self) -> Option<Satellite> {
        while let FinalDecision::Reduce { size, tag } =
            self.lr_parser_execution.decide_final()?
        {
            self.handle_reduce(size, tag);
        }
        self.satellite_stack.pop()
    }

    fn handle_reduce(&mut self, size: usize, tag: Handle<AtomicTranslator<Satellite>>) -> Option<()> {
        if self.satellite_stack.len() < size {
            // Satellite stack is too short for rule
            return None;
        }
        let rhs_satellites = self.satellite_stack
            .drain((self.satellite_stack.len() - size)..).collect();
        let lhs_satellite =
            self.translator.atomic_translators[tag].translate(rhs_satellites);
        self.satellite_stack.push(lhs_satellite);
        Some(())
    }
}
