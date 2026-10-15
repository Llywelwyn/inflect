# inflect-rs

[![ci](https://github.com/Llywelwyn/inflect_rs/actions/workflows/cargo-build-test.yml/badge.svg)](https://github.com/Llywelwyn/inflect_rs/actions)
[![crates.io](https://img.shields.io/crates/v/inflect-rs.svg)](https://crates.io/crates/inflect-rs)

## NAME

inflect-rs - Accurately generate plurals, singular nouns, ordinals, indefinite articles, and word-based representations of numbers. This functionality is limited to English.

inflect-rs is a Rust port of the Python [inflect](https://pypi.org/project/inflect/) library, version 7.5.0.

## SYNOPSIS

```rust
use inflect_rs::Engine;

let p = Engine::new();
```

Simple example with pluralization and word-representation of numbers:

```rust
# use inflect_rs::Engine;
# let p = Engine::new();
let count = 1;
let sentence = format!(
    "There {} {} {} by the door.",
    p.plural_verb_with_count("was", count),
    p.number_to_words(count).unwrap(),
    p.plural_noun_with_count("person", count),
);
assert_eq!(sentence, "There was one person by the door.");
```

When `count = 243`, the same code will generate:

```text
There were two hundred and forty-three people by the door.
```

## Methods

- `plural`, `plural_noun`, `plural_verb`, `plural_adj`, `singular_noun`, `no`, `num`
- `compare`, `compare_nouns`, `compare_verbs`, `compare_adjs`
- `a`, `an`
- `present_participle`
- `ordinal`, `number_to_words`
- `join`
- `inflect`, `classical`, `set_gender`
- `defnoun`, `defverb`, `defadj`, `defa`, `defan`

Where the Python methods take an optional count, the Rust methods come in
pairs: `plural(word)` and `plural_with_count(word, count)`, `a(word)` and
`a_with_count(word, count)`, and so on. A count can be a number or a word
such as `"one"`, `"each"` or `"no"`.

### Plurality/Singularity

#### Unconditionally Form the Plural

```rust
# use inflect_rs::Engine;
# let p = Engine::new();
assert_eq!(
    format!("the plural of person is {}", p.plural("person")),
    "the plural of person is people"
);
```

#### Conditionally Form the Plural

```rust
# use inflect_rs::Engine;
# let p = Engine::new();
assert_eq!(
    format!("the plural of 1 person is {}", p.plural_with_count("person", 1)),
    "the plural of 1 person is person"
);
```

#### Form Plurals for Specific Parts of Speech

```rust
# use inflect_rs::Engine;
# let p = Engine::new();
assert_eq!(p.plural_noun_with_count("I", 2), "we");
assert_eq!(p.plural_verb_with_count("saw", 1), "saw");
assert_eq!(p.plural_adj_with_count("my", 2), "our");
assert_eq!(p.plural_noun_with_count("saw", 2), "saws");
```

#### Form the Singular of Plural Nouns

```rust
# use inflect_rs::Engine;
# let p = Engine::new();
assert_eq!(
    format!("The singular of people is {}", p.singular_noun("people").unwrap()),
    "The singular of people is person"
);
```

`singular_noun` returns `None` if the word does not look like a plural noun.

#### Select the Gender of Singular Pronouns

```rust
# use inflect_rs::{Engine, Gender};
let mut p = Engine::new();
assert_eq!(p.singular_noun("they").as_deref(), Some("it"));
p.set_gender(Gender::Feminine);
assert_eq!(p.singular_noun("they").as_deref(), Some("she"));
```

#### Deal with "0/1/N" -> "no/1/N" Translation

```rust
# use inflect_rs::Engine;
# let p = Engine::new();
let errors = 1;
assert_eq!(
    format!("There {}{}", p.plural_verb_with_count("was", errors), p.no_with_count(" error", errors)),
    "There was 1 error"
);
let errors = 2;
assert_eq!(
    format!("There {}{}", p.plural_verb_with_count("was", errors), p.no_with_count(" error", errors)),
    "There were 2 errors"
);
```

#### Use Default Counts

```rust
# use inflect_rs::Engine;
let mut p = Engine::new();
p.num(Some(1));
assert_eq!(p.plural("I"), "I");
assert_eq!(p.plural_verb(" saw"), " saw");
assert_eq!(p.num(Some(2)), "2");
assert_eq!(p.plural_noun(" saw"), " saws");

let errors = 2;
p.num(Some(errors));
assert_eq!(
    format!("There {}{}", p.plural_verb("was"), p.no(" error")),
    "There were 2 errors"
);
```

#### Compare Two Words Number-Insensitively

```rust
# use inflect_rs::{Comparison, Engine};
# let p = Engine::new();
assert_eq!(p.compare("person", "person"), Some(Comparison::Eq));
assert_eq!(p.compare("person", "people"), Some(Comparison::SingularPlural));
assert_eq!(p.compare_nouns("person", "people"), Some(Comparison::SingularPlural));
assert_eq!(p.compare_verbs("run", "ran"), None);
assert_eq!(p.compare_verbs("run", "running"), None);
assert_eq!(p.compare_verbs("run", "run"), Some(Comparison::Eq));
assert_eq!(p.compare_adjs("my", "mine"), None);
assert_eq!(p.compare_adjs("my", "our"), Some(Comparison::SingularPlural));
```

#### Add Correct *a* or *an* for a Given Word

```rust
# use inflect_rs::Engine;
# let p = Engine::new();
assert_eq!(
    format!("Did you want {} or {}", p.a("thing"), p.a("idea")),
    "Did you want a thing or an idea"
);
```

#### Convert Numerals into Ordinals

```rust
# use inflect_rs::Engine;
# let p = Engine::new();
assert_eq!(format!("It was {} from the left", p.ordinal(1)), "It was 1st from the left");
assert_eq!(format!("It was {} from the left", p.ordinal(2)), "It was 2nd from the left");
assert_eq!(format!("It was {} from the left", p.ordinal(3)), "It was 3rd from the left");
assert_eq!(format!("It was {} from the left", p.ordinal(347)), "It was 347th from the left");
```

#### Convert Numerals to Words

Note: This returns a single string.

```rust
# use inflect_rs::Engine;
# let p = Engine::new();
assert_eq!(p.number_to_words(1).unwrap(), "one");
assert_eq!(p.number_to_words(38).unwrap(), "thirty-eight");
assert_eq!(p.number_to_words(1234).unwrap(), "one thousand, two hundred and thirty-four");
assert_eq!(
    p.number_to_words(p.ordinal(1234)).unwrap(),
    "one thousand, two hundred and thirty-fourth"
);
```

#### Retrieve Words as List of Parts

```rust
# use inflect_rs::{Engine, NumberToWordsOptions};
# let p = Engine::new();
assert_eq!(
    p.number_to_words_list(1234, &NumberToWordsOptions::default()).unwrap(),
    ["one thousand", "two hundred and thirty-four"]
);
```

#### Grouping Options

```rust
# use inflect_rs::{Engine, NumberToWordsOptions};
# let p = Engine::new();
let options = |group| NumberToWordsOptions { group, ..Default::default() };

assert_eq!(p.number_to_words_with(12345, &options(1)).unwrap(), "one, two, three, four, five");
assert_eq!(p.number_to_words_with(12345, &options(2)).unwrap(), "twelve, thirty-four, five");
assert_eq!(p.number_to_words_with(12345, &options(3)).unwrap(), "one twenty-three, forty-five");

let andword = |andword: &str| NumberToWordsOptions { andword: andword.into(), ..Default::default() };
assert_eq!(
    p.number_to_words_with(1234, &andword("")).unwrap(),
    "one thousand, two hundred thirty-four"
);
assert_eq!(
    p.number_to_words_with(1234, &andword(", plus")).unwrap(),
    "one thousand, two hundred, plus thirty-four"
);

let zero = NumberToWordsOptions { group: 1, zero: "oh".into(), ..Default::default() };
assert_eq!(p.number_to_words_with(555_1202, &zero).unwrap(), "five, five, five, one, two, oh, two");
let one = NumberToWordsOptions { group: 1, one: "unity".into(), ..Default::default() };
assert_eq!(p.number_to_words_with(555_1202, &one).unwrap(), "five, five, five, unity, two, zero, two");
let decimal = NumberToWordsOptions { group: 1, decimal: Some("mark".into()), ..Default::default() };
assert_eq!(p.number_to_words_with(123.456, &decimal).unwrap(), "one, two, three, mark, four, five, six");
```

#### Apply Threshold for Word-Representation of Numbers

Above provided threshold, numerals will remain numerals

```rust
# use inflect_rs::{Engine, NumberToWordsOptions};
# let p = Engine::new();
let options = NumberToWordsOptions { threshold: Some(10), ..Default::default() };
assert_eq!(p.number_to_words_with(9, &options).unwrap(), "nine");
assert_eq!(p.number_to_words_with(10, &options).unwrap(), "ten");
assert_eq!(p.number_to_words_with(11, &options).unwrap(), "11");
assert_eq!(p.number_to_words_with(1000, &options).unwrap(), "1,000");
```

#### Join Words into a List

```rust
# use inflect_rs::{Engine, JoinOptions};
# let p = Engine::new();
assert_eq!(p.join(&["apple", "banana", "carrot"]), "apple, banana, and carrot");
assert_eq!(p.join(&["apple", "banana"]), "apple and banana");

let final_sep = JoinOptions { final_sep: Some("".into()), ..Default::default() };
assert_eq!(p.join_with(&["apple", "banana", "carrot"], &final_sep), "apple, banana and carrot");

let conj = JoinOptions { conj: "and even".into(), ..Default::default() };
assert_eq!(p.join_with(&["apples", "bananas", "carrots"], &conj), "apples, bananas, and even carrots");

let slashes = JoinOptions {
    sep: Some("/".into()),
    sep_spaced: false,
    conj: "".into(),
    conj_spaced: false,
    ..Default::default()
};
assert_eq!(p.join_with(&["apple", "banana", "carrot"], &slashes), "apple/banana/carrot");
```

#### Require Classical Plurals

Adhere to conventions from Classical Latin and Classical Greek

```rust
# use inflect_rs::{Classical, Engine};
let mut p = Engine::new();
p.classical = Classical::all();
assert_eq!(p.plural_noun_with_count("focus", 2), "foci");
assert_eq!(p.plural_noun_with_count("cherubim", 2), "cherubims");
assert_eq!(p.plural_noun_with_count("cherub", 2), "cherubim");
```

Other options for classical plurals:

```rust
# use inflect_rs::{Classical, Engine};
# let mut p = Engine::new();
p.classical = Classical::all(); // USE ALL CLASSICAL PLURALS
p.classical = Classical::none(); // SWITCH OFF CLASSICAL MODE

p.classical.zero = true; // "no error" INSTEAD OF "no errors"
p.classical.zero = false; // "no errors" INSTEAD OF "no error"

p.classical.herd = true; // "2 buffalo" INSTEAD OF "2 buffalos"
p.classical.herd = false; // "2 buffalos" INSTEAD OF "2 buffalo"

p.classical.persons = true; // "2 chairpersons" INSTEAD OF "2 chairpeople"
p.classical.persons = false; // "2 chairpeople" INSTEAD OF "2 chairpersons"

p.classical.ancient = true; // "2 formulae" INSTEAD OF "2 formulas"
p.classical.ancient = false; // "2 formulas" INSTEAD OF "2 formulae"
```

#### Support for interpolation

Supports string interpolation with the following functions: `plural()`, `plural_noun()`, `plural_verb()`, `plural_adj()`, `singular_noun()`, `a()`, `an()`, `no()`, `num()`, `ordinal()`, `number_to_words()` and `present_participle()`. Arguments are written as in Python, so strings must be quoted.

```rust
# use inflect_rs::Engine;
let mut p = Engine::new();
assert_eq!(
    p.inflect(&format!("The plural of {0} is plural('{0}')", "car")).unwrap(),
    "The plural of car is cars"
);
assert_eq!(
    p.inflect(&format!("The singular of {0} is singular_noun('{0}')", "cars")).unwrap(),
    "The singular of cars is car"
);
assert_eq!(
    p.inflect(&format!("I saw {0} plural('cat',{0})", 3)).unwrap(),
    "I saw 3 cats"
);
assert_eq!(
    p.inflect(&format!(
        "plural('I',{0}) plural_verb('saw',{0}) plural('a',{1}) plural_noun('saw',{1})",
        1, 2
    ))
    .unwrap(),
    "I saw some saws"
);
assert_eq!(
    p.inflect(&format!(
        "num({0}, False)plural('I') plural_verb('saw') num({1}, False)plural('a') plural_noun('saw')",
        1, 1
    ))
    .unwrap(),
    "I saw a saw"
);
assert_eq!(
    p.inflect(&format!(
        "num({0}, False)plural('I') plural_verb('saw') num({1}, False)plural('a') plural_noun('saw')",
        2, 2
    ))
    .unwrap(),
    "we saw some saws"
);
assert_eq!(
    p.inflect(&format!("I saw num({0}) plural('cat')\nnum()", 3)).unwrap(),
    "I saw 3 cats\n"
);
let errors = 2;
assert_eq!(
    p.inflect(&format!("There plural_verb('was',{0}) no('error',{0})", errors)).unwrap(),
    "There were 2 errors"
);
assert_eq!(
    p.inflect(&format!("There num({0}, False)plural_verb('was') no('error')", errors)).unwrap(),
    "There were 2 errors"
);
assert_eq!(
    p.inflect(&format!("Did you want a('{0}') or an('{1}')", "thing", "idea")).unwrap(),
    "Did you want a thing or an idea"
);
assert_eq!(
    p.inflect(&format!("It was ordinal('{0}') from the left", 2)).unwrap(),
    "It was 2nd from the left"
);
```

#### Add User-Defined Inflections

Allows for overriding default rules. Patterns are regular expressions, and
`$1`, `$2`, ... in a replacement refer to groups captured by the pattern.

Override noun defaults:

```rust
# use inflect_rs::Engine;
# let mut p = Engine::new();
p.defnoun("VAX", "VAXen").unwrap(); // SINGULAR => PLURAL
```

Override Verb defaults:

```rust
# use inflect_rs::Engine;
# let mut p = Engine::new();
p.defverb(
    "will",  // 1ST PERSON SINGULAR
    "shall", // 1ST PERSON PLURAL
    "will",  // 2ND PERSON SINGULAR
    "will",  // 2ND PERSON PLURAL
    "will",  // 3RD PERSON SINGULAR
    "will",  // 3RD PERSON PLURAL
)
.unwrap();
```

Override adjective defaults:

```rust
# use inflect_rs::Engine;
# let mut p = Engine::new();
p.defadj("hir", "their").unwrap();
assert_eq!(p.plural_adj_with_count("hir", 2), "their");
```

Override the words that use the indefinite articles "a" or "an":

```rust
# use inflect_rs::Engine;
# let mut p = Engine::new();
assert_eq!(p.a_with_count("ape", 1), "an ape");
p.defa("a").unwrap();
assert_eq!(p.a_with_count("ape", 1), "an ape");
p.defa("ape").unwrap();
assert_eq!(p.a_with_count("ape", 1), "a ape");
p.defan("horrendous.*").unwrap();
assert_eq!(p.a_with_count("horrendous affectation", 1), "an horrendous affectation");
```

## DESCRIPTION

The methods of `inflect_rs::Engine` provide plural inflections, singular
noun inflections, "a"/"an" selection for English words, and manipulation of
numbers as words.

Plural forms of all nouns, most verbs, and some adjectives are
provided. Where appropriate, "classical" variants (for example: "brother" ->
"brethren", "dogma" -> "dogmata", etc.) are also provided.

Single forms of nouns are also provided. The gender of singular pronouns
can be chosen (for example "they" -> "it" or "she" or "he" or "they").

Pronunciation-based "a"/"an" selection is provided for all English
words, and most initialisms.

It is also possible to inflect numerals (1,2,3) to ordinals (1st, 2nd, 3rd)
or to English words ("one", "two", "three").

In generating these inflections, inflect-rs follows the Oxford
English Dictionary and the guidelines in Fowler's Modern English
Usage, preferring the former where the two disagree.

The library is built around standard British spelling, but is designed
to cope with common American variants as well. Slang, jargon, and
other English dialects are *not* explicitly catered for.

Where two or more inflected forms exist for a single word (typically a
"classical" form and a "modern" form), inflect-rs prefers the
more common form (typically the "modern" one), unless "classical"
processing has been specified
(see [Require Classical Plurals](#require-classical-plurals)).

## FORMING PLURALS AND SINGULARS

### Inflecting Plurals and Singulars

All of the `plural...` plural inflection methods take the word to be
inflected as their first argument and return the corresponding inflection.
Note that all such methods expect the *singular* form of the word. The
results of passing a plural form are undefined (and unlikely to be correct).
Similarly, the `singular_noun` singular inflection method expects the *plural*
form of the word.

The `plural..._with_count` methods also take a second argument,
which indicates the grammatical "number" of the word (or of another word
with which the word being inflected must agree). If the "number" argument is
not `1` (or `"one"` or `"a"`, or some other adjective that
implies the singular), the plural form of the word is returned. If the
"number" argument *does* indicate singularity, the (uninflected) word
itself is returned. The methods without a count return the plural form
unconditionally (unless a default count has been set with `num`).

`singular_noun_with_count` takes a second argument in a similar fashion. If it
is some form of the number `1`, the singular form is returned.
Otherwise the plural is returned unaltered. `singular_noun` returns the
singular form.

The various methods of `inflect_rs::Engine` are:

`plural_noun(word)`, `plural_noun_with_count(word, count)`

> The method `plural_noun()` takes a *singular* English noun or
> pronoun and returns its plural. Pronouns in the nominative ("I" ->
> "we") and accusative ("me" -> "us") cases are handled, as are
> possessive pronouns ("mine" -> "ours").

`plural_verb(word)`, `plural_verb_with_count(word, count)`

> The method `plural_verb()` takes the *singular* form of a
> conjugated verb (that is, one which is already in the correct "person"
> and "mood") and returns the corresponding plural conjugation.

`plural_adj(word)`, `plural_adj_with_count(word, count)`

> The method `plural_adj()` takes the *singular* form of
> certain types of adjectives and returns the corresponding plural form.
> Adjectives that are correctly handled include: "numerical" adjectives
> ("a" -> "some"), demonstrative adjectives ("this" -> "these", "that" ->
> "those"), and possessives ("my" -> "our", "cat's" -> "cats'", "child's"
> -> "children's", etc.)

`plural(word)`, `plural_with_count(word, count)`

> The method `plural()` takes a *singular* English noun,
> pronoun, verb, or adjective and returns its plural form. Where a word
> has more than one inflection depending on its part of speech (for
> example, the noun "thought" inflects to "thoughts", the verb "thought"
> to "thought"), the (singular) noun sense is preferred to the (singular)
> verb sense.
>
> Hence `plural("knife")` will return "knives" ("knife" having been treated
> as a singular noun), whereas `plural("knifes")` will return "knife"
> ("knifes" having been treated as a 3rd person singular verb).
>
> The inherent ambiguity of such cases suggests that,
> where the part of speech is known, `plural_noun`, `plural_verb`, and
> `plural_adj` should be used in preference to `plural`.

`singular_noun(word)`, `singular_noun_with_count(word, count)`, `singular_noun_with_gender(word, gender)`

> The method `singular_noun()` takes a *plural* English noun or
> pronoun and returns its singular. Pronouns in the nominative ("we" ->
> "I") and accusative ("us" -> "me") cases are handled, as are
> possessive pronouns ("ours" -> "mine"). When third person
> singular pronouns are returned they take the neuter gender by default
> ("they" -> "it"), not ("they"-> "she") nor ("they" -> "he"). This can be
> changed with `set_gender()`, or for a single call with
> `singular_noun_with_gender()`.

Note that all these methods ignore any whitespace surrounding the
word being inflected, but preserve that whitespace when the result is
returned. For example, `plural(" cat  ")` returns `" cats  "`.

`set_gender(gender)`

> The third person plural pronoun takes the same form for the female, male and
> neuter (e.g. "they"). The singular however, depends upon gender (e.g. "she",
> "he", "it" and "they" -- "they" being the gender neutral form.) By default
> `singular_noun` returns the neuter form, however, the gender can be selected
> with the `set_gender` method. Pass a `Gender` (`Neuter`, `Feminine`,
> `Masculine`, `GenderNeutral`, `FeminineOrMasculine` or
> `MasculineOrFeminine`) to select the form of the singular. e.g.
> `set_gender(Gender::Feminine)` followed by `singular_noun("themselves")`
> returns "herself".

### Numbered plurals

The `plural...` methods return only the inflected word, not the count that
was used to inflect it. Thus, in order to produce "I saw 3 ducks", it
is necessary to use:

```rust
# use inflect_rs::Engine;
# let p = Engine::new();
# let (n, animal) = (3, "duck");
println!("I saw {} {}", n, p.plural_noun_with_count(animal, n));
```

Since the usual purpose of producing a plural is to make it agree with
a preceding count, inflect-rs provides a method
(`no_with_count(word, count)`) which, given a word and a count, returns the
count followed by the correctly inflected word. Hence the previous
example can be rewritten:

```rust
# use inflect_rs::Engine;
# let p = Engine::new();
# let (n, animal) = (3, "duck");
println!("I saw {}", p.no_with_count(animal, n));
```

In addition, if the count is zero (or some other term which implies
zero, such as `"zero"`, `"nil"`, etc.) the count is replaced by the
word "no". Hence, if `n` had the value zero, the previous example
would print (the somewhat more elegant):

```text
I saw no animals
```

rather than:

```text
I saw 0 animals
```

Note that the name of the method is a pun: the method
returns either a number (a *No.*) or a `"no"`, in front of the
inflected word.

### Reducing the number of counts required

In some contexts, the need to supply an explicit count to the various
`plural...` methods makes for tiresome repetition. For example:

```rust
# use inflect_rs::Engine;
# let p = Engine::new();
# let errors = 2;
println!(
    "{}{}{} fatal.",
    p.plural_adj_with_count("This", errors),
    p.plural_noun_with_count(" error", errors),
    p.plural_verb_with_count(" was", errors),
);
```

inflect-rs therefore provides a method
(`num(count)`) which may be used to set a persistent "default number"
value. If such a value is set, it is subsequently used whenever the
methods without a count are called. The default value thus set
can subsequently be removed by calling `num(None)`.
Hence we could rewrite the previous example:

```rust
# use inflect_rs::Engine;
# let mut p = Engine::new();
# let errors = 2;
p.num(Some(errors));
println!("{}{}{} fatal.", p.plural_adj("This"), p.plural_noun(" error"), p.plural_verb(" was"));
p.num(None);
```

`num()` returns its argument as a string (or an empty string for `None`), so
that it may also be "inlined" in contexts like:

```rust
# use inflect_rs::Engine;
# let mut p = Engine::new();
# let (errors, severity) = (2, 2);
println!("{}{}{} detected.", p.num(Some(errors)), p.plural_noun(" error"), p.plural_verb(" was"));
if severity > 1 {
    println!("{}{}{} fatal.", p.plural_adj("This"), p.plural_noun(" error"), p.plural_verb(" was"));
}
```

Within strings passed to `inflect` (see
[Support for interpolation](#support-for-interpolation)) it is
preferable that `num()` return an empty string. Hence `num()` there accepts
an optional second argument. If that argument is supplied and evaluates to
false, `num` returns an empty string instead of its first argument. For
example:

```rust
# use inflect_rs::Engine;
# let mut p = Engine::new();
assert_eq!(
    p.inflect("num(2, False)no('error') plural_verb('was') detected.").unwrap(),
    "2 errors were detected."
);
```

### Number-insensitive equality

inflect-rs also provides a solution to the problem
of comparing words of differing plurality through the methods
`compare(word1, word2)`, `compare_nouns(word1, word2)`,
`compare_verbs(word1, word2)`, and `compare_adjs(word1, word2)`.
Each of these methods takes two strings, and compares them
using the corresponding plural-inflection method (`plural()`, `plural_noun()`,
`plural_verb()`, and `plural_adj()` respectively).

The comparison succeeds if:

- the strings are equal, or
- one string is equal to a plural form of the other, or
- the strings are two different plural forms of the one word.

Hence all of the following succeed:

```rust
# use inflect_rs::{Comparison::*, Engine};
# let p = Engine::new();
assert_eq!(p.compare("index", "index"), Some(Eq));
assert_eq!(p.compare("index", "indexes"), Some(SingularPlural));
assert_eq!(p.compare("index", "indices"), Some(SingularPlural));
assert_eq!(p.compare("indexes", "index"), Some(PluralSingular));
assert_eq!(p.compare("indices", "index"), Some(PluralSingular));
assert_eq!(p.compare("indices", "indexes"), Some(PluralPlural));
assert_eq!(p.compare("indexes", "indices"), Some(PluralPlural));
assert_eq!(p.compare("indices", "indices"), Some(Eq));
```

As indicated by the previous example, the actual value
returned by the various `compare` methods encodes which of the
three equality rules succeeded: `Eq` is returned if the strings were
identical, `SingularPlural` if the strings were singular and plural
respectively, `PluralSingular` for plural and singular, and `PluralPlural`
for two distinct plurals. `Comparison::as_str()` gives the Python library's
codes ("eq", "s:p", "p:s" and "p:p"). Inequality is indicated by returning
`None`.

It should be noted that two distinct singular words which happen to take
the same plural form are *not* considered equal, nor are cases where
one (singular) word's plural is the other (plural) word's singular.
Hence all of the following return `None`:

```rust
# use inflect_rs::Engine;
# let p = Engine::new();
assert_eq!(p.compare("base", "basis"), None); // ALTHOUGH BOTH -> "bases"
assert_eq!(p.compare("syrinx", "syringe"), None); // ALTHOUGH BOTH -> "syringes"
assert_eq!(p.compare("she", "he"), None); // ALTHOUGH BOTH -> "they"

assert_eq!(p.compare("opus", "operas"), None); // ALTHOUGH "opus" -> "opera" -> "operas"
assert_eq!(p.compare("taxi", "taxes"), None); // ALTHOUGH "taxi" -> "taxis" -> "taxes"
```

Note too that, although the comparison is "number-insensitive" it is *not*
case-insensitive (that is, `compare("time", "Times")` returns `None`). To
obtain both number and case insensitivity, use `to_lowercase()` on both strings
(that is, `compare(&"time".to_lowercase(), &"Times".to_lowercase())`
succeeds).

## Differences from the Python library

- Optional arguments become separate methods (`plural` and
  `plural_with_count`), and number and join options are passed as
  `NumberToWordsOptions` and `JoinOptions`.
- `singular_noun` returns `Option<String>` (`None` where Python returns
  `False`), and the comparison methods return `Option<Comparison>`.
- Classical modes are the public fields of `Engine::classical`, and genders
  are the `Gender` enum, so invalid modes and genders cannot be expressed.
- Errors are returned as `Result<_, inflect_rs::Error>` instead of raised.
- A few inputs that crash the Python library (whitespace-only text, malformed
  numbers passed to `ordinal`) return a best-effort result instead.

## Related Functionality

inflect-rs is a port of the Python [inflect](https://github.com/jaraco/inflect)
library, which is itself based upon the Perl module
[Lingua::EN::Inflect](https://metacpan.org/pod/Lingua::EN::Inflect).
