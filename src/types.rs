use std::fmt;
use std::str::FromStr;

use crate::Error;

/// A count used to decide between singular and plural forms.
///
/// Numbers and words are both accepted, mirroring the Python library: `1`,
/// `"one"`, `"a"`, `"each"` and so on count as singular, anything else as
/// plural.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Count {
    Num(i64),
    Word(String),
}

impl fmt::Display for Count {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Count::Num(n) => write!(f, "{n}"),
            Count::Word(w) => f.write_str(w),
        }
    }
}

macro_rules! count_from_int {
    ($($t:ty),*) => {
        $(impl From<$t> for Count {
            fn from(n: $t) -> Self {
                match i64::try_from(n) {
                    Ok(n) => Count::Num(n),
                    Err(_) => Count::Word(n.to_string()),
                }
            }
        })*
    };
}

count_from_int!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

impl From<&str> for Count {
    fn from(s: &str) -> Self {
        Count::Word(s.to_string())
    }
}

impl From<String> for Count {
    fn from(s: String) -> Self {
        Count::Word(s)
    }
}

impl From<&Count> for Count {
    fn from(c: &Count) -> Self {
        c.clone()
    }
}

/// The gender used when singularizing third-person plural pronouns.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Gender {
    #[default]
    Neuter,
    Feminine,
    Masculine,
    GenderNeutral,
    FeminineOrMasculine,
    MasculineOrFeminine,
}

impl Gender {
    pub const ALL: [Gender; 6] = [
        Gender::Neuter,
        Gender::Feminine,
        Gender::Masculine,
        Gender::GenderNeutral,
        Gender::FeminineOrMasculine,
        Gender::MasculineOrFeminine,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Gender::Neuter => "neuter",
            Gender::Feminine => "feminine",
            Gender::Masculine => "masculine",
            Gender::GenderNeutral => "gender-neutral",
            Gender::FeminineOrMasculine => "feminine or masculine",
            Gender::MasculineOrFeminine => "masculine or feminine",
        }
    }
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Gender {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Gender::ALL
            .into_iter()
            .find(|g| g.as_str() == s)
            .ok_or_else(|| Error::BadGender(s.to_string()))
    }
}

/// Which classical (Latin/Greek or archaic) inflections to prefer.
///
/// The default enables only `names`, matching the Python library.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Classical {
    /// Prefer classical plurals everywhere a choice exists ("formulae", "indices").
    pub all: bool,
    /// Treat zero as singular when given as a word count ("no cat" rather than "no cats").
    pub zero: bool,
    /// Leave herd animals uninflected ("buffalo" rather than "buffalos").
    pub herd: bool,
    /// Pluralize proper names ending in -y with -s ("Marys" rather than "Maries").
    pub names: bool,
    /// Use "persons" rather than "people".
    pub persons: bool,
    /// Use classical plurals for incompletely assimilated imports ("cherubim").
    pub ancient: bool,
}

impl Classical {
    pub const fn all() -> Self {
        Classical {
            all: true,
            zero: true,
            herd: true,
            names: true,
            persons: true,
            ancient: true,
        }
    }

    pub const fn none() -> Self {
        Classical {
            all: false,
            zero: false,
            herd: false,
            names: false,
            persons: false,
            ancient: false,
        }
    }
}

impl Default for Classical {
    fn default() -> Self {
        Classical {
            names: true,
            ..Classical::none()
        }
    }
}

/// The result of comparing two words regardless of plurality.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Comparison {
    /// The words are identical.
    Eq,
    /// The first word is the plural of the second.
    PluralSingular,
    /// The second word is the plural of the first.
    SingularPlural,
    /// The words are two different plural forms of the same word.
    PluralPlural,
}

impl Comparison {
    pub fn as_str(self) -> &'static str {
        match self {
            Comparison::Eq => "eq",
            Comparison::PluralSingular => "p:s",
            Comparison::SingularPlural => "s:p",
            Comparison::PluralPlural => "p:p",
        }
    }
}

impl fmt::Display for Comparison {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Options for [`Engine::number_to_words_with`](crate::Engine::number_to_words_with).
#[derive(Clone, Debug, PartialEq)]
pub struct NumberToWordsOptions {
    /// Read digits in groups of 1, 2 or 3 ("one two three"); 0 reads the whole number.
    pub group: u8,
    pub comma: String,
    /// The word between hundreds and tens ("one hundred and one"); may be empty.
    pub andword: String,
    pub zero: String,
    pub one: String,
    /// The word for a decimal point; `None` disables decimal handling.
    pub decimal: Option<String>,
    /// Numbers above this are returned as digits (with thousands commas) instead of words.
    pub threshold: Option<i64>,
}

impl Default for NumberToWordsOptions {
    fn default() -> Self {
        NumberToWordsOptions {
            group: 0,
            comma: ",".to_string(),
            andword: "and".to_string(),
            zero: "zero".to_string(),
            one: "one".to_string(),
            decimal: Some("point".to_string()),
            threshold: None,
        }
    }
}

/// Options for [`Engine::join_with`](crate::Engine::join_with).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JoinOptions {
    /// Separator between items; defaults to "," or ";" if any item contains a comma.
    pub sep: Option<String>,
    pub sep_spaced: bool,
    /// Separator before the conjunction; defaults to `sep`.
    pub final_sep: Option<String>,
    pub conj: String,
    pub conj_spaced: bool,
}

impl Default for JoinOptions {
    fn default() -> Self {
        JoinOptions {
            sep: None,
            sep_spaced: true,
            final_sep: None,
            conj: "and".to_string(),
            conj_spaced: true,
        }
    }
}
