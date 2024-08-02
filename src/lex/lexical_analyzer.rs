use std::collections::HashMap;
use std::hash::Hash;
use crate::automata::dfa::Dfa;
use crate::automata::nfa::Nfa;
use crate::handle::auto::AutomaticallyHandled;
use crate::lex::lexeme_iterator::LexemeIterator;
use crate::lex::{Lexeme, LexemeDescriptor};
use crate::reader::Reader;


pub struct LexicalAnalyzer<LexemeType>
{
    dfa: Dfa<u8, LexemeType>,
}

// Earlier lexeme descriptors are prioritized
impl<LexemeType> LexicalAnalyzer<LexemeType>
where
    LexemeType: Hash + Eq + Clone,
{
    pub fn new(lexeme_descriptors: Vec<LexemeDescriptor<LexemeType>>) -> LexicalAnalyzer<LexemeType>
    {
        let mut nfa = Nfa::new();
        let global_start_state = nfa.new_state();
        nfa.set_initial_state(global_start_state);

        let mut priority_map = HashMap::new();

        for (priority, LexemeDescriptor { pattern, lexeme_type })
        in lexeme_descriptors.iter().enumerate()
        {
            let (pattern_start_state, pattern_end_state) =
                pattern.build_into_nfa(&mut nfa);
            nfa.link(global_start_state, pattern_start_state, None);
            nfa.label(pattern_end_state, Some(lexeme_type.clone()));

            priority_map.insert(lexeme_type, priority);
        }

        let dfa = nfa
            .compile_to_dfa(|labels| {
                labels.iter().min_by_key(|&&lexeme_type| {
                    priority_map.get(lexeme_type)
                }).cloned().cloned()
            })
            .minimize();

        // Make initial state is unlabeled, so we won't get stuck on epsilon when input is exhausted
        let initial_state = dfa.get_initial_state().expect(
            "Minimized DFA should have an initial state, as the associated NFA had one, and \
            the initial state "
        );
        if !dfa.get_label(initial_state).is_none() {
            panic!(
                "Tried to create a lexical analyzer where some lexeme type is associated with a \
                regex accepting the empty string, which will make it get stuck on input exhaustion"
            )
        }

        LexicalAnalyzer { dfa }
    }

    pub fn analyze<'a>(&'a self, reader: &'a mut impl Reader<u8>)
                       -> impl Iterator<Item=Lexeme<LexemeType>> + 'a
    {
        LexemeIterator::new(self, reader)
    }

    fn identify_next_lexeme(&self, reader: &mut impl Reader<u8>)
                            -> LexemeIdentificationResult<LexemeType>
    {
        let mut recent_lexeme_type: Option<LexemeType> = None;
        let mut current_state = self.dfa.get_initial_state();

        let mut is_string_empty = true;

        loop {
            if current_state.is_none() || !reader.is_available() {
                return if is_string_empty {
                    LexemeIdentificationResult::InputExhausted
                } else if let Some(lexeme_type) = recent_lexeme_type {
                    LexemeIdentificationResult::Identified(lexeme_type)
                } else {
                    // We read some data, but couldn't identify available prefix
                    LexemeIdentificationResult::LexicalError
                };
            }

            let actual_current_state = current_state.expect(
                "Current state should not be None, as it was just checked"
            );
            if let Some(lexeme_type) = self.dfa.get_label(actual_current_state) {
                recent_lexeme_type = Some(lexeme_type.clone());
                reader.set_tail();
            }

            let next_byte = reader.read_next().expect(
                "Reader should not have been exhausted, since we just checked its availability"
            );
            current_state = self.dfa.step(actual_current_state, next_byte.handle());
            is_string_empty = false;
        }
    }

    pub(super) fn collect_next_lexeme(&self, reader: &mut impl Reader<u8>)
                                      -> Option<Lexeme<LexemeType>>
    {
        let lexeme_type = loop {
            match self.identify_next_lexeme(reader) {
                LexemeIdentificationResult::Identified(lexeme_type) => {
                    break lexeme_type
                }
                LexemeIdentificationResult::InputExhausted => {
                    return None
                }
                LexemeIdentificationResult::LexicalError => {
                    self.error_recovery_routine(reader);
                }
            }
        };

        let contents = String::from_utf8(reader.get_sequence().collect()).expect(
            "Tokens from lexically-analyzed Reader<u8> are expected to be UTF-8 encoded"
        );
        let lexeme = Lexeme { lexeme_type, contents };
        reader.restart_from_tail();
        Some(lexeme)
    }

    fn error_recovery_routine(&self, _reader: &mut impl Reader<u8>) {
        // TODO make this configurable
        panic!("Reader had a lexical error in it, and error recovery is not yet implemented");
    }
}

enum LexemeIdentificationResult<LexemeType> {
    Identified(LexemeType),
    InputExhausted,
    LexicalError,
}
