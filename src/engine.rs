use std::collections::HashMap;
use std::sync::LazyLock;

use fancy_regex::Regex;

use crate::data::*;
use crate::text::{
    capitalize, captures, char_len, chop, ends_with_any, group, is_match, last_char, regex,
    slice_from_end, starts_upper, upper, Words,
};
use crate::{Classical, Comparison, Count, Error, Gender, JoinOptions};

static PARTITION_WORD: LazyLock<Regex> = LazyLock::new(|| regex(r"\A(\s*)(.+?)(\s*)\z"));
static PL_SB_POSTFIX_ADJ_STEMS_RE: LazyLock<Regex> =
    LazyLock::new(|| regex(&format!("(?i)^(?:{})$", *PL_SB_POSTFIX_ADJ_STEMS)));
static PL_SB_PREP_DUAL_COMPOUND_RE: LazyLock<Regex> =
    LazyLock::new(|| regex(&format!("(?i)^(?:{})$", *PL_SB_PREP_DUAL_COMPOUND)));
static DENOMINATOR: LazyLock<Regex> = LazyLock::new(|| regex(r"(?P<denominator>.+)( (per|a) .+)"));
static PLVERB_SPECIAL_S_RE: LazyLock<Regex> =
    LazyLock::new(|| regex(&format!("^({})$", *PLVERB_SPECIAL_S)));
static PLVERB_AMBIGUOUS_PRES_RE: LazyLock<Regex> = LazyLock::new(|| {
    regex(&format!(
        r"(?i)^({})((\s.*)?)$",
        *PLVERB_AMBIGUOUS_PRES_KEYS
    ))
});
static ENDS_WITH_S: LazyLock<Regex> = LazyLock::new(|| regex(r"(?i)^(.*[^s])s$"));
static ENDS_WITH_APOSTROPHE_S: LazyLock<Regex> = LazyLock::new(|| regex(r"^(.+)'s?$"));
static INDEFINITE_ARTICLE_TEST: LazyLock<Regex> =
    LazyLock::new(|| regex(r"(?i)\A(\s*)(?:an?\s+)?(.+?)(\s*)\z"));

static INDEF_ARTICLE_CASES: LazyLock<Vec<(Regex, &'static str)>> = LazyLock::new(|| {
    [
        (r"(?i)^([bcdgjkpqtuvwyz]-?th)", "a"),
        (r"(?i)^([aefhilmnorsx]-?th)", "an"),
        (r"(?i)^((?:euler|hour(?!i)|heir|honest|hono[ur]|mpeg))", "an"),
        (r"(?i)^[aefhilmnorsx]$", "an"),
        (r"(?i)^[bcdgjkpqtuvwyz]$", "a"),
        (
            r"^(?!FJO|[HLMNS]Y.|RY[EO]|SQU|(F[LR]?|[HL]|MN?|N|RH?|S[CHKLMNPTVW]?|X(YL)?)[AEIOU])[FHLMNRSX][A-Z]",
            "an",
        ),
        (r"(?i)^[aefhilmnorsx][.-]", "an"),
        (r"(?i)^[a-z][.-]", "a"),
        (r"(?i)^[^aeiouy]", "a"),
        (r"(?i)^e[uw]", "a"),
        (r"(?i)^onc?e\b", "a"),
        (r"(?i)^onetime\b", "a"),
        (r"(?i)^uni([^nmd]|mo)", "a"),
        (r"(?i)^u[bcfghjkqrst][aeiou]", "a"),
        (r"(?i)^ukr", "a"),
        (r"(?i)^((?:unabomber|unanimous|US))", "a"),
        (r"^U[NK][AIEO]?", "a"),
        (r"(?i)^[aeiou]", "an"),
        (r"(?i)^(y(b[lor]|cl[ea]|fere|gg|p[ios]|rou|tt))", "an"),
    ]
    .into_iter()
    .map(|(pattern, article)| (regex(pattern), article))
    .collect()
});

static PRESENT_PARTICIPLE_REPLACEMENTS: LazyLock<Vec<(Regex, &'static str)>> =
    LazyLock::new(|| {
        [
            (r"ie$", "y"),
            (r"ue$", "u"),
            (r"([auy])e$", "${1}"),
            (r"ski$", "ski"),
            (r"[^b]i$", ""),
            (r"^(are|were)$", "be"),
            (r"^(had)$", "hav"),
            (r"^(hoe)$", "${1}"),
            (r"([^e])e$", "${1}"),
            (r"er$", "er"),
            (r"([^aeiou][aeiouy]([bdgmnprst]))$", "${1}${2}"),
        ]
        .into_iter()
        .map(|(pattern, replacement)| (regex(pattern), replacement))
        .collect()
    });

static PLURAL_STEM_ENDINGS: LazyLock<Vec<(Regex, &'static str, &'static str)>> =
    LazyLock::new(|| {
        [
            (C_A_ATA.stem.as_str(), "as", "ata"),
            (PL_SB_C_IS_IDES.as_str(), "is", "ides"),
            (C_A_AE.stem.as_str(), "s", "e"),
            (C_EN_INA.stem.as_str(), "ens", "ina"),
            (C_UM_A.stem.as_str(), "ums", "a"),
            (C_US_I.stem.as_str(), "uses", "i"),
            (C_ON_A.stem.as_str(), "ons", "a"),
            (C_O_I.stem.as_str(), "os", "i"),
            (C_EX_ICES.stem.as_str(), "exes", "ices"),
            (C_IX_ICES.stem.as_str(), "ixes", "ices"),
            (C_I.stem.as_str(), "s", "i"),
            (C_IM.stem.as_str(), "s", "im"),
            (".*eau", "s", "x"),
            (".*ieu", "s", "x"),
            (".*tri", "xes", "ces"),
            (".{2,}[yia]n", "xes", "ges"),
        ]
        .into_iter()
        .map(|(stems, end1, end2)| {
            let pattern = format!(r"({stems})({end1}\|\1{end2}|{end2}\|\1{end1})");
            (regex(&pattern), end1, end2)
        })
        .collect()
    });

enum SiPron {
    Plain(&'static str),
    Gendered([&'static str; 6]),
}

#[derive(Clone, Copy)]
enum Case {
    Nom,
    Acc,
}

const THEMSELVES: [&str; 6] = [
    "itself",
    "herself",
    "himself",
    "themself",
    "herself or himself",
    "himself or herself",
];

fn invert_pronouns(pairs: &[(&'static str, &'static str)]) -> HashMap<&'static str, SiPron> {
    pairs
        .iter()
        .map(|&(singular, plural)| (plural, SiPron::Plain(singular)))
        .collect()
}

static SI_PRON_NOM: LazyLock<HashMap<&'static str, SiPron>> = LazyLock::new(|| {
    let mut nom = invert_pronouns(PL_PRON_NOM);
    nom.insert("we", SiPron::Plain("I"));
    nom.insert(
        "they",
        SiPron::Gendered(["it", "she", "he", "they", "she or he", "he or she"]),
    );
    nom.insert("themselves", SiPron::Gendered(THEMSELVES));
    nom.insert(
        "theirs",
        SiPron::Gendered(["its", "hers", "his", "theirs", "hers or his", "his or hers"]),
    );
    nom
});

static SI_PRON_ACC: LazyLock<HashMap<&'static str, SiPron>> = LazyLock::new(|| {
    let mut acc = invert_pronouns(PL_PRON_ACC);
    acc.insert(
        "them",
        SiPron::Gendered(["it", "her", "him", "them", "her or him", "him or her"]),
    );
    acc.insert("themselves", SiPron::Gendered(THEMSELVES));
    acc
});

fn get_si_pron(case: Case, word: &str, gender: Gender) -> Option<&'static str> {
    let table = match case {
        Case::Nom => &*SI_PRON_NOM,
        Case::Acc => &*SI_PRON_ACC,
    };
    table.get(word).map(|pron| match pron {
        SiPron::Plain(s) => *s,
        SiPron::Gendered(forms) => forms[gender as usize],
    })
}

fn partition_word(text: &str) -> (&str, &str, &str) {
    match captures(&PARTITION_WORD, text) {
        Some(caps) => (group(&caps, 1), group(&caps, 2), group(&caps, 3)),
        None => ("", "", ""),
    }
}

fn prep_pronoun_split<'a>(words: &'a Words) -> Option<(&'a str, &'a str)> {
    let lowered = words.lowered.as_str();
    let mut tokens = lowered.split_whitespace();
    let (prep, pronoun) = (tokens.next()?, tokens.next()?);
    let exact = tokens.next().is_none()
        && lowered.starts_with(prep)
        && lowered.ends_with(pronoun)
        && PL_PREP_SET.contains(prep);
    exact.then_some((prep, pronoun))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Number {
    One,
    Many,
    Unset,
}

const MANY: Count = Count::Num(2);
const ONE: Count = Count::Num(1);

#[derive(Clone, Copy, PartialEq, Eq)]
enum PartOfSpeech {
    Noun,
    Verb,
    Adjective,
}

#[derive(Clone, Debug)]
struct UserDefined {
    pattern: Regex,
    replacement: String,
}

/// The inflection engine.
///
/// Holds the settings that affect inflection: classical modes, a persistent
/// count, the gender used for singular pronouns, and any user-defined
/// inflections.
#[derive(Clone, Debug, Default)]
pub struct Engine {
    pub classical: Classical,
    persistent_count: Option<i64>,
    gender: Gender,
    pl_sb_user_defined: Vec<UserDefined>,
    pl_v_user_defined: Vec<UserDefined>,
    pl_adj_user_defined: Vec<UserDefined>,
    si_sb_user_defined: Vec<UserDefined>,
    a_a_user_defined: Vec<UserDefined>,
}

fn user_defined(pattern: &str, replacement: &str) -> Result<UserDefined, Error> {
    let compiled = Regex::new(&format!("(?i)^{pattern}$"))
        .map_err(|_| Error::BadUserDefinedPattern(pattern.to_string()))?;
    Ok(UserDefined {
        pattern: compiled,
        replacement: replacement.to_string(),
    })
}

fn ud_match(word: &str, wordlist: &[UserDefined]) -> Option<String> {
    wordlist.iter().rev().find_map(|ud| {
        let caps = captures(&ud.pattern, word)?;
        let mut expanded = String::new();
        let mut rest = ud.replacement.as_str();
        while let Some(pos) = rest.find('$') {
            expanded.push_str(&rest[..pos]);
            let after = &rest[pos + 1..];
            let digits = after.len() - after.trim_start_matches(|c: char| c.is_ascii_digit()).len();
            if digits == 0 {
                expanded.push('$');
                rest = after;
            } else {
                let index: usize = after[..digits].parse().unwrap_or(usize::MAX);
                expanded.push_str(caps.get(index).map_or("", |m| m.as_str()));
                rest = &after[digits..];
            }
        }
        expanded.push_str(rest);
        Some(expanded)
    })
}

impl Engine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the noun plural of words matching `singular` (a regex) to `plural`.
    ///
    /// `$1`, `$2`, ... in `plural` are replaced by groups captured by `singular`.
    pub fn defnoun(&mut self, singular: &str, plural: &str) -> Result<(), Error> {
        self.pl_sb_user_defined
            .push(user_defined(singular, plural)?);
        self.si_sb_user_defined
            .push(user_defined(plural, singular)?);
        Ok(())
    }

    /// Sets the plurals of the 1st, 2nd and 3rd person singular forms of a verb.
    pub fn defverb(
        &mut self,
        s1: &str,
        p1: &str,
        s2: &str,
        p2: &str,
        s3: &str,
        p3: &str,
    ) -> Result<(), Error> {
        let defs = [
            user_defined(s1, p1)?,
            user_defined(s2, p2)?,
            user_defined(s3, p3)?,
        ];
        self.pl_v_user_defined.extend(defs);
        Ok(())
    }

    pub fn defadj(&mut self, singular: &str, plural: &str) -> Result<(), Error> {
        self.pl_adj_user_defined
            .push(user_defined(singular, plural)?);
        Ok(())
    }

    /// Uses "a" as the indefinite article for words matching `pattern`.
    pub fn defa(&mut self, pattern: &str) -> Result<(), Error> {
        self.a_a_user_defined.push(user_defined(pattern, "a")?);
        Ok(())
    }

    /// Uses "an" as the indefinite article for words matching `pattern`.
    pub fn defan(&mut self, pattern: &str) -> Result<(), Error> {
        self.a_a_user_defined.push(user_defined(pattern, "an")?);
        Ok(())
    }

    /// Sets the count used by later calls that are not given one, returning
    /// it as a string. `None` clears it and returns an empty string.
    pub fn num(&mut self, count: Option<i64>) -> String {
        self.persistent_count = count;
        count.map(|n| n.to_string()).unwrap_or_default()
    }

    pub fn persistent_count(&self) -> Option<i64> {
        self.persistent_count
    }

    pub fn set_gender(&mut self, gender: Gender) {
        self.gender = gender;
    }

    pub fn gender(&self) -> Gender {
        self.gender
    }

    fn postprocess(&self, orig: &str, inflected: &str) -> String {
        let choice = usize::from(self.classical.all);
        let mut result: Vec<String> = if inflected.contains('|') {
            let options: Vec<&str> = inflected.split('|').collect();
            if options[0].split(' ').count() == options[1].split(' ').count() {
                options[choice].split(' ').map(String::from).collect()
            } else {
                inflected
                    .split(' ')
                    .map(|word| match word.split('|').nth(choice) {
                        Some(option) if word.contains('|') => option.to_string(),
                        _ => word.to_string(),
                    })
                    .collect()
            }
        } else {
            inflected.split(' ').map(String::from).collect()
        };

        for (index, word) in orig.split(' ').enumerate() {
            if word.is_empty() || word == "I" || index >= result.len() {
                continue;
            }
            if capitalize(word) == word {
                result[index] = capitalize(&result[index]);
            }
            if upper(word) == word {
                result[index] = upper(&result[index]);
            }
        }
        result.join(" ")
    }

    fn inflect_partitioned(&self, text: &str, inflect: impl FnOnce(&str) -> String) -> String {
        let (pre, word, post) = partition_word(text);
        if word.trim().is_empty() {
            return text.to_string();
        }
        format!("{pre}{}{post}", self.postprocess(word, &inflect(word)))
    }

    /// Returns the plural of `text`, which may be a noun, verb or adjective.
    ///
    /// Leading and trailing whitespace is preserved.
    pub fn plural(&self, text: &str) -> String {
        self.plural_count(text, None)
    }

    /// Returns `text` unchanged if `count` is singular (1, "a", "one", "each",
    /// ...), otherwise its plural.
    pub fn plural_with_count(&self, text: &str, count: impl Into<Count>) -> String {
        self.plural_count(text, Some(&count.into()))
    }

    pub(crate) fn plural_count(&self, text: &str, count: Option<&Count>) -> String {
        self.inflect_partitioned(text, |word| {
            self.pl_special_adjective(word, count)
                .or_else(|| self.pl_special_verb(word, count))
                .unwrap_or_else(|| self.plnoun(word, count))
        })
    }

    pub fn plural_noun(&self, text: &str) -> String {
        self.plural_noun_count(text, None)
    }

    pub fn plural_noun_with_count(&self, text: &str, count: impl Into<Count>) -> String {
        self.plural_noun_count(text, Some(&count.into()))
    }

    pub(crate) fn plural_noun_count(&self, text: &str, count: Option<&Count>) -> String {
        self.inflect_partitioned(text, |word| self.plnoun(word, count))
    }

    pub fn plural_verb(&self, text: &str) -> String {
        self.plural_verb_count(text, None)
    }

    pub fn plural_verb_with_count(&self, text: &str, count: impl Into<Count>) -> String {
        self.plural_verb_count(text, Some(&count.into()))
    }

    pub(crate) fn plural_verb_count(&self, text: &str, count: Option<&Count>) -> String {
        self.inflect_partitioned(text, |word| {
            self.pl_special_verb(word, count)
                .unwrap_or_else(|| self.pl_general_verb(word, count))
        })
    }

    pub fn plural_adj(&self, text: &str) -> String {
        self.plural_adj_count(text, None)
    }

    pub fn plural_adj_with_count(&self, text: &str, count: impl Into<Count>) -> String {
        self.plural_adj_count(text, Some(&count.into()))
    }

    pub(crate) fn plural_adj_count(&self, text: &str, count: Option<&Count>) -> String {
        self.inflect_partitioned(text, |word| {
            self.pl_special_adjective(word, count)
                .unwrap_or_else(|| word.to_string())
        })
    }

    /// Returns the singular of `text`, a plural noun, or `None` if `text`
    /// does not look like a plural.
    pub fn singular_noun(&self, text: &str) -> Option<String> {
        self.singular_noun_count(text, None, None)
    }

    pub fn singular_noun_with_count(&self, text: &str, count: impl Into<Count>) -> Option<String> {
        self.singular_noun_count(text, Some(&count.into()), None)
    }

    /// Like [`singular_noun`](Self::singular_noun), using `gender` for
    /// pronouns instead of the engine's gender.
    pub fn singular_noun_with_gender(&self, text: &str, gender: Gender) -> Option<String> {
        self.singular_noun_count(text, None, Some(gender))
    }

    pub(crate) fn singular_noun_count(
        &self,
        text: &str,
        count: Option<&Count>,
        gender: Option<Gender>,
    ) -> Option<String> {
        let (pre, word, post) = partition_word(text);
        if word.trim().is_empty() {
            return Some(text.to_string());
        }
        let singular = self.sinoun(word, count, gender.unwrap_or(self.gender))?;
        Some(format!("{pre}{}{post}", self.postprocess(word, &singular)))
    }

    /// Compares two words for equality regardless of plurality, trying them as
    /// nouns, then verbs, then adjectives.
    pub fn compare(&self, word1: &str, word2: &str) -> Option<Comparison> {
        [
            PartOfSpeech::Noun,
            PartOfSpeech::Verb,
            PartOfSpeech::Adjective,
        ]
        .into_iter()
        .find_map(|pos| self.plequal(word1, word2, pos))
    }

    pub fn compare_nouns(&self, word1: &str, word2: &str) -> Option<Comparison> {
        self.plequal(word1, word2, PartOfSpeech::Noun)
    }

    pub fn compare_verbs(&self, word1: &str, word2: &str) -> Option<Comparison> {
        self.plequal(word1, word2, PartOfSpeech::Verb)
    }

    pub fn compare_adjs(&self, word1: &str, word2: &str) -> Option<Comparison> {
        self.plequal(word1, word2, PartOfSpeech::Adjective)
    }

    fn plequal(&self, word1: &str, word2: &str, pos: PartOfSpeech) -> Option<Comparison> {
        for classical in [Classical::all(), Classical::none()] {
            let engine = Engine {
                classical,
                ..self.clone()
            };
            let pl = |word: &str| match pos {
                PartOfSpeech::Noun => engine.plural_noun(word),
                PartOfSpeech::Verb => engine.plural_verb(word),
                PartOfSpeech::Adjective => engine.plural_adj(word),
            };
            if word1 == word2 {
                return Some(Comparison::Eq);
            }
            if word1 == pl(word2) {
                return Some(Comparison::PluralSingular);
            }
            if pl(word1) == word2 {
                return Some(Comparison::SingularPlural);
            }
        }
        let plurals = match pos {
            PartOfSpeech::Noun => {
                pl_check_plurals_n(word1, word2) || pl_check_plurals_n(word2, word1)
            }
            PartOfSpeech::Adjective => pl_check_plurals_adj(word1, word2),
            PartOfSpeech::Verb => false,
        };
        plurals.then_some(Comparison::PluralPlural)
    }

    fn get_count(&self, count: Option<&Count>) -> Number {
        let text = match (count, self.persistent_count) {
            (Some(count), _) => count.to_string(),
            (None, Some(n)) => n.to_string(),
            (None, None) => return Number::Unset,
        };
        let zero = self.classical.zero && PL_COUNT_ZERO_SET.contains(text.to_lowercase().as_str());
        if PL_COUNT_ONE_SET.contains(text.as_str()) || zero {
            Number::One
        } else {
            Number::Many
        }
    }

    fn plnoun(&self, word: &str, count: Option<&Count>) -> String {
        if self.get_count(count) == Number::One {
            return word.to_string();
        }

        if let Some(value) = ud_match(word, &self.pl_sb_user_defined) {
            return value;
        }

        if word.is_empty() {
            return String::new();
        }

        let words = Words::new(word);
        let lowered = words.lowered.as_str();
        let lowered_last = words.last.to_lowercase();
        let lowered_last = lowered_last.as_str();

        if PL_SB_UNINFLECTED_COMPLETE.contains(lowered_last) {
            if words.split.len() >= 3 {
                if let Some(handled) = self.handle_long_compounds(&words, Number::Many) {
                    return handled;
                }
            }
            return word.to_string();
        }

        if PL_SB_UNINFLECTED_CAPS_SET.contains(word)
            || ends_with_any(lowered, &PL_SB_UNINFLECTED_ENDINGS)
            || (self.classical.herd && PL_SB_UNINFLECTED_HERD_SET.contains(lowered_last))
        {
            return word.to_string();
        }

        if let Some(caps) = captures(&PL_SB_POSTFIX_ADJ_STEMS_RE, word) {
            if !group(&caps, 2).is_empty() {
                return format!(
                    "{}{}",
                    self.plnoun(group(&caps, 1), Some(&MANY)),
                    group(&caps, 2)
                );
            }
        }

        if lowered.contains(" a ") || lowered.contains("-a-") {
            if let Some(caps) = captures(&PL_SB_PREP_DUAL_COMPOUND_RE, word) {
                if !group(&caps, 2).is_empty() && !group(&caps, 3).is_empty() {
                    return format!(
                        "{}{}{}",
                        self.plnoun(group(&caps, 1), Some(&MANY)),
                        group(&caps, 2),
                        self.plnoun(group(&caps, 3), None)
                    );
                }
            }
        }

        if words.split.len() >= 3 {
            if let Some(handled) = self.handle_long_compounds(&words, Number::Many) {
                return handled;
            }
        }

        if let Some(caps) = captures(&DENOMINATOR, lowered) {
            let index = char_len(group(&caps, 1));
            let split = word
                .char_indices()
                .nth(index)
                .map_or(word.len(), |(i, _)| i);
            return format!("{}{}", self.plnoun(&word[..split], None), &word[split..]);
        }

        if words.split.len() >= 2 && words.split[words.split.len() - 2] == "degree" {
            let rest = words.split[1..].iter().map(|s| s.to_string());
            return std::iter::once(self.plnoun(words.first, None))
                .chain(rest)
                .collect::<Vec<_>>()
                .join(" ");
        }

        if let Some(handled) =
            handle_prepositional_phrase(lowered, '-', |part| Some(self.plnoun(part, Some(&MANY))))
        {
            return handled;
        }

        if let Some((_, pronoun)) = prep_pronoun_split(&words) {
            if let Some(plural) = PL_PRON_ACC_MAP.get(pronoun) {
                return format!("{}{plural}", chop(lowered, char_len(pronoun)));
            }
        }

        if let Some(plural) = PL_PRON_NOM_MAP.get(lowered) {
            return plural.to_string();
        }
        if let Some(plural) = PL_PRON_ACC_MAP.get(lowered) {
            return plural.to_string();
        }

        if let Some(plural) = PL_SB_IRREGULAR_CAPS_MAP.get(words.last) {
            return format!("{}{plural}", chop(word, char_len(words.last)));
        }

        if let Some(plural) = PL_SB_IRREGULAR.get(lowered_last) {
            return format!("{}{plural}", chop(word, char_len(lowered_last)));
        }

        let compound = last_two_dash_parts(lowered);
        if let Some(plural) = PL_SB_IRREGULAR_COMPOUND_MAP.get(compound.as_str()) {
            return format!("{}{plural}", chop(word, char_len(&compound)));
        }

        if lowered.ends_with("quy") {
            return format!("{}ies", chop(word, 1));
        }

        if lowered.ends_with("person") {
            return if self.classical.persons {
                format!("{word}s")
            } else {
                format!("{}ople", chop(word, 4))
            };
        }

        if lowered.ends_with("man") {
            if ends_with_any(lowered, PL_SB_U_MAN_MANS_LIST)
                || ends_with_any(word, PL_SB_U_MAN_MANS_CAPS_LIST)
            {
                return format!("{word}s");
            }
            return format!("{}men", chop(word, 3));
        }
        if lowered.ends_with("mouse") {
            return format!("{}mice", chop(word, 5));
        }
        if lowered.ends_with("louse") {
            if PL_SB_U_LOUSE_LICE_LIST.contains(&lowered) {
                return format!("{}lice", chop(word, 5));
            }
            return format!("{word}s");
        }
        if lowered.ends_with("goose") {
            return format!("{}geese", chop(word, 5));
        }
        if lowered.ends_with("tooth") {
            return format!("{}teeth", chop(word, 5));
        }
        if lowered.ends_with("foot") {
            return format!("{}feet", chop(word, 4));
        }
        if lowered.ends_with("taco") {
            return format!("{}tacos", chop(word, 4));
        }

        if lowered == "die" {
            return "dice".to_string();
        }

        if lowered.ends_with("ceps") {
            return word.to_string();
        }
        if lowered.ends_with("zoon") {
            return format!("{}a", chop(word, 2));
        }
        if ["cis", "sis", "xis"].iter().any(|e| lowered.ends_with(e)) {
            return format!("{}es", chop(word, 2));
        }

        let last = last_char(lowered);
        let unassimilated: [(char, &[&str], usize, &str); 7] = [
            ('h', PL_SB_U_CH_CHS_LIST, 0, "s"),
            ('x', PL_SB_U_EX_ICES_LIST, 2, "ices"),
            ('x', PL_SB_U_IX_ICES_LIST, 2, "ices"),
            ('m', PL_SB_U_UM_A_LIST, 2, "a"),
            ('s', PL_SB_U_US_I_LIST, 2, "i"),
            ('n', PL_SB_U_ON_A_LIST, 2, "a"),
            ('a', PL_SB_U_A_AE_LIST, 0, "e"),
        ];
        for (lastlet, list, numend, post) in unassimilated {
            if last == Some(lastlet) && ends_with_any(lowered, list) {
                return format!("{}{post}", chop(word, numend));
            }
        }

        if self.classical.ancient {
            if lowered.ends_with("trix") {
                return format!("{}ces", chop(word, 1));
            }
            if lowered.ends_with("eau") || lowered.ends_with("ieu") {
                return format!("{word}x");
            }
            if ["ynx", "inx", "anx"].iter().any(|e| lowered.ends_with(e)) && char_len(word) > 4 {
                return format!("{}ges", chop(word, 1));
            }

            let incompletely_assimilated: [(Option<char>, &[&str], usize, &str); 13] = [
                (Some('n'), PL_SB_C_EN_INA_LIST, 2, "ina"),
                (Some('x'), PL_SB_C_EX_ICES_LIST, 2, "ices"),
                (Some('x'), PL_SB_C_IX_ICES_LIST, 2, "ices"),
                (Some('m'), PL_SB_C_UM_A_LIST, 2, "a"),
                (Some('s'), PL_SB_C_US_I_LIST, 2, "i"),
                (Some('s'), PL_SB_C_US_US, 0, ""),
                (Some('a'), PL_SB_C_A_AE_LIST, 0, "e"),
                (Some('a'), PL_SB_C_A_ATA_LIST, 0, "ta"),
                (Some('s'), &PL_SB_C_IS_IDES_LIST, 1, "des"),
                (Some('o'), PL_SB_C_O_I, 1, "i"),
                (Some('n'), PL_SB_C_ON_A_LIST, 2, "a"),
                (None, PL_SB_C_I_LIST, 0, "i"),
                (None, PL_SB_C_IM_LIST, 0, "im"),
            ];
            for (lastlet, list, numend, post) in incompletely_assimilated {
                if (lastlet.is_none() || last == lastlet) && ends_with_any(lowered, list) {
                    return format!("{}{post}", chop(word, numend));
                }
            }
        }

        if PL_SB_SINGULAR_S_COMPLETE.contains(&lowered_last)
            || ends_with_any(lowered, &PL_SB_SINGULAR_S_ENDINGS)
            || (lowered.ends_with("es") && starts_upper(word))
        {
            return format!("{word}es");
        }

        if last == Some('z') {
            if ends_with_any(lowered, PL_SB_Z_ZES_LIST) {
                return format!("{word}es");
            }
            if slice_from_end(lowered, 2, 1) != "z" {
                return format!("{word}zes");
            }
        }

        if lowered.ends_with("ze") && ends_with_any(lowered, PL_SB_ZE_ZES_LIST) {
            return format!("{word}s");
        }

        if ["ch", "sh", "zz", "ss"]
            .iter()
            .any(|e| lowered.ends_with(e))
            || last == Some('x')
        {
            return format!("{word}es");
        }

        if ["elf", "alf", "olf"].iter().any(|e| lowered.ends_with(e)) {
            return format!("{}ves", chop(word, 1));
        }
        if lowered.ends_with("eaf") && slice_from_end(lowered, 4, 3) != "d" {
            return format!("{}ves", chop(word, 1));
        }
        if ["nife", "life", "wife"]
            .iter()
            .any(|e| lowered.ends_with(e))
        {
            return format!("{}ves", chop(word, 2));
        }
        if lowered.ends_with("arf") {
            return format!("{}ves", chop(word, 1));
        }

        if last == Some('y') {
            if "aeiou".contains(slice_from_end(lowered, 2, 1)) || char_len(word) == 1 {
                return format!("{word}s");
            }
            if self.classical.names && starts_upper(word) {
                return format!("{word}s");
            }
            return format!("{}ies", chop(word, 1));
        }

        if PL_SB_U_O_OS_COMPLETE_SET.contains(lowered_last)
            || ends_with_any(lowered, &PL_SB_U_O_OS_ENDINGS)
            || ["ao", "eo", "io", "oo", "uo"]
                .iter()
                .any(|e| lowered.ends_with(e))
        {
            return format!("{word}s");
        }

        if last == Some('o') {
            return format!("{word}es");
        }

        format!("{word}s")
    }

    fn handle_long_compounds(&self, words: &Words, number: Number) -> Option<String> {
        let split = &words.split;
        (0..split.len().saturating_sub(1)).find_map(|i| {
            let (candidate, prep) = (split[i], split[i + 1]);
            if !PL_PREP_SET.contains(prep) {
                return None;
            }
            let inflected = if number == Number::One {
                self.sinoun(candidate, Some(&ONE), self.gender)
                    .unwrap_or_else(|| candidate.to_string())
            } else {
                self.plnoun(candidate, Some(&MANY))
            };
            let parts: Vec<&str> = split[..i]
                .iter()
                .copied()
                .chain([inflected.as_str(), prep])
                .chain(split[i + 2..].iter().copied())
                .collect();
            Some(parts.join(" "))
        })
    }

    fn pl_special_verb(&self, word: &str, count: Option<&Count>) -> Option<String> {
        if self.classical.zero {
            if let Some(count) = count {
                if PL_COUNT_ZERO_SET.contains(count.to_string().to_lowercase().as_str()) {
                    return None;
                }
            }
        }
        if self.get_count(count) == Number::One {
            return Some(word.to_string());
        }

        if let Some(value) = ud_match(word, &self.pl_v_user_defined) {
            return Some(value).filter(|v| !v.is_empty());
        }

        let words = Words::new(word);
        if words.split.is_empty() {
            return None;
        }
        let first = words.first;
        let after_first = &word[first.len()..];

        if let Some(plural) = PLVERB_IRREGULAR_PRES_MAP.get(first) {
            return Some(format!("{plural}{after_first}"));
        }

        if PLVERB_IRREGULAR_NON_PRES_SET.contains(first) {
            return Some(word.to_string());
        }

        if let Some(stem) = first.strip_suffix("n't") {
            if let Some(plural) = PLVERB_IRREGULAR_PRES_MAP.get(stem) {
                return Some(format!("{plural}n't{after_first}"));
            }
            return Some(word.to_string());
        }

        if is_match(&PLVERB_SPECIAL_S_RE, word) || word.chars().any(char::is_whitespace) {
            return None;
        }

        let lowered = words.lowered.as_str();
        if lowered == "quizzes" {
            return Some("quiz".to_string());
        }

        if ["ches", "shes", "zzes", "sses"]
            .iter()
            .any(|e| lowered.ends_with(e))
            || lowered.ends_with("xes")
        {
            return Some(chop(word, 2).to_string());
        }

        if lowered.ends_with("ies") && char_len(word) > 3 {
            return Some(format!("{}y", chop(lowered, 3)));
        }

        if PL_V_OES_OE_SET.contains(words.last.to_lowercase().as_str())
            || ends_with_any(lowered, PL_V_OES_OE_ENDINGS_SIZE4)
            || ends_with_any(lowered, PL_V_OES_OE_ENDINGS_SIZE5)
        {
            return Some(chop(word, 1).to_string());
        }

        if lowered.ends_with("oes") && char_len(word) > 3 {
            return Some(chop(lowered, 2).to_string());
        }

        captures(&ENDS_WITH_S, word).map(|caps| group(&caps, 1).to_string())
    }

    fn pl_general_verb(&self, word: &str, count: Option<&Count>) -> String {
        if self.get_count(count) == Number::One {
            return word.to_string();
        }

        if let Some(caps) = captures(&PLVERB_AMBIGUOUS_PRES_RE, word) {
            let key = group(&caps, 1).to_lowercase();
            if let Some(plural) = PLVERB_AMBIGUOUS_PRES_MAP.get(key.as_str()) {
                return format!("{plural}{}", group(&caps, 2));
            }
        }

        word.to_string()
    }

    fn pl_special_adjective(&self, word: &str, count: Option<&Count>) -> Option<String> {
        if self.get_count(count) == Number::One {
            return Some(word.to_string());
        }

        if let Some(value) = ud_match(word, &self.pl_adj_user_defined) {
            return Some(value).filter(|v| !v.is_empty());
        }

        let lowered = word.to_lowercase();
        if let Some(plural) = PL_ADJ_SPECIAL_MAP.get(lowered.as_str()) {
            return Some(plural.to_string());
        }
        if let Some(plural) = PL_ADJ_POSS_MAP.get(lowered.as_str()) {
            return Some(plural.to_string());
        }

        let caps = captures(&ENDS_WITH_APOSTROPHE_S, word)?;
        let plural = self.plural_noun(group(&caps, 1));
        let trailing_s = if plural.ends_with('s') { "" } else { "s" };
        Some(format!("{plural}'{trailing_s}"))
    }

    fn sinoun(&self, word: &str, count: Option<&Count>, gender: Gender) -> Option<String> {
        if self.get_count(count) == Number::Many {
            return Some(word.to_string());
        }

        if let Some(value) = ud_match(word, &self.si_sb_user_defined) {
            return Some(value);
        }

        if word.is_empty() {
            return Some(String::new());
        }

        if SI_SB_OIS_OI_CASE_SET.contains(word) {
            return Some(chop(word, 1).to_string());
        }

        let words = Words::new(word);
        let lowered = words.lowered.as_str();
        let lowered_last = words.last.to_lowercase();
        let lowered_last = lowered_last.as_str();
        let unchanged = || Some(word.to_string());

        if PL_SB_UNINFLECTED_COMPLETE.contains(lowered_last) {
            if words.split.len() >= 3 {
                if let Some(handled) = self.handle_long_compounds(&words, Number::One) {
                    return Some(handled);
                }
            }
            return unchanged();
        }

        if PL_SB_UNINFLECTED_CAPS_SET.contains(word)
            || ends_with_any(lowered, &PL_SB_UNINFLECTED_ENDINGS)
            || (self.classical.herd && PL_SB_UNINFLECTED_HERD_SET.contains(lowered_last))
        {
            return unchanged();
        }

        if PL_SB_C_US_US_SET.contains(lowered_last) {
            return if self.classical.ancient {
                unchanged()
            } else {
                None
            };
        }

        if let Some(caps) = captures(&PL_SB_POSTFIX_ADJ_STEMS_RE, word) {
            if !group(&caps, 2).is_empty() {
                let head = group(&caps, 1);
                let singular = self
                    .sinoun(head, Some(&ONE), gender)
                    .unwrap_or_else(|| head.to_string());
                return Some(format!("{singular}{}", group(&caps, 2)));
            }
        }

        for sep in [' ', '-'] {
            if let Some(handled) = handle_prepositional_phrase(lowered, sep, |part| {
                self.sinoun(part, Some(&ONE), gender)
            }) {
                return Some(handled);
            }
        }

        if let Some((_, pronoun)) = prep_pronoun_split(&words) {
            if let Some(singular) = get_si_pron(Case::Acc, pronoun, gender) {
                return Some(format!("{}{singular}", chop(lowered, char_len(pronoun))));
            }
        }

        if let Some(singular) = get_si_pron(Case::Nom, lowered, gender) {
            return Some(singular.to_string());
        }
        if let Some(singular) = get_si_pron(Case::Acc, lowered, gender) {
            return Some(singular.to_string());
        }

        if let Some(singular) = SI_SB_IRREGULAR_CAPS.get(words.last) {
            return Some(format!("{}{singular}", chop(word, char_len(words.last))));
        }

        if let Some(singular) = SI_SB_IRREGULAR.get(lowered_last) {
            return Some(format!("{}{singular}", chop(word, char_len(lowered_last))));
        }

        let compound = last_two_dash_parts(lowered);
        if let Some(singular) = SI_SB_IRREGULAR_COMPOUND.get(compound.as_str()) {
            return Some(format!("{}{singular}", chop(word, char_len(&compound))));
        }

        if lowered.ends_with("quies") {
            return Some(format!("{}y", chop(word, 3)));
        }

        if lowered.ends_with("persons") {
            return Some(chop(word, 1).to_string());
        }
        if lowered.ends_with("people") {
            return Some(format!("{}rson", chop(word, 4)));
        }

        if lowered.ends_with("mans")
            && (ends_with_any(lowered, &U_MAN_MANS.plurals)
                || ends_with_any(word, &U_MAN_MANS_CAPS.plurals))
        {
            return Some(chop(word, 1).to_string());
        }
        if lowered.ends_with("men") {
            return Some(format!("{}man", chop(word, 3)));
        }
        if lowered.ends_with("mice") {
            return Some(format!("{}mouse", chop(word, 4)));
        }
        if lowered.ends_with("lice") && U_LOUSE_LICE.plurals.iter().any(|p| p == lowered) {
            return Some(format!("{}louse", chop(word, 4)));
        }
        if lowered.ends_with("geese") {
            return Some(format!("{}goose", chop(word, 5)));
        }
        if lowered.ends_with("teeth") {
            return Some(format!("{}tooth", chop(word, 5)));
        }
        if lowered.ends_with("feet") {
            return Some(format!("{}foot", chop(word, 4)));
        }

        if lowered == "dice" {
            return Some("die".to_string());
        }

        if lowered.ends_with("ceps") {
            return unchanged();
        }
        if lowered.ends_with("zoa") {
            return Some(format!("{}on", chop(word, 1)));
        }

        let last = last_char(lowered);
        let unassimilated: [(char, &[String], usize, &str); 7] = [
            ('s', &U_CH_CHS.plurals, 1, ""),
            ('s', &U_EX_ICES.plurals, 4, "ex"),
            ('s', &U_IX_ICES.plurals, 4, "ix"),
            ('a', &U_UM_A.plurals, 1, "um"),
            ('i', &U_US_I.plurals, 1, "us"),
            ('a', &U_ON_A.plurals, 1, "on"),
            ('e', &U_A_AE.plurals, 1, ""),
        ];
        for (lastlet, list, numend, post) in unassimilated {
            if last == Some(lastlet) && ends_with_any(lowered, list) {
                return Some(format!("{}{post}", chop(word, numend)));
            }
        }

        if self.classical.ancient {
            if lowered.ends_with("trices") {
                return Some(format!("{}x", chop(word, 3)));
            }
            if lowered.ends_with("eaux") || lowered.ends_with("ieux") {
                return Some(chop(word, 1).to_string());
            }
            if ["ynges", "inges", "anges"]
                .iter()
                .any(|e| lowered.ends_with(e))
                && char_len(word) > 6
            {
                return Some(format!("{}x", chop(word, 3)));
            }

            let c_us_us: Vec<String> = PL_SB_C_US_US.iter().map(|w| w.to_string()).collect();
            let incompletely_assimilated: [(char, &[String], usize, &str); 13] = [
                ('a', &C_EN_INA.plurals, 3, "en"),
                ('s', &C_EX_ICES.plurals, 4, "ex"),
                ('s', &C_IX_ICES.plurals, 4, "ix"),
                ('a', &C_UM_A.plurals, 1, "um"),
                ('i', &C_US_I.plurals, 1, "us"),
                ('s', &c_us_us, 0, ""),
                ('e', &C_A_AE.plurals, 1, ""),
                ('a', &C_A_ATA.plurals, 2, ""),
                ('s', &SI_SB_C_IS_IDES, 3, "s"),
                ('i', &C_O_I.plurals, 1, "o"),
                ('a', &C_ON_A.plurals, 1, "on"),
                ('m', &C_IM.plurals, 2, ""),
                ('i', &C_I.plurals, 1, ""),
            ];
            for (lastlet, list, numend, post) in incompletely_assimilated {
                if last == Some(lastlet) && ends_with_any(lowered, list) {
                    return Some(format!("{}{post}", chop(word, numend)));
                }
            }
        }

        if lowered.ends_with("houses")
            || SI_SB_USES_USE_CASE_SET.contains(word)
            || SI_SB_USES_USE_SET.contains(lowered_last)
            || SI_SB_IES_IE_CASE_SET.contains(word)
            || SI_SB_IES_IE_SET.contains(lowered_last)
            || lowered.ends_with("shoes")
            || SI_SB_OES_OE_CASE_SET.contains(word)
            || SI_SB_OES_OE_SET.contains(lowered_last)
            || SI_SB_SSES_SSE_CASE_SET.contains(word)
            || SI_SB_SSES_SSE_SET.contains(lowered_last)
        {
            return Some(chop(word, 1).to_string());
        }

        if SI_SB_SINGULAR_S_COMPLETE.contains(lowered_last)
            || ends_with_any(lowered, &SI_SB_SINGULAR_S_ENDINGS)
            || (lowered.ends_with("eses") && starts_upper(word))
            || SI_SB_Z_ZES_SET.contains(lowered_last)
            || SI_SB_ZZES_ZZ_SET.contains(lowered_last)
        {
            return Some(chop(word, 2).to_string());
        }

        if lowered.ends_with("zzes") {
            return Some(chop(word, 3).to_string());
        }

        if SI_SB_CHES_CHE_CASE_SET.contains(word) || SI_SB_CHES_CHE_SET.contains(lowered_last) {
            return Some(chop(word, 1).to_string());
        }
        if lowered.ends_with("ches") || lowered.ends_with("shes") {
            return Some(chop(word, 2).to_string());
        }

        if SI_SB_XES_XE_SET.contains(lowered_last) {
            return Some(chop(word, 1).to_string());
        }
        if lowered.ends_with("xes") {
            return Some(chop(word, 2).to_string());
        }

        if SI_SB_VES_VE_CASE_SET.contains(word) || SI_SB_VES_VE_SET.contains(lowered_last) {
            return Some(chop(word, 1).to_string());
        }

        if lowered.ends_with("ves") {
            let before = slice_from_end(lowered, 5, 3);
            if ["el", "al", "ol", "ar"].contains(&before)
                || (before == "ea" && slice_from_end(word, 6, 5) != "d")
            {
                return Some(format!("{}f", chop(word, 3)));
            }
            if ["ni", "li", "wi"].contains(&before) {
                return Some(format!("{}fe", chop(word, 3)));
            }
        }

        if lowered.ends_with("ys") {
            let vowel_before = lowered
                .chars()
                .rev()
                .nth(2)
                .is_some_and(|c| "aeiou".contains(c));
            if vowel_before || (self.classical.names && starts_upper(word)) {
                return Some(chop(word, 1).to_string());
            }
        }

        if lowered.ends_with("ies") {
            return Some(format!("{}y", chop(word, 3)));
        }

        if lowered.ends_with("os")
            && (SI_SB_U_O_OS_COMPLETE.contains(lowered_last)
                || ends_with_any(lowered, &SI_SB_U_O_OS_ENDINGS)
                || ["aos", "eos", "ios", "oos", "uos"]
                    .iter()
                    .any(|e| lowered.ends_with(e)))
        {
            return Some(chop(word, 1).to_string());
        }

        if lowered.ends_with("oes") {
            return Some(chop(word, 2).to_string());
        }

        if SI_SB_ES_IS_SET.contains(word) {
            return Some(format!("{}is", chop(word, 2)));
        }

        if last == Some('s') {
            return Some(chop(word, 1).to_string());
        }

        None
    }

    /// Prefixes `text` with "a" or "an" as appropriate.
    ///
    /// Leading and trailing whitespace is preserved, and an existing "a" or
    /// "an" is replaced.
    pub fn a(&self, text: &str) -> String {
        self.a_count(text, &ONE)
    }

    pub fn an(&self, text: &str) -> String {
        self.a(text)
    }

    /// Like [`a`](Self::a), but if `count` is not singular, prefixes `text`
    /// with the count instead.
    pub fn a_with_count(&self, text: &str, count: impl Into<Count>) -> String {
        self.a_count(text, &count.into())
    }

    fn a_count(&self, text: &str, count: &Count) -> String {
        let Some(caps) = captures(&INDEFINITE_ARTICLE_TEST, text) else {
            return String::new();
        };
        let (pre, word, post) = (group(&caps, 1), group(&caps, 2), group(&caps, 3));
        if word.is_empty() {
            return text.to_string();
        }
        format!("{pre}{}{post}", self.indef_article(word, count))
    }

    fn indef_article(&self, word: &str, count: &Count) -> String {
        if self.get_count(Some(count)) != Number::One {
            return format!("{count} {word}");
        }

        if let Some(article) = ud_match(word, &self.a_a_user_defined) {
            return format!("{article} {word}");
        }

        let article = INDEF_ARTICLE_CASES
            .iter()
            .find(|(re, _)| is_match(re, word))
            .map_or("a", |&(_, article)| article);
        format!("{article} {word}")
    }

    /// Returns "no" followed by the plural of `text` when the count is zero,
    /// otherwise the count followed by the appropriately inflected `text`.
    ///
    /// Uses the persistent count if one is set, otherwise zero.
    pub fn no(&self, text: &str) -> String {
        let count = Count::Num(self.persistent_count.unwrap_or(0));
        self.no_count(text, count)
    }

    pub fn no_with_count(&self, text: &str, count: impl Into<Count>) -> String {
        self.no_count(text, count.into())
    }

    fn no_count(&self, text: &str, count: Count) -> String {
        let (pre, word, post) = partition_word(text);
        let count = if PL_COUNT_ZERO_SET.contains(count.to_string().to_lowercase().as_str()) {
            Count::Word("no".to_string())
        } else {
            count
        };
        let plural = self.plural_count(word, Some(&count));
        format!("{pre}{count} {plural}{post}")
    }

    /// Returns the present participle of `word`, a verb in the 3rd person
    /// singular ("runs" -> "running").
    pub fn present_participle(&self, word: &str) -> String {
        let plural = self.plural_verb_with_count(word, 2);
        let participle = PRESENT_PARTICIPLE_REPLACEMENTS
            .iter()
            .find(|(re, _)| is_match(re, &plural))
            .map_or_else(
                || plural.clone(),
                |(re, replacement)| re.replace_all(&plural, *replacement).into_owned(),
            );
        format!("{participle}ing")
    }

    /// Joins words into a list: "ant, bee, and fly".
    pub fn join(&self, words: &[&str]) -> String {
        self.join_with(words, &JoinOptions::default())
    }

    pub fn join_with(&self, words: &[&str], options: &JoinOptions) -> String {
        match words {
            [] => return String::new(),
            [word] => return word.to_string(),
            _ => {}
        }

        let conj = if !options.conj_spaced {
            options.conj.clone()
        } else if options.conj.is_empty() {
            " ".to_string()
        } else {
            format!(" {} ", options.conj)
        };

        if let [first, second] = words {
            return format!("{first}{conj}{second}");
        }

        let mut sep = options.sep.clone().unwrap_or_else(|| {
            if words.iter().any(|w| w.contains(',')) {
                ";".to_string()
            } else {
                ",".to_string()
            }
        });
        let final_sep = format!("{}{conj}", options.final_sep.as_ref().unwrap_or(&sep));
        if options.sep_spaced {
            sep.push(' ');
        }

        let (last, init) = words.split_last().expect("at least three words");
        format!("{}{final_sep}{last}", init.join(&sep))
    }
}

fn last_two_dash_parts(lowered: &str) -> String {
    let parts: Vec<&str> = lowered.split('-').collect();
    parts[parts.len().saturating_sub(2)..].join(" ")
}

fn handle_prepositional_phrase(
    phrase: &str,
    sep: char,
    transform: impl FnOnce(&str) -> Option<String>,
) -> Option<String> {
    let parts: Vec<&str> = phrase.split(sep).collect();
    if parts.len() < 3 {
        return None;
    }
    let pivot = (1..parts.len() - 1).find(|&i| PL_PREP_SET.contains(parts[i]))?;
    let transformed = transform(parts[pivot - 1])
        .filter(|t| !t.is_empty())
        .unwrap_or_else(|| parts[pivot - 1].to_string());
    let sep = sep.to_string();
    let joined = [transformed.as_str(), parts[pivot], ""].join(&sep);
    let head: Vec<&str> = parts[..pivot - 1]
        .iter()
        .copied()
        .chain([joined.as_str()])
        .collect();
    Some(head.join(" ") + &parts[pivot + 1..].join(" "))
}

fn pl_check_plurals_n(word1: &str, word2: &str) -> bool {
    let (last1, last2) = (Words::new(word1).last, Words::new(word2).last);
    let pair = format!("{last1}|{last2}");
    let single_separator = !last1.contains('|') && !last2.contains('|');
    PL_SB_IRREGULAR_VALUES.contains(pair.as_str())
        || PLURAL_STEM_ENDINGS.iter().any(|(re, end1, end2)| {
            let possible = !single_separator || last1.ends_with(end1) || last1.ends_with(end2);
            possible && is_match(re, &pair)
        })
}

fn pl_check_plurals_adj(word1: &str, word2: &str) -> bool {
    let strip_possessive = |word: &str| -> String {
        if word.ends_with("'s") || word.ends_with('\'') {
            word[..word.rfind('\'').unwrap_or(0)].to_string()
        } else {
            String::new()
        }
    };
    let (word1, word2) = (strip_possessive(word1), strip_possessive(word2));
    !word1.is_empty()
        && !word2.is_empty()
        && (pl_check_plurals_n(&word1, &word2) || pl_check_plurals_n(&word2, &word1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn si_pron() {
        assert_eq!(
            get_si_pron(Case::Acc, "them", Gender::Masculine),
            Some("him")
        );
        assert_eq!(
            get_si_pron(Case::Acc, "them", Gender::Feminine),
            Some("her")
        );
        assert_eq!(get_si_pron(Case::Acc, "them", Gender::Neuter), Some("it"));
        assert_eq!(get_si_pron(Case::Acc, "you", Gender::Neuter), Some("you"));
        assert_eq!(get_si_pron(Case::Nom, "we", Gender::Neuter), Some("I"));
        assert_eq!(get_si_pron(Case::Acc, "itself", Gender::Neuter), None);
    }

    #[test]
    fn prepositional_phrase() {
        let upper = |s: &str| Some(s.to_uppercase());
        assert_eq!(
            handle_prepositional_phrase("man-of-war", '-', upper).as_deref(),
            Some("MAN-of-war")
        );
        assert_eq!(
            handle_prepositional_phrase("man of war", ' ', upper).as_deref(),
            Some("MAN of war")
        );
        assert_eq!(handle_prepositional_phrase("man-war", '-', upper), None);
    }

    #[test]
    fn long_compounds() {
        let e = Engine::new();
        let words = Words::new("pair of scissors");
        assert_eq!(
            e.handle_long_compounds(&words, Number::Many).as_deref(),
            Some("pairs of scissors")
        );
        let words = Words::new("men beyond hills");
        assert_eq!(
            e.handle_long_compounds(&words, Number::One).as_deref(),
            Some("man beyond hills")
        );
    }

    #[test]
    fn user_defined_expansion() {
        let defs = [user_defined("(.*)x", "$1y$$2").unwrap()];
        assert_eq!(ud_match("box", &defs).as_deref(), Some("boy$"));
        assert_eq!(ud_match("cat", &defs), None);
    }
}
