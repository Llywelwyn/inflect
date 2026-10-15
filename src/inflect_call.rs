use std::sync::LazyLock;

use fancy_regex::Regex;

use crate::text::{group, regex};
use crate::{Count, Engine, Error, Gender, NumberToWordsOptions};

static FUNCTION_CALL: LazyLock<Regex> = LazyLock::new(|| regex(r"((\w+)\([^)]*\)*)"));

const METHODS: &[(&str, &[&str])] = &[
    ("plural", &["text", "count"]),
    ("plural_adj", &["text", "count"]),
    ("plural_noun", &["text", "count"]),
    ("plural_verb", &["text", "count"]),
    ("singular_noun", &["text", "count", "gender"]),
    ("a", &["text", "count"]),
    ("an", &["text", "count"]),
    ("no", &["text", "count"]),
    ("ordinal", &["num"]),
    (
        "number_to_words",
        &[
            "num",
            "wantlist",
            "group",
            "comma",
            "andword",
            "zero",
            "one",
            "decimal",
            "threshold",
        ],
    ),
    ("present_participle", &["word"]),
    ("num", &["count", "show"]),
];

#[derive(Clone, Debug, PartialEq)]
enum Value {
    Str(String),
    Int(String),
    Float(String),
    Bool(bool),
    None,
    Tuple(Vec<Value>),
    List(Vec<Value>),
}

impl Value {
    fn repr(&self) -> String {
        match self {
            Value::Str(s) if s.contains('\'') && !s.contains('"') => {
                format!("\"{}\"", s.replace('\\', "\\\\"))
            }
            Value::Str(s) => format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'")),
            Value::Tuple(items) if items.len() == 1 => format!("({},)", items[0].repr()),
            Value::Tuple(items) => format!("({})", join_reprs(items)),
            Value::List(items) => format!("[{}]", join_reprs(items)),
            other => other.str(),
        }
    }

    fn str(&self) -> String {
        match self {
            Value::Str(s) | Value::Int(s) | Value::Float(s) => s.clone(),
            Value::Bool(true) => "True".to_string(),
            Value::Bool(false) => "False".to_string(),
            Value::None => "None".to_string(),
            other => other.repr(),
        }
    }

    fn truthy(&self) -> bool {
        match self {
            Value::Str(s) => !s.is_empty(),
            Value::Int(s) | Value::Float(s) => s.chars().any(|c| ('1'..='9').contains(&c)),
            Value::Bool(b) => *b,
            Value::None => false,
            Value::Tuple(items) | Value::List(items) => !items.is_empty(),
        }
    }
}

fn join_reprs(items: &[Value]) -> String {
    items.iter().map(Value::repr).collect::<Vec<_>>().join(", ")
}

type Call<'a> = (Vec<Value>, Vec<(&'a str, Value)>);

struct Parser<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    fn skip_ws(&mut self) {
        while self.peek().is_some_and(char::is_whitespace) {
            self.bump();
        }
    }

    fn eat(&mut self, c: char) -> bool {
        self.skip_ws();
        if self.peek() == Some(c) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn ident(&mut self) -> &'a str {
        let start = self.pos;
        while self.peek().is_some_and(|c| c.is_alphanumeric() || c == '_') {
            self.bump();
        }
        &self.src[start..self.pos]
    }

    fn string(&mut self, quote: char) -> Option<Value> {
        let mut s = String::new();
        loop {
            match self.bump()? {
                c if c == quote => return Some(Value::Str(s)),
                '\\' => match self.bump()? {
                    'n' => s.push('\n'),
                    't' => s.push('\t'),
                    c @ ('\\' | '\'' | '"') => s.push(c),
                    c => {
                        s.push('\\');
                        s.push(c);
                    }
                },
                c => s.push(c),
            }
        }
    }

    fn number(&mut self) -> Option<Value> {
        let start = self.pos;
        if self.peek() == Some('-') {
            self.bump();
        }
        while self
            .peek()
            .is_some_and(|c| c.is_ascii_digit() || c == '_' || c == '.')
        {
            self.bump();
        }
        let text = self.src[start..self.pos].replace('_', "");
        if text.is_empty() || text == "-" {
            return None;
        }
        Some(if text.contains('.') {
            Value::Float(text)
        } else {
            let negative = text.starts_with('-');
            let digits = text.trim_start_matches('-').trim_start_matches('0');
            let digits = if digits.is_empty() { "0" } else { digits };
            Value::Int(format!(
                "{}{digits}",
                if negative && digits != "0" { "-" } else { "" }
            ))
        })
    }

    fn sequence(&mut self, close: char) -> Option<(Vec<Value>, bool)> {
        let mut items = Vec::new();
        let mut trailing_comma = false;
        loop {
            if self.eat(close) {
                return Some((items, trailing_comma));
            }
            items.push(self.value()?);
            trailing_comma = self.eat(',');
            if !trailing_comma {
                return self.eat(close).then_some((items, false));
            }
        }
    }

    fn value(&mut self) -> Option<Value> {
        self.skip_ws();
        match self.peek()? {
            q @ ('\'' | '"') => {
                self.bump();
                self.string(q)
            }
            '(' => {
                self.bump();
                let (mut items, trailing_comma) = self.sequence(')')?;
                Some(if items.len() == 1 && !trailing_comma {
                    items.remove(0)
                } else {
                    Value::Tuple(items)
                })
            }
            '[' => {
                self.bump();
                Some(Value::List(self.sequence(']')?.0))
            }
            c if c.is_ascii_digit() || c == '-' || c == '.' => self.number(),
            _ => match self.ident() {
                "True" => Some(Value::Bool(true)),
                "False" => Some(Value::Bool(false)),
                "None" => Some(Value::None),
                _ => None,
            },
        }
    }

    fn call(&mut self) -> Option<Call<'a>> {
        self.ident();
        if !self.eat('(') {
            return None;
        }
        let mut args = Vec::new();
        let mut kwargs = Vec::new();
        loop {
            if self.eat(')') {
                break;
            }
            self.skip_ws();
            let checkpoint = self.pos;
            let name = self.ident();
            if !name.is_empty() && self.eat('=') {
                kwargs.push((name, self.value()?));
            } else {
                self.pos = checkpoint;
                if !kwargs.is_empty() {
                    return None;
                }
                args.push(self.value()?);
            }
            if !self.eat(',') {
                if !self.eat(')') {
                    return None;
                }
                break;
            }
        }
        self.skip_ws();
        (self.pos == self.src.len()).then_some((args, kwargs))
    }
}

fn bind(
    params: &[&str],
    args: Vec<Value>,
    kwargs: Vec<(&str, Value)>,
) -> Option<Vec<Option<Value>>> {
    if args.len() > params.len() {
        return None;
    }
    let mut bound: Vec<Option<Value>> = vec![None; params.len()];
    for (slot, arg) in bound.iter_mut().zip(args) {
        *slot = Some(arg);
    }
    for (name, value) in kwargs {
        let index = params.iter().position(|p| *p == name)?;
        if bound[index].replace(value).is_some() {
            return None;
        }
    }
    Some(bound)
}

impl Engine {
    /// Performs the inflections written as function calls inside `text`.
    ///
    /// `"The plural of cat is plural('cat')"` becomes
    /// `"The plural of cat is cats"`. The available functions are `plural`,
    /// `plural_noun`, `plural_verb`, `plural_adj`, `singular_noun`, `a`, `an`,
    /// `no`, `ordinal`, `number_to_words`, `present_participle` and `num`,
    /// taking the same arguments as the Python library.
    pub fn inflect(&mut self, text: &str) -> Result<String, Error> {
        let saved_count = self.persistent_count();
        let result = self.substitute_calls(text);
        self.num(saved_count);
        result
    }

    fn substitute_calls(&mut self, text: &str) -> Result<String, Error> {
        let mut out = String::with_capacity(text.len());
        let mut last = 0;
        for caps in FUNCTION_CALL.captures_iter(text) {
            let Ok(caps) = caps else { break };
            let whole = caps.get(0).expect("group 0 always matches");
            out.push_str(&text[last..whole.start()]);
            out.push_str(&self.substitute(group(&caps, 1), group(&caps, 2))?);
            last = whole.end();
        }
        out.push_str(&text[last..]);
        Ok(out)
    }

    fn substitute(&mut self, call: &str, name: &str) -> Result<String, Error> {
        let Some(&(_, params)) = METHODS.iter().find(|(method, _)| *method == name) else {
            return Ok(call.to_string());
        };
        let bad_call = || Error::BadInflectCall(call.to_string());
        let mut parser = Parser { src: call, pos: 0 };
        let (args, kwargs) = parser.call().ok_or_else(bad_call)?;
        let mut bound = bind(params, args, kwargs).ok_or_else(bad_call)?.into_iter();
        let mut next = || bound.next().flatten();

        let text = |value: Option<Value>| match value {
            Some(Value::Str(s)) if !s.is_empty() => Ok(s),
            _ => Err(bad_call()),
        };
        let count = |value: Option<Value>| match value {
            None | Some(Value::None) => None,
            Some(Value::Int(n)) => Some(n.parse().map_or(Count::Word(n), Count::Num)),
            Some(other) => Some(Count::Word(other.str())),
        };

        Ok(match name {
            "plural" | "plural_adj" | "plural_noun" | "plural_verb" => {
                let (word, count) = (text(next())?, count(next()));
                let count = count.as_ref();
                match name {
                    "plural" => self.plural_count(&word, count),
                    "plural_adj" => self.plural_adj_count(&word, count),
                    "plural_noun" => self.plural_noun_count(&word, count),
                    _ => self.plural_verb_count(&word, count),
                }
            }
            "singular_noun" => {
                let (word, count) = (text(next())?, count(next()));
                let gender = match next() {
                    None | Some(Value::None) => None,
                    Some(Value::Str(g)) => Some(g.parse::<Gender>()?),
                    Some(_) => return Err(bad_call()),
                };
                self.singular_noun_count(&word, count.as_ref(), gender)
                    .ok_or_else(bad_call)?
            }
            "a" | "an" => {
                let word = text(next())?;
                let count = match next() {
                    None => Count::Num(1),
                    Some(value) => count(Some(value)).unwrap_or(Count::Word("None".into())),
                };
                self.a_with_count(&word, count)
            }
            "no" => {
                let word = text(next())?;
                match count(next()) {
                    Some(count) => self.no_with_count(&word, count),
                    None => self.no(&word),
                }
            }
            "ordinal" => match next() {
                Some(Value::Str(s)) if !s.is_empty() => self.ordinal(s),
                Some(n @ (Value::Int(_) | Value::Float(_))) => self.ordinal(n.str()),
                _ => return Err(bad_call()),
            },
            "number_to_words" => {
                let num = match next() {
                    Some(Value::Str(s)) if !s.is_empty() => s,
                    Some(n @ (Value::Int(_) | Value::Float(_))) => n.str(),
                    _ => return Err(bad_call()),
                };
                if next().is_some_and(|wantlist| wantlist.truthy()) {
                    return Err(bad_call());
                }
                let defaults = NumberToWordsOptions::default();
                let string_or = |value: Option<Value>, default: String| match value {
                    None => Ok(default),
                    Some(Value::Str(s)) => Ok(s),
                    Some(_) => Err(bad_call()),
                };
                let options = NumberToWordsOptions {
                    group: match next() {
                        None => 0,
                        Some(Value::Int(n)) => n.parse().map_err(|_| Error::BadChunkingOption)?,
                        Some(_) => return Err(bad_call()),
                    },
                    comma: match next() {
                        None => defaults.comma,
                        Some(Value::Str(s)) => s,
                        Some(Value::Bool(false) | Value::None) => String::new(),
                        Some(_) => return Err(bad_call()),
                    },
                    andword: string_or(next(), defaults.andword)?,
                    zero: string_or(next(), defaults.zero)?,
                    one: string_or(next(), defaults.one)?,
                    decimal: match next() {
                        None => defaults.decimal,
                        Some(Value::Str(s)) => Some(s),
                        Some(Value::Bool(false) | Value::None) => None,
                        Some(_) => return Err(bad_call()),
                    },
                    threshold: match next() {
                        None | Some(Value::None) => None,
                        Some(Value::Int(n)) => Some(n.parse().map_err(|_| bad_call())?),
                        Some(_) => return Err(bad_call()),
                    },
                };
                self.number_to_words_with(num, &options)?
            }
            "present_participle" => self.present_participle(&text(next())?),
            "num" => {
                let value = next();
                let show = next();
                let count = match &value {
                    None | Some(Value::None) => None,
                    Some(Value::Bool(b)) => Some(i64::from(*b)),
                    Some(Value::Float(f)) => Some(
                        f.parse::<f64>()
                            .map_err(|_| Error::BadNumValue(f.clone()))?
                            .trunc() as i64,
                    ),
                    Some(other) => {
                        let s = other.str();
                        Some(s.trim().parse().map_err(|_| Error::BadNumValue(s))?)
                    }
                };
                self.num(count);
                match (&value, show) {
                    (None | Some(Value::None), _) => String::new(),
                    (_, Some(show)) if show != Value::None && !show.truthy() => String::new(),
                    (Some(value), _) => value.str(),
                }
            }
            _ => unreachable!("method table and dispatch agree"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(call: &str) -> Option<Call<'_>> {
        Parser { src: call, pos: 0 }.call()
    }

    #[test]
    fn parses_literals() {
        let (args, kwargs) = parse("f('a', 2, (1,), ['x'], None, andword='')").unwrap();
        assert_eq!(
            args,
            vec![
                Value::Str("a".into()),
                Value::Int("2".into()),
                Value::Tuple(vec![Value::Int("1".into())]),
                Value::List(vec![Value::Str("x".into())]),
                Value::None,
            ]
        );
        assert_eq!(kwargs, vec![("andword", Value::Str(String::new()))]);
        assert_eq!(parse("f(cat)"), None);
        assert_eq!(parse("f('cat'))"), None);
    }

    #[test]
    fn reprs_like_python() {
        let tuple = Value::Tuple(vec![Value::Str("a".into()), Value::Str("b".into())]);
        assert_eq!(tuple.str(), "('a', 'b')");
        assert_eq!(Value::Str("it's".into()).repr(), "\"it's\"");
        assert!(!Value::Int("0".into()).truthy());
        assert!(Value::Int("3".into()).truthy());
        assert!(!Value::Str(String::new()).truthy());
    }
}
