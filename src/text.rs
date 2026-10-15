use fancy_regex::{Captures, Regex};

pub(crate) fn regex(pattern: &str) -> Regex {
    Regex::new(pattern).unwrap_or_else(|err| panic!("invalid built-in pattern {pattern:?}: {err}"))
}

pub(crate) fn is_match(re: &Regex, text: &str) -> bool {
    re.is_match(text).unwrap_or(false)
}

pub(crate) fn captures<'t>(re: &Regex, text: &'t str) -> Option<Captures<'t, str>> {
    re.captures(text).ok().flatten()
}

pub(crate) fn group<'t>(caps: &Captures<'t, str>, index: usize) -> &'t str {
    caps.get(index).map_or("", |m| m.as_str())
}

pub(crate) fn char_len(s: &str) -> usize {
    s.chars().count()
}

pub(crate) fn chop(s: &str, n: usize) -> &str {
    if n == 0 {
        return s;
    }
    match s.char_indices().rev().nth(n - 1) {
        Some((i, _)) => &s[..i],
        None => "",
    }
}

pub(crate) fn tail(s: &str, n: usize) -> &str {
    if n == 0 {
        return "";
    }
    match s.char_indices().rev().nth(n - 1) {
        Some((i, _)) => &s[i..],
        None => s,
    }
}

pub(crate) fn slice_from_end(s: &str, start: usize, end: usize) -> &str {
    chop(tail(s, start), end)
}

pub(crate) fn last_char(s: &str) -> Option<char> {
    s.chars().next_back()
}

pub(crate) fn ends_with_any<S: AsRef<str>>(s: &str, endings: &[S]) -> bool {
    endings.iter().any(|e| s.ends_with(e.as_ref()))
}

pub(crate) fn upper(s: &str) -> String {
    s.to_uppercase()
}

pub(crate) fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first
            .to_uppercase()
            .chain(chars.as_str().to_lowercase().chars())
            .collect(),
        None => String::new(),
    }
}

pub(crate) fn starts_upper(s: &str) -> bool {
    s.chars().next().is_some_and(|c| {
        let mut up = c.to_uppercase();
        up.next() == Some(c) && up.next().is_none()
    })
}

pub(crate) struct Words<'a> {
    pub(crate) lowered: String,
    pub(crate) split: Vec<&'a str>,
    pub(crate) first: &'a str,
    pub(crate) last: &'a str,
}

impl<'a> Words<'a> {
    pub(crate) fn new(word: &'a str) -> Self {
        let split: Vec<&str> = word.split_whitespace().collect();
        Words {
            lowered: word.to_lowercase(),
            first: split.first().copied().unwrap_or(""),
            last: split.last().copied().unwrap_or(""),
            split,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slicing_counts_chars() {
        assert_eq!(chop("ångström", 2), "ångstr");
        assert_eq!(chop("ab", 5), "");
        assert_eq!(chop("ab", 0), "ab");
        assert_eq!(tail("cliché", 2), "hé");
        assert_eq!(tail("ab", 5), "ab");
        assert_eq!(slice_from_end("wolves", 5, 3), "ol");
        assert_eq!(slice_from_end("ves", 5, 3), "");
    }

    #[test]
    fn capitalization() {
        assert_eq!(capitalize("hELLO"), "Hello");
        assert_eq!(capitalize(""), "");
        assert!(starts_upper("Cat"));
        assert!(starts_upper("1st"));
        assert!(!starts_upper("cat"));
        assert!(!starts_upper("ß"));
    }

    #[test]
    fn words() {
        let words = Words::new("A quick brown fox.");
        assert_eq!(words.lowered, "a quick brown fox.");
        assert_eq!(words.split, vec!["A", "quick", "brown", "fox."]);
        assert_eq!(words.first, "A");
        assert_eq!(words.last, "fox.");
    }
}
