use crate::automata::nfa::{Nfa, NfaState};
use crate::handles::Handle;
use crate::handles::specials::AutomaticallyHandled;

#[derive(Clone)]
pub enum Regex {
    SingleCharacter { value: u8 },
    Union { options: Vec<Regex> },
    Concat { parts: Vec<Regex> },
    Star { repeated_pattern: Box<Regex> },
}

impl Regex {
    pub fn single_char(value: char) -> Regex {
        Regex::SingleCharacter {
            value: value.try_into().unwrap_or_else(|_| panic!(
                "Cannot create a single-character regex from {:?}, as it's not 1-byte long", value
            ))
        }
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
            star_pattern,
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

    pub fn build_into_nfa<Label>(
        &self, nfa: &mut Nfa<u8, Label>,
    ) -> (Handle<NfaState<u8, Label>>, Handle<NfaState<u8, Label>>)
    where
    {
        match self {
            Regex::SingleCharacter { value } => {
                let start = nfa.new_state();
                let end = nfa.new_state();
                nfa.link(start, end, Some(value.handle()));
                (start, end)
            }
            Regex::Union { options } => {
                let start = nfa.new_state();
                let end = nfa.new_state();
                for option in options {
                    let (option_start, option_end) =
                        option.build_into_nfa(nfa);
                    nfa.link(start, option_start, None);
                    nfa.link(option_end, end, None);
                }
                (start, end)
            }
            Regex::Concat { parts } => {
                let start = nfa.new_state();
                let end = nfa.new_state();
                let mut curr = start;
                for part in parts {
                    let (part_start, part_end) =
                        part.build_into_nfa(nfa);
                    nfa.link(curr, part_start, None);
                    curr = part_end;
                }
                nfa.link(curr, end, None);
                (start, end)
            }
            Regex::Star { repeated_pattern } => {
                let start = nfa.new_state();
                let end = nfa.new_state();
                let (repeated_pattern_start, repeated_pattern_end) =
                    repeated_pattern.build_into_nfa(nfa);

                nfa.link(start, repeated_pattern_start, None);
                nfa.link(start, end, None);
                nfa.link(repeated_pattern_end, end, None);
                nfa.link(repeated_pattern_end, repeated_pattern_start, None);

                (start, end)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::automata::dfa::Dfa;
    use super::*;

    fn create_dfa_for_regex(pattern: Regex) -> Dfa<u8, ()> {
        let mut nfa = Nfa::new();
        let (start, end) = pattern.build_into_nfa(&mut nfa);
        nfa.label(end, Some(()));
        nfa.set_initial_state(start);

        let dfa = nfa
            .compile_to_dfa(|labels| {
                if labels.is_empty() {
                    None
                } else {
                    Some(())
                }
            })
            .minimize();

        return dfa;
    }

    fn is_string_in(dfa: &Dfa<u8, ()>, data: &str) -> bool {
        match dfa.scan(String::from(data).into_bytes().into_iter().map(|x| x.handle())) {
            None => false,
            Some(end_state) => {
                !dfa.get_label(end_state).is_none()
            }
        }
    }
    #[test]
    fn test_single_char() {
        let pattern = Regex::single_char('a');
        let dfa = create_dfa_for_regex(pattern);

        assert_eq!(is_string_in(&dfa, "a"), true);
        assert_eq!(is_string_in(&dfa, ""), false);
        assert_eq!(is_string_in(&dfa, "aa"), false);
    }

    #[test]
    fn test_union() {
        let pattern = Regex::union(vec![
            Regex::single_char('a'),
            Regex::single_char('b'),
            Regex::single_char('c'),
        ]);
        let dfa = create_dfa_for_regex(pattern);

        assert_eq!(is_string_in(&dfa, "a"), true);
        assert_eq!(is_string_in(&dfa, "b"), true);
        assert_eq!(is_string_in(&dfa, "c"), true);
        assert_eq!(is_string_in(&dfa, ""), false);
        assert_eq!(is_string_in(&dfa, "aa"), false);
        assert_eq!(is_string_in(&dfa, "d"), false);
    }

    #[test]
    fn test_concat() {
        let pattern = Regex::concat(vec![
            Regex::single_char('a'),
            Regex::single_char('b'),
            Regex::single_char('c'),
        ]);
        let dfa = create_dfa_for_regex(pattern);


        assert_eq!(is_string_in(&dfa, "abc"), true);
        assert_eq!(is_string_in(&dfa, ""), false);
        assert_eq!(is_string_in(&dfa, "a"), false);
        assert_eq!(is_string_in(&dfa, "bc"), false);
    }

    //noinspection ALL
    #[test]
    fn test_star() {
        let pattern = Regex::star_from(
            Regex::single_char('a'),
        );
        let dfa = create_dfa_for_regex(pattern);

        assert_eq!(is_string_in(&dfa, ""), true);
        assert_eq!(is_string_in(&dfa, "a"), true);
        assert_eq!(is_string_in(&dfa, "aa"), true);
        assert_eq!(is_string_in(&dfa, "aaaaaaa"), true);
        assert_eq!(is_string_in(&dfa, "b"), false);
        assert_eq!(is_string_in(&dfa, "ab"), false);
    }

    #[test]
    fn test_plus() {
        let pattern = Regex::plus_from(
            Regex::single_char('a'),
        );
        let dfa = create_dfa_for_regex(pattern);

        assert_eq!(is_string_in(&dfa, ""), false);
        assert_eq!(is_string_in(&dfa, "a"), true);
        assert_eq!(is_string_in(&dfa, "aa"), true);
        assert_eq!(is_string_in(&dfa, "aaaaaaa"), true);
        assert_eq!(is_string_in(&dfa, "b"), false);
        assert_eq!(is_string_in(&dfa, "ab"), false);
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
        let dfa = create_dfa_for_regex(pattern);

        assert_eq!(is_string_in(&dfa, "MyThing"), true);
        assert_eq!(is_string_in(&dfa, "our_thing_12"), true);
        assert_eq!(is_string_in(&dfa, "i"), true);
        assert_eq!(is_string_in(&dfa, "a1jh2b45"), true);
        assert_eq!(is_string_in(&dfa, ""), false);
        assert_eq!(is_string_in(&dfa, "mine()"), false);
        assert_eq!(is_string_in(&dfa, "12"), false);
        assert_eq!(is_string_in(&dfa, "1ours"), false);
    }
}