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
            Regex::union(Vec::new()),
        ])
    }
}

// TODO tests