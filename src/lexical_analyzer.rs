use std::collections::HashMap;
use crate::automata::labeled_dfa::{DfaLabel, LabeledDfa};
use crate::automata::nfa::NfaBuilder;
use crate::reader::Reader;
use crate::regex::Regex;

struct LexemeDescriptor<T> {
    pattern: Regex,
    lexeme_type: T,
}

struct Lexeme<T> {
    lexeme_type: T,
    contents: String,
}

// TODO doc: maximum of 256 symbols
struct LexicalAnalyzer<T>
where
    T: Clone,
{
    labeled_dfa: LabeledDfa,
    lexeme_types_for_dfa_labels: HashMap<DfaLabel, T>,
}

// Earlier lexeme descriptors are prioritized
// TODO doc maximum number of patterns is 255 (not 256!)
impl<T> LexicalAnalyzer<T>
where
    T: Clone,
{
    pub fn new(lexeme_descriptors: Vec<LexemeDescriptor<T>>) -> LexicalAnalyzer<T> {
        let mut nfa_builder = NfaBuilder::new(u8::MAX as u16);
        let global_start_state = nfa_builder.new_state();
        let mut lexeme_types_for_dfa_labels = HashMap::new();

        let mut nfa_state_to_label_map = HashMap::new();
        for (pattern_index, LexemeDescriptor { pattern, lexeme_type })
        in lexeme_descriptors.iter().enumerate() {

            // Create DFA label for pattern
            // We add 1 to the pattern index, to reserve label 0 for "no pattern associated"
            let label = DfaLabel((pattern_index + 1) as u8); // Possible type confusion
            lexeme_types_for_dfa_labels.insert(label, lexeme_type.clone());

            // Build pattern into the nfa
            let (pattern_start_state, pattern_end_state) =
                pattern.build_into_nfa(&mut nfa_builder);
            nfa_builder.link(global_start_state, pattern_start_state, None);
            nfa_state_to_label_map.insert(pattern_end_state, label);
        }

        // Compile the NFA to a DFA
        let nfa = nfa_builder.build(global_start_state);
        let (dfa, dfa_to_nfa_states_map)
            = nfa.compile_to_dfa();

        // Label DFA states according to the first pattern associated with them
        let mut unoptimized_labeled_dfa = LabeledDfa::new(dfa);
        for (&dfa_state, associated_nfa_states)
        in dfa_to_nfa_states_map.iter() {
            let optional_label = associated_nfa_states
                .iter()
                .map(|nfa_state| nfa_state_to_label_map.get(nfa_state))
                .flatten()
                .filter(|x| x.0 != 0)
                .min(); // We pick the minimum label to prioritize earlier lexeme descriptions
            if let Some(&label) = optional_label {
                unoptimized_labeled_dfa.label(dfa_state, label)
            }
        }

        // Optimize the DFA (minimize its number of states)
        let labeled_dfa = unoptimized_labeled_dfa.minimize();

        LexicalAnalyzer { labeled_dfa, lexeme_types_for_dfa_labels }
    }
}

// TODO add tests