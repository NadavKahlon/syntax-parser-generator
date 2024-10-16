use std::fmt::Debug;
use crate::handles::{Handle, Handled};
use crate::handles::collections::{HandledVec, HandleMap};
use crate::handles::specials::AutomaticallyHandled;
use crate::lex::Lexeme;
use crate::parsing::lr_parser::execute::{FinalDecision, LrParserDecision, LrParserExecution};
use crate::parsing::lr_parser::LrParser;
use crate::parsing::translator::build::Nonterminal;
use crate::parsing::translator::handlers::{LeafSatelliteBuilder, SatelliteReducer};

pub struct SyntaxDirectedTranslator<LexemeType: Handled, Context, Satellite> {
    pub(super) lr_parser: LrParser<LexemeType, Nonterminal, SatelliteReducer<Context, Satellite>>,
    pub(super) default_leaf_satellite_builder: Option<LeafSatelliteBuilder<Context, Satellite>>,
    pub(super) leaf_satellite_builder_map: HandleMap<LexemeType, LeafSatelliteBuilder<Context, Satellite>>,
    pub(super) satellite_reducers: HandledVec<SatelliteReducer<Context, Satellite>>,
}

impl<LexemeType: Debug, Context, Satellite> SyntaxDirectedTranslator<LexemeType, Context, Satellite>
where
    LexemeType: AutomaticallyHandled,
{
    pub fn translate(&self, context: &mut Context, stream: impl Iterator<Item=Lexeme<LexemeType>>)
                     -> Option<Satellite>
    {
        let mut execution =
            SyntaxDirectedTranslatorExecution::new(self, context);
        for lexeme in stream {
            execution.feed(lexeme)?;
        }
        execution.finalize()
    }

    fn build_leaf(&self, context: &mut Context, lexeme: Lexeme<LexemeType>)
                  -> (Handle<LexemeType>, Satellite)
    {
        let terminal = lexeme.lexeme_type.handle();
        let builder =
            if let Some(builder) =
                self.leaf_satellite_builder_map.get(lexeme.lexeme_type.handle())
            {
                builder
            } else if let Some(builder)
                = &self.default_leaf_satellite_builder
            {
                builder
            } else {
                panic!(
                    "Tried to build a leaf satellite for a lexeme type for which no leaf satellite \
                    builder was specified, and no default builder was set"
                )
            };
        (terminal, builder(context, lexeme.contents))
    }

    fn reduce_satellites(
        &self, reducer: Handle<SatelliteReducer<Context, Satellite>>,
        context: &mut Context, satellites: Vec<Satellite>,
    ) -> Satellite
    {
        self.satellite_reducers[reducer](context, satellites)
    }
}

struct SyntaxDirectedTranslatorExecution<'a, LexemeType: AutomaticallyHandled, Context, Satellite>
{
    translator: &'a SyntaxDirectedTranslator<LexemeType, Context, Satellite>,
    context: &'a mut Context,
    satellite_stack: Vec<Satellite>,
    lr_parser_execution:
        LrParserExecution<'a, LexemeType, Nonterminal, SatelliteReducer<Context, Satellite>>,
}

impl<'a, LexemeType: AutomaticallyHandled + Debug, Context, Satellite>
SyntaxDirectedTranslatorExecution<'a, LexemeType, Context, Satellite>
{
    fn new(
        translator: &'a SyntaxDirectedTranslator<LexemeType, Context, Satellite>,
        context: &'a mut Context,
    ) -> Self {
        Self {
            translator,
            context,
            satellite_stack: Vec::new(),
            lr_parser_execution: translator.lr_parser.new_execution(),
        }
    }

    fn feed(&mut self, lexeme: Lexeme<LexemeType>) -> Option<()> {
        let (terminal, satellite)
            = self.translator.build_leaf(self.context, lexeme);
        loop {
            match self.lr_parser_execution.decide(terminal)? {
                LrParserDecision::Reduce { size, tag } => {
                    self.handle_reduce(size, tag)?
                }
                LrParserDecision::Shift => {
                    self.satellite_stack.push(satellite);
                    break;
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

    fn handle_reduce(
        &mut self, size: usize, reducer: Handle<SatelliteReducer<Context, Satellite>>,
    ) -> Option<()>
    {
        if self.satellite_stack.len() < size {
            // Satellite stack is too short for rule
            return None;
        }
        let rhs_satellites = self.satellite_stack
            .drain((self.satellite_stack.len() - size)..).collect();
        let lhs_satellite =
            self.translator.reduce_satellites(reducer, self.context, rhs_satellites);
        self.satellite_stack.push(lhs_satellite);
        Some(())
    }
}