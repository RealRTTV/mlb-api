pub fn to_upper_snake_case(s: &str) -> String {
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    enum Case {
        Lowercase,
        Uppercase,
    }

    impl TryFrom<char> for Case {
        type Error = ();

        fn try_from(value: char) -> Result<Self, Self::Error> {
            if value.is_ascii_uppercase() {
                Ok(Self::Uppercase)
            } else if value.is_ascii_lowercase() {
                Ok(Self::Lowercase)
            } else {
                Err(())
            }
        }
    }

    let mut s = s.replace('+', "Plus").replace(|c: char| !c.is_ascii_alphanumeric(), "");
    
    if s.starts_with(|c: char| !c.is_ascii_alphabetic()) {
        s = format!("_{s}");
    }

    let mut builder = String::new();
    let mut section_start = 0;
    let mut prev_case: Option<Case> = None;

    for (idx, char) in s.char_indices() {
        let case = Case::try_from(char).ok();
        if let Some(case) = case {
            if let Some(prev) = prev_case {
                if case > prev {
                    builder.push_str(&s[section_start..idx].to_ascii_uppercase());
                    builder.push('_');
                    section_start = idx;
                }
            }
            prev_case = Some(case);
        } else if let '_' | '-' = char {
            builder.push_str(&s[section_start..idx].to_ascii_uppercase());
            builder.push('_');
            section_start = idx + char.len_utf8();
            prev_case = None;
        }
    }

    builder.push_str(&s[section_start..].to_ascii_uppercase());

    builder
}