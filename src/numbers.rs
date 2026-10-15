use std::fmt::Display;
use std::sync::LazyLock;

use fancy_regex::{Captures, Regex};

use crate::data::{nth, MILL, NTH_SUFF, ORDINAL, TEEN, TEN, UNIT};
use crate::text::{chop, regex};
use crate::{Engine, Error, NumberToWordsOptions};

static DIGIT_GROUP: LazyLock<Regex> = LazyLock::new(|| regex(r"(\d)"));
static TWO_DIGITS: LazyLock<Regex> = LazyLock::new(|| regex(r"(\d)(\d)"));
static THREE_DIGITS: LazyLock<Regex> = LazyLock::new(|| regex(r"(\d)(\d)(\d)"));
static THREE_DIGITS_WORD: LazyLock<Regex> = LazyLock::new(|| regex(r"(\d)(\d)(\d)(?=\D*\z)"));
static TWO_DIGITS_WORD: LazyLock<Regex> = LazyLock::new(|| regex(r"(\d)(\d)(?=\D*\z)"));
static ONE_DIGIT_WORD: LazyLock<Regex> = LazyLock::new(|| regex(r"(\d)(?=\D*\z)"));
static FOUR_DIGIT_COMMA: LazyLock<Regex> = LazyLock::new(|| regex(r"(\d)(\d{3}(?:,|\z))"));
static NON_DIGIT: LazyLock<Regex> = LazyLock::new(|| regex(r"\D"));
static WHITESPACES_COMMA: LazyLock<Regex> = LazyLock::new(|| regex(r"\s+,"));
static COMMA_WORD: LazyLock<Regex> = LazyLock::new(|| regex(r", (\S+)\s+\z"));
static WHITESPACES: LazyLock<Regex> = LazyLock::new(|| regex(r"\s+"));

fn sub(
    re: &Regex,
    text: &str,
    limit: Option<usize>,
    mut replace: impl FnMut(&Captures<'_, str>) -> Result<String, Error>,
) -> Result<String, Error> {
    let mut out = String::with_capacity(text.len());
    let mut last = 0;
    for caps in re.captures_iter(text).take(limit.unwrap_or(usize::MAX)) {
        let Ok(caps) = caps else { break };
        let whole = caps.get(0).expect("group 0 always matches");
        out.push_str(&text[last..whole.start()]);
        out.push_str(&replace(&caps)?);
        last = whole.end();
    }
    out.push_str(&text[last..]);
    Ok(out)
}

fn digit(caps: &Captures<'_, str>, index: usize) -> usize {
    caps.get(index)
        .and_then(|m| m.as_str().chars().next())
        .and_then(|c| c.to_digit(10))
        .map_or(0, |d| d as usize)
}

fn millfn(index: usize) -> Result<&'static str, Error> {
    MILL.get(index).copied().ok_or(Error::NumOutOfRange)
}

fn unitfn(units: usize, mindex: usize) -> Result<String, Error> {
    Ok(format!("{}{}", UNIT[units], millfn(mindex)?))
}

fn tenfn(tens: usize, units: usize, mindex: usize) -> Result<String, Error> {
    if tens == 1 {
        return Ok(format!("{}{}", TEEN[units], millfn(mindex)?));
    }
    let hyphen = if tens > 0 && units > 0 { "-" } else { "" };
    Ok(format!(
        "{}{hyphen}{}{}",
        TEN[tens],
        UNIT[units],
        millfn(mindex)?
    ))
}

fn sub_ord(val: &str) -> String {
    ORDINAL
        .iter()
        .find_map(|(cardinal, ordinal)| {
            val.strip_suffix(cardinal)
                .map(|stem| format!("{stem}{ordinal}"))
        })
        .unwrap_or_else(|| format!("{val}th"))
}

fn last_two_digits(s: &str) -> u32 {
    let s = s.trim();
    let digits = s.len() - s.trim_end_matches(|c: char| c.is_ascii_digit()).len();
    let start = s.len() - digits.min(2);
    s[start..].parse().unwrap_or(0)
}

struct Words<'a> {
    andword: &'a str,
    zero: &'a str,
    one: &'a str,
    mill_count: usize,
}

impl Words<'_> {
    fn hundfn(&self, hundreds: usize, tens: usize, units: usize) -> Result<String, Error> {
        if hundreds == 0 && tens == 0 && units == 0 {
            return Ok(String::new());
        }
        let mill = millfn(self.mill_count)?;
        if hundreds > 0 {
            let andword = if tens > 0 || units > 0 {
                format!(" {} ", self.andword)
            } else {
                String::new()
            };
            return Ok(format!(
                "{} hundred{andword}{}{mill}, ",
                UNIT[hundreds],
                tenfn(tens, units, 0)?
            ));
        }
        Ok(format!("{}{mill}, ", tenfn(tens, units, 0)?))
    }

    fn group1sub(&self, caps: &Captures<'_, str>) -> Result<String, Error> {
        Ok(match digit(caps, 1) {
            1 => format!(" {}, ", self.one),
            0 => format!(" {}, ", self.zero),
            units => format!("{}, ", UNIT[units]),
        })
    }

    fn group1bsub(&self, caps: &Captures<'_, str>) -> Result<String, Error> {
        Ok(match digit(caps, 1) {
            0 => format!(" {}, ", self.zero),
            units => format!("{}, ", UNIT[units]),
        })
    }

    fn group2sub(&self, caps: &Captures<'_, str>) -> Result<String, Error> {
        let (tens, units) = (digit(caps, 1), digit(caps, 2));
        if tens > 0 {
            return Ok(format!("{}, ", tenfn(tens, units, 0)?));
        }
        if units > 0 {
            return Ok(format!(" {} {}, ", self.zero, UNIT[units]));
        }
        Ok(format!(" {} {}, ", self.zero, self.zero))
    }

    fn group3sub(&self, caps: &Captures<'_, str>) -> Result<String, Error> {
        let (hundreds, tens, units) = (digit(caps, 1), digit(caps, 2), digit(caps, 3));
        let hunword = match hundreds {
            1 => format!(" {}", self.one),
            0 => format!(" {}", self.zero),
            h => UNIT[h].to_string(),
        };
        let tenword = if tens > 0 {
            tenfn(tens, units, 0)?
        } else if units > 0 {
            format!(" {} {}", self.zero, UNIT[units])
        } else {
            format!(" {} {}", self.zero, self.zero)
        };
        Ok(format!("{hunword} {tenword}, "))
    }

    fn enword(&mut self, num: &str, group: u8) -> Result<String, Error> {
        match group {
            1 => sub(&DIGIT_GROUP, num, None, |c| self.group1sub(c)),
            2 => {
                let num = sub(&TWO_DIGITS, num, None, |c| self.group2sub(c))?;
                sub(&DIGIT_GROUP, &num, Some(1), |c| self.group1bsub(c))
            }
            3 => {
                let num = sub(&THREE_DIGITS, num, None, |c| self.group3sub(c))?;
                let num = sub(&TWO_DIGITS, &num, Some(1), |c| self.group2sub(c))?;
                sub(&DIGIT_GROUP, &num, Some(1), |c| self.group1sub(c))
            }
            _ => {
                let mut num = num.trim_start().trim_start_matches('0').to_string();
                if num.chars().all(|c| c == '0') {
                    return Ok(self.zero.to_string());
                }
                if num == "1" {
                    return Ok(self.one.to_string());
                }
                self.mill_count = 0;
                while THREE_DIGITS_WORD.is_match(&num).unwrap_or(false) {
                    num = sub(&THREE_DIGITS_WORD, &num, Some(1), |c| {
                        let words = self.hundfn(digit(c, 1), digit(c, 2), digit(c, 3))?;
                        self.mill_count += 1;
                        Ok(words)
                    })?;
                }
                let num = sub(&TWO_DIGITS_WORD, &num, Some(1), |c| {
                    Ok(format!(
                        "{}, ",
                        tenfn(digit(c, 1), digit(c, 2), self.mill_count)?
                    ))
                })?;
                sub(&ONE_DIGIT_WORD, &num, Some(1), |c| {
                    Ok(format!("{}, ", unitfn(digit(c, 1), self.mill_count)?))
                })
            }
        }
    }
}

fn render(chunks: &[String], decimal: Option<&str>, comma: &str) -> String {
    let Some((first_item, rest)) = chunks.split_first() else {
        return String::new();
    };
    let mut out = first_item.clone();
    let mut first = decimal.is_none_or(|d| !first_item.ends_with(d));
    for chunk in rest {
        if Some(chunk.as_str()) == decimal {
            first = false;
        } else if first {
            out.push_str(comma);
        }
        out.push(' ');
        out.push_str(chunk);
    }
    out
}

enum Spelled {
    Threshold(String),
    Chunks {
        sign: &'static str,
        chunks: Vec<String>,
    },
}

impl Engine {
    /// Returns the ordinal of a number ("1" -> "1st") or of a number word
    /// ("one" -> "first").
    pub fn ordinal(&self, num: impl Display) -> String {
        let num = num.to_string();
        if !num.starts_with(|c: char| c.is_ascii_digit()) {
            return sub_ord(&num);
        }
        let n = if !num.contains('.') {
            last_two_digits(&num)
        } else if num.ends_with(|c: char| c.is_ascii_digit()) {
            last_two_digits(crate::text::tail(&num, 1))
        } else {
            last_two_digits(chop(&num, 1))
        };
        let suffix = nth(n % 100).or_else(|| nth(n % 10)).unwrap_or("th");
        format!("{num}{suffix}")
    }

    /// Returns a number in words: 1234 -> "one thousand, two hundred and thirty-four".
    pub fn number_to_words(&self, num: impl Display) -> Result<String, Error> {
        self.number_to_words_with(num, &NumberToWordsOptions::default())
    }

    pub fn number_to_words_with(
        &self,
        num: impl Display,
        options: &NumberToWordsOptions,
    ) -> Result<String, Error> {
        Ok(match spell(&num.to_string(), options)? {
            Spelled::Threshold(s) => s,
            Spelled::Chunks { sign, chunks } => {
                let value = if options.group > 0 {
                    chunks.join(", ")
                } else {
                    render(&chunks, options.decimal.as_deref(), &options.comma)
                };
                if sign.is_empty() {
                    value
                } else {
                    format!("{sign} {value}")
                }
            }
        })
    }

    /// Like [`number_to_words_with`](Self::number_to_words_with), returning
    /// the separate chunks ("one thousand", "two hundred and thirty-four").
    pub fn number_to_words_list(
        &self,
        num: impl Display,
        options: &NumberToWordsOptions,
    ) -> Result<Vec<String>, Error> {
        Ok(match spell(&num.to_string(), options)? {
            Spelled::Threshold(s) => vec![s],
            Spelled::Chunks { sign, chunks } => {
                let sign = (!sign.is_empty()).then(|| sign.to_string());
                sign.into_iter().chain(chunks).collect()
            }
        })
    }
}

fn spell(num: &str, options: &NumberToWordsOptions) -> Result<Spelled, Error> {
    let bad_value = || Error::BadNumValue(num.to_string());

    if let Some(threshold) = options.threshold {
        let value: f64 = num.trim().parse().map_err(|_| bad_value())?;
        if value > threshold as f64 {
            let mut parts = num.splitn(2, '.');
            let mut whole = parts.next().unwrap_or_default().to_string();
            while !options.comma.is_empty() && FOUR_DIGIT_COMMA.is_match(&whole).unwrap_or(false) {
                whole = FOUR_DIGIT_COMMA
                    .replace_all(&whole, "${1},${2}")
                    .into_owned();
            }
            return Ok(Spelled::Threshold(match parts.next() {
                Some(fraction) => format!("{whole}.{fraction}"),
                None => whole,
            }));
        }
    }

    if options.group > 3 {
        return Err(Error::BadChunkingOption);
    }
    let group = options.group;

    let sign = match num.trim_start().chars().next() {
        Some('+') => "plus",
        Some('-') => "minus",
        Some(_) => "",
        None => return Err(bad_value()),
    };

    let mut num = num.to_string();
    if NTH_SUFF.contains(&num.as_str()) {
        num = options.zero.clone();
    }
    let myord = NTH_SUFF.contains(&crate::text::tail(&num, 2));
    if myord {
        num = chop(&num, 2).to_string();
    }

    let decimal = options.decimal.as_deref().filter(|d| !d.is_empty());
    let mut chunks: Vec<String> = match decimal {
        Some(_) if group == 0 => num.splitn(2, '.').map(String::from).collect(),
        Some(_) => num.split('.').map(String::from).collect(),
        None => vec![num],
    };
    let finalpoint = chunks.last().is_some_and(|c| c.is_empty());
    if finalpoint {
        chunks.pop();
    }

    let loopstart = chunks.first().is_some_and(|c| c.is_empty());
    let mut first = Some(!loopstart);
    let mut words = Words {
        andword: &options.andword,
        zero: &options.zero,
        one: &options.one,
        mill_count: 0,
    };
    for chunk in chunks.iter_mut().skip(usize::from(loopstart)) {
        let mut digits = NON_DIGIT.replace_all(chunk, "").into_owned();
        if digits.is_empty() {
            digits = "0".to_string();
        }
        let is_first = first == Some(true);
        let mut text = if group == 0 && !is_first {
            words.enword(&digits, 1)?
        } else {
            words.enword(&digits, group)?
        };
        if let Some(stripped) = text.strip_suffix(", ") {
            text = stripped.to_string();
        }
        text = WHITESPACES_COMMA.replace_all(&text, ",").into_owned();
        if group == 0 && is_first {
            text = sub(&COMMA_WORD, &text, None, |c| {
                Ok(format!(" {} {}", options.andword, crate::text::group(c, 1)))
            })?;
        }
        text = WHITESPACES.replace_all(&text, " ").trim().to_string();
        if is_first {
            first = None;
        }
        *chunk = text;
    }

    let separator = format!("{} ", options.comma);
    let mut numchunks: Vec<String> = Vec::new();
    if first != Some(false) {
        numchunks.extend(chunks[0].split(&separator).map(String::from));
    }
    if myord {
        if let Some(last) = numchunks.last_mut() {
            *last = sub_ord(last);
        }
    }
    let decimal = options.decimal.clone().unwrap_or_default();
    for chunk in chunks.iter().skip(1) {
        numchunks.push(decimal.clone());
        numchunks.extend(chunk.split(&separator).map(String::from));
    }
    if finalpoint {
        numchunks.push(decimal);
    }

    Ok(Spelled::Chunks {
        sign,
        chunks: numchunks,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinal_suffixes() {
        assert_eq!(sub_ord("one"), "first");
        assert_eq!(sub_ord("twenty"), "twentieth");
        assert_eq!(sub_ord("four"), "fourth");
        assert_eq!(last_two_digits("123456"), 56);
    }

    #[test]
    fn tens() {
        assert_eq!(tenfn(2, 1, 0).unwrap(), "twenty-one ");
        assert_eq!(tenfn(1, 3, 1).unwrap(), "thirteen thousand");
        assert_eq!(tenfn(0, 0, 12), Err(Error::NumOutOfRange));
    }
}
