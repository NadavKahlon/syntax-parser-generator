use crate::handle::{Handle, Handled};
use crate::handle::handled_vec::HandledVec;
use crate::parsing::lr_parser::execute::LrParserDecision;
use crate::parsing::lr_parser::LrParser;
use crate::parsing::translator::build::Nonterminal;
use crate::parsing::translator::atomic_translator::AtomicTranslator;

pub mod build;
mod atomic_translator;
#[cfg(test)]
mod tests;

pub struct SyntaxDirectedTranslator<Terminal, Satellite>
where
    Terminal: Handled,
{
    lr_parser: LrParser<Terminal, Nonterminal, AtomicTranslator<Satellite>>,
    atomic_translators: HandledVec<AtomicTranslator<Satellite>>,
}

impl<Terminal, Satellite> SyntaxDirectedTranslator<Terminal, Satellite>
where
    Terminal: Handled,
{
    pub fn translate(
        &self, mut stream: impl Iterator<Item=(Handle<Terminal>, Satellite)>,
    ) -> Option<Satellite>
    {
        let mut satellite_stack: Vec<Satellite> = Vec::new();
        let mut lr_parser_execution = self.lr_parser.new_execution();
        let mut next_input_pair: Option<(Handle<Terminal>, Satellite)> = None;

        loop {
            next_input_pair = match next_input_pair {
                None => {
                    match stream.next() {
                        None => break,
                        Some(input_pair) => Some(input_pair),
                    }
                }

                Some((terminal, satellite)) => {
                    match lr_parser_execution.decide(terminal)? {
                        LrParserDecision::Shift => {
                            satellite_stack.push(satellite);
                            None
                        },
                        LrParserDecision::Reduce { size, tag } => {
                            if satellite_stack.len() < size {
                                // Satellite stack is too short for rule
                                return None;
                            }
                            let rhs_satellites = satellite_stack
                                .drain((satellite_stack.len() - size)..).collect();
                            let lhs_satellite =
                                self.atomic_translators[tag].translate(rhs_satellites);
                            satellite_stack.push(lhs_satellite);

                            // Do not reset next_input_pair as Reduce does not consume input
                            Some((terminal, satellite))
                        }
                    }
                }
            }
        }

        // We reach here on input exhaustion
        if (satellite_stack.len() == 1) && lr_parser_execution.finalize() {
            satellite_stack.pop()
        } else {
            None
        }
    }
}
