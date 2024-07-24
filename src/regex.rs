use crate::automata::{InputSymbol, nfa};
use crate::automata::nfa::NfaStateHandle;

#[derive(Clone)]
pub enum Regex {
    SingleCharacter { value: u8 },
    Union { options: Vec<Regex> },
    Concat { parts: Vec<Regex> },
    Star { repeated_pattern: Box<Regex> },
}

impl Regex {
    pub fn single_char(value: char) -> Regex {
        Regex::SingleCharacter { value: value.try_into().unwrap() }  // TODO unwrap
    }

    pub fn union(options: Vec<Regex>) -> Regex {
        Regex::Union { options }
    }

    pub fn concat(parts: Vec<Regex>) -> Regex {
        Regex::Concat { parts }
    }

    pub fn star_from(repeated_pattern: Regex) -> Regex {
        Regex::Star { repeated_pattern: Box::new(repeated_pattern) }
    }

    pub fn plus_from(repeated_pattern: Regex) -> Regex {
        let star_pattern = Regex::star_from(repeated_pattern.clone());
        Regex::concat(vec![
            repeated_pattern,
            star_pattern
        ])
    }

    pub fn white_space() -> Regex {
        let white_space_characters = vec![' ', '\t', '\n', '\r', '\x0B', '\x0C'];
        Regex::union(
            white_space_characters
                .into_iter()
                .map(Regex::single_char)
                .collect()
        )
    }

    pub fn constant_string(string: &str) -> Regex {
        Regex::concat(
            string
                .chars()
                .map(Regex::single_char)
                .collect()
        )
    }

    pub fn character_range(start: char, end: char) -> Regex {
        Regex::union(
            (start..=end)
                .map(Regex::single_char)
                .collect()
        )
    }

    pub fn optional(option: Regex) -> Regex {
        Regex::union(vec![
            option,
            Regex::epsilon(),
        ])
    }

    pub fn epsilon() -> Regex {
        Regex::concat(vec![])
    }

    pub fn build_into_nfa(
        &self, nfa_builder: &mut nfa::NfaBuilder,
    ) -> (NfaStateHandle, NfaStateHandle) {
        match self {
            Regex::SingleCharacter { value } => {
                let start = nfa_builder.new_state();
                let end = nfa_builder.new_state();
                let symbol = InputSymbol { id: *value as u16 };
                nfa_builder.link(start, end, Some(symbol));
                (start, end)
            }
            Regex::Union { options } => {
                let start = nfa_builder.new_state();
                let end = nfa_builder.new_state();
                for option in options {
                    let (option_start, option_end) =
                        option.build_into_nfa(nfa_builder);
                    nfa_builder.link(start, option_start, None);
                    nfa_builder.link(option_end, end, None);
                }
                (start, end)
            }
            Regex::Concat { parts } => {
                let start = nfa_builder.new_state();
                let end = nfa_builder.new_state();
                let mut curr = start;
                for part in parts {
                    let (part_start, part_end) =
                        part.build_into_nfa(nfa_builder);
                    nfa_builder.link(curr, part_start, None);
                    curr = part_end;
                }
                nfa_builder.link(curr, end, None);
                (start, end)
            }
            Regex::Star { repeated_pattern } => {
                let start = nfa_builder.new_state();
                let end = nfa_builder.new_state();
                let (repeated_pattern_start, repeated_pattern_end) =
                    repeated_pattern.build_into_nfa(nfa_builder);

                nfa_builder.link(start, repeated_pattern_start, None);
                nfa_builder.link(start, end, None);
                nfa_builder.link(repeated_pattern_end, end, None);
                nfa_builder.link(repeated_pattern_end, repeated_pattern_start, None);

                (start, end)
            }
        }
    }
}

// TODO tests
#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::automata::dfa::{Dfa, DfaStateHandle};
    use super::*;
    use crate::automata::nfa::NfaBuilder;

    fn create_dfa_for_regex(pattern: Regex) -> (Dfa, HashSet<DfaStateHandle>) {
        let mut nfa_builder = NfaBuilder::new(u8::MAX as u16);
        let (start, end) = pattern.build_into_nfa(&mut nfa_builder);
        let nfa = nfa_builder.build(start);
        let (dfa, dfa_states_map) =
            nfa.compile_to_dfa();

        let accepting_dfa_states = dfa_states_map
            .keys()
            .filter(
                |dfa_state| {
                    dfa_states_map.get(dfa_state).unwrap().contains(&end)
                }
            ).copied()
            .collect();

        return (dfa, accepting_dfa_states);
    }

    fn string_to_stream<'a>(data: &'a str) -> impl Iterator<Item=InputSymbol> + 'a {
        data.chars().map(|c| InputSymbol { id: c as u8 as u16 })
    }

    #[test]
    fn test_single_char() {
        let pattern = Regex::single_char('a');
        let (dfa, accepting_states) = create_dfa_for_regex(pattern);

        assert!(accepting_states.contains(&dfa.scan(string_to_stream("a"))));
        assert!(!accepting_states.contains(&dfa.scan(string_to_stream(""))));
        assert!(!accepting_states.contains(&dfa.scan(string_to_stream("aa"))));
    }

    #[test]
    fn test_union() {
        let pattern = Regex::union(vec![
            Regex::single_char('a'),
            Regex::single_char('b'),
            Regex::single_char('c'),
        ]);
        let (dfa, accepting_states) = create_dfa_for_regex(pattern);

        assert!(accepting_states.contains(&dfa.scan(string_to_stream("a"))));
        assert!(accepting_states.contains(&dfa.scan(string_to_stream("b"))));
        assert!(accepting_states.contains(&dfa.scan(string_to_stream("c"))));
        assert!(!accepting_states.contains(&dfa.scan(string_to_stream(""))));
        assert!(!accepting_states.contains(&dfa.scan(string_to_stream("aa"))));
        assert!(!accepting_states.contains(&dfa.scan(string_to_stream("d"))));
    }

    #[test]
    fn test_concat() {
        let pattern = Regex::concat(vec![
            Regex::single_char('a'),
            Regex::single_char('b'),
            Regex::single_char('c'),
        ]);
        let (dfa, accepting_states) = create_dfa_for_regex(pattern);

        assert!(accepting_states.contains(&dfa.scan(string_to_stream("abc"))));
        assert!(!accepting_states.contains(&dfa.scan(string_to_stream(""))));
        assert!(!accepting_states.contains(&dfa.scan(string_to_stream("a"))));
        assert!(!accepting_states.contains(&dfa.scan(string_to_stream("bc"))));
    }

    //noinspection ALL
    #[test]
    fn test_star() {
        let pattern = Regex::star_from(
            Regex::single_char('a'),
        );
        let (dfa, accepting_states) = create_dfa_for_regex(pattern);

        assert!(accepting_states.contains(&dfa.scan(string_to_stream(""))));
        assert!(accepting_states.contains(&dfa.scan(string_to_stream("a"))));
        assert!(accepting_states.contains(&dfa.scan(string_to_stream("aa"))));
        assert!(accepting_states.contains(&dfa.scan(string_to_stream("aaaaaaa"))));
        assert!(!accepting_states.contains(&dfa.scan(string_to_stream("b"))));
        assert!(!accepting_states.contains(&dfa.scan(string_to_stream("ab"))));
    }

    #[test]
    fn test_plus() {
        let pattern = Regex::plus_from(
            Regex::single_char('a'),
        );
        let (dfa, accepting_states) = create_dfa_for_regex(pattern);

        assert!(!accepting_states.contains(&dfa.scan(string_to_stream(""))));
        assert!(accepting_states.contains(&dfa.scan(string_to_stream("a"))));
        assert!(accepting_states.contains(&dfa.scan(string_to_stream("aa"))));
        assert!(accepting_states.contains(&dfa.scan(string_to_stream("aaaaaaa"))));
        assert!(!accepting_states.contains(&dfa.scan(string_to_stream("b"))));
        assert!(!accepting_states.contains(&dfa.scan(string_to_stream("ab"))));
    }

    #[test]
    fn test_complex() {
        let pattern = Regex::concat(vec![
            Regex::union(vec![
                Regex::character_range('a', 'z'),
                Regex::character_range('A', 'Z'),
                Regex::single_char('_'),
            ]),
            Regex::star_from(
                Regex::union(vec![
                    Regex::character_range('a', 'z'),
                    Regex::character_range('A', 'Z'),
                    Regex::character_range('0', '9'),
                    Regex::single_char('_'),
                ]),
            ),
        ]);
        let (dfa, accepting_states) = create_dfa_for_regex(pattern);

        assert!(accepting_states.contains(&dfa.scan(string_to_stream("MyThing"))));
        assert!(accepting_states.contains(&dfa.scan(string_to_stream("our_thing_12"))));
        assert!(accepting_states.contains(&dfa.scan(string_to_stream("i"))));
        assert!(accepting_states.contains(&dfa.scan(string_to_stream("a1jh2b45"))));
        assert!(!accepting_states.contains(&dfa.scan(string_to_stream(""))));
        assert!(!accepting_states.contains(&dfa.scan(string_to_stream("mine()"))));
        assert!(!accepting_states.contains(&dfa.scan(string_to_stream("12"))));
        assert!(!accepting_states.contains(&dfa.scan(string_to_stream("1ours"))));
    }
}