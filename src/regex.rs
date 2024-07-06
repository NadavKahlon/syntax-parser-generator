pub enum Regex {
}

impl Regex {
    pub fn single_char(value: char) -> Regex {
        todo!()
    }

    pub fn union(options: impl Iterator<Item=Regex>) -> Regex {
        todo!()
    }

    pub fn concat(parts: impl Iterator<Item=Regex>) -> Regex {
        todo!()
    }

    pub fn star_from(repeated_pattern: Regex) -> Regex {
        todo!()
    }

    pub fn white_space() -> Regex {
        todo!()
    }

    pub fn constant_string(string: &str) -> Regex {
        todo!()
    }

    pub fn character_range(start: char, end: char) -> Regex {
        todo!()
    }

    pub fn optional(option: Regex) -> Regex {
        todo!()
    }
}
