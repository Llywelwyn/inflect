use inflect_rs::*;

#[derive(PartialEq)]
enum Kind {
    Verb,
    Noun,
    Any,
}

fn pluralize(e: &Engine, kind: &Kind, word: &str) -> String {
    match kind {
        Kind::Verb => e.plural_verb(word),
        Kind::Noun => e.plural_noun(word),
        Kind::Any => e.plural(word),
    }
}

fn is_eq(e: &Engine, a: &str, b: &str) -> Option<Comparison> {
    e.compare(a, b)
        .or_else(|| e.compare_nouns(a, b))
        .or_else(|| e.compare_verbs(a, b))
        .or_else(|| e.compare_adjs(a, b))
}

#[test]
fn many() {
    let mut e = Engine::new();
    let mut failures = Vec::new();

    for line in include_str!("inflections.txt").lines().map(str::trim) {
        if line.contains("TODO:") {
            continue;
        }
        let Some((singular, rest)) = line.split_once("->") else {
            continue;
        };
        let singular = singular.trim();
        let (plural, comment) = rest.trim().split_once('#').unwrap_or((rest.trim(), ""));
        let (mod_plural, class_plural) = match plural.split_once('|') {
            Some((m, c)) => (m.trim(), c.trim()),
            None => (plural.trim(), plural.trim()),
        };
        let comment = comment.to_lowercase();
        let kind = if comment.contains("verb") {
            Kind::Verb
        } else if comment.contains("noun") {
            Kind::Noun
        } else {
            Kind::Any
        };

        e.classical = Classical::none();
        let mod_pl = pluralize(&e, &kind, singular);
        e.classical = Classical::all();
        let class_pl = pluralize(&e, &kind, singular);

        assert!(!singular.is_empty());
        if mod_pl != mod_plural {
            failures.push(format!(
                "{singular}: modern plural {mod_pl:?} != {mod_plural:?}"
            ));
        }
        if class_pl != class_plural {
            failures.push(format!(
                "{singular}: classical plural {class_pl:?} != {class_plural:?}"
            ));
        }
        for (a, b) in [
            (singular, mod_plural),
            (mod_plural, singular),
            (singular, class_plural),
            (class_plural, singular),
        ] {
            let cmp = is_eq(&e, a, b);
            if !matches!(
                cmp,
                Some(Comparison::Eq | Comparison::SingularPlural | Comparison::PluralSingular)
            ) {
                failures.push(format!("compare({a:?}, {b:?}) == {cmp:?}"));
            }
        }
        if kind != Kind::Verb {
            for plural in [mod_plural, class_plural] {
                let got = e.singular_noun_with_count(plural, 1);
                if got.as_deref() != Some(singular) {
                    failures.push(format!(
                        "singular_noun({plural:?}, 1) == {got:?} != {singular:?}"
                    ));
                }
            }
        }
    }

    assert!(
        failures.is_empty(),
        "{} failures:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

#[test]
fn def() {
    let mut e = Engine::new();

    e.defnoun("kin", "kine").unwrap();
    e.defnoun("(.*)x", "$1xen").unwrap();
    e.defverb("foobar", "feebar", "foobar", "feebar", "foobars", "feebar")
        .unwrap();
    e.defadj("red", "red|gules").unwrap();

    assert_eq!(e.no_with_count("kin", 0), "no kine");
    assert_eq!(e.no_with_count("kin", 1), "1 kin");
    assert_eq!(e.no_with_count("kin", 2), "2 kine");

    assert_eq!(e.no_with_count("regex", 0), "no regexen");

    assert_eq!(e.plural_with_count("foobar", 2), "feebar");
    assert_eq!(e.plural_with_count("foobars", 2), "feebar");

    assert_eq!(e.plural_with_count("red", 0), "red");
    assert_eq!(e.plural_with_count("red", 1), "red");
    assert_eq!(e.plural_with_count("red", 2), "red");
    e.classical = Classical::all();
    assert_eq!(e.plural_with_count("red", 0), "red");
    assert_eq!(e.plural_with_count("red", 1), "red");
    assert_eq!(e.plural_with_count("red", 2), "gules");
}

#[test]
fn ordinal() {
    let e = Engine::new();
    for (n, expected) in [
        (0, "0th"),
        (1, "1st"),
        (2, "2nd"),
        (3, "3rd"),
        (4, "4th"),
        (5, "5th"),
        (6, "6th"),
        (7, "7th"),
        (8, "8th"),
        (9, "9th"),
        (10, "10th"),
        (11, "11th"),
        (12, "12th"),
        (13, "13th"),
        (14, "14th"),
        (15, "15th"),
        (16, "16th"),
        (17, "17th"),
        (18, "18th"),
        (19, "19th"),
        (20, "20th"),
        (21, "21st"),
        (22, "22nd"),
        (23, "23rd"),
        (24, "24th"),
        (100, "100th"),
        (101, "101st"),
        (102, "102nd"),
        (103, "103rd"),
        (104, "104th"),
    ] {
        assert_eq!(e.ordinal(n), expected);
    }
    assert_eq!(e.ordinal("1"), "1st");

    assert_eq!(e.ordinal(1.1), "1.1st");
    assert_eq!(e.ordinal("1.1"), "1.1st");
    assert_eq!(e.ordinal(1.2), "1.2nd");
    assert_eq!(e.ordinal(5.502), "5.502nd");

    for (word, expected) in [
        ("zero", "zeroth"),
        ("one", "first"),
        ("two", "second"),
        ("three", "third"),
        ("four", "fourth"),
        ("five", "fifth"),
        ("six", "sixth"),
        ("seven", "seventh"),
        ("eight", "eighth"),
        ("nine", "ninth"),
        ("ten", "tenth"),
        ("eleven", "eleventh"),
        ("twelve", "twelfth"),
        ("thirteen", "thirteenth"),
        ("fourteen", "fourteenth"),
        ("fifteen", "fifteenth"),
        ("sixteen", "sixteenth"),
        ("seventeen", "seventeenth"),
        ("eighteen", "eighteenth"),
        ("nineteen", "nineteenth"),
        ("twenty", "twentieth"),
        ("twenty-one", "twenty-first"),
        ("twenty-two", "twenty-second"),
        ("twenty-three", "twenty-third"),
        ("twenty-four", "twenty-fourth"),
        ("one hundred", "one hundredth"),
        ("one hundred and one", "one hundred and first"),
        ("one hundred and two", "one hundred and second"),
        ("one hundred and three", "one hundred and third"),
        ("one hundred and four", "one hundred and fourth"),
    ] {
        assert_eq!(e.ordinal(word), expected);
    }
}

#[test]
fn decimal_ordinals() {
    let e = Engine::new();
    assert_eq!(e.ordinal("1.23"), "1.23rd");
    assert_eq!(e.ordinal("7.09"), "7.09th");
}

#[test]
fn prespart() {
    let e = Engine::new();
    for (verb, participle) in [
        ("sees", "seeing"),
        ("eats", "eating"),
        ("bats", "batting"),
        ("hates", "hating"),
        ("spies", "spying"),
        ("skis", "skiing"),
    ] {
        assert_eq!(e.present_participle(verb), participle);
    }
}

fn check_inflect(cases: &[(&str, &str)]) {
    let mut e = Engine::new();
    for (input, expected) in cases {
        assert_eq!(e.inflect(input).unwrap(), *expected, "inflect({input:?})");
    }
}

#[test]
fn inflect_on_tuples() {
    check_inflect(&[
        ("plural('egg', ('a', 'b', 'c'))", "eggs"),
        ("plural('egg', ['a', 'b', 'c'])", "eggs"),
        ("plural_noun('egg', ('a', 'b', 'c'))", "eggs"),
        ("plural_adj('a', ('a', 'b', 'c'))", "some"),
        ("plural_verb('was', ('a', 'b', 'c'))", "were"),
        ("singular_noun('eggs', ('a', 'b', 'c'))", "eggs"),
        ("an('error', ('a', 'b', 'c'))", "('a', 'b', 'c') error"),
        (
            "This is not a function(name)",
            "This is not a function(name)",
        ),
    ]);
}

#[test]
fn inflect_on_builtin_constants() {
    check_inflect(&[
        (
            "Plural of False is plural('False')",
            "Plural of False is Falses",
        ),
        ("num(10, False) plural('False')", " Falses"),
        ("plural('True')", "Trues"),
        ("num(10, True) plural('False')", "10 Falses"),
        ("plural('None')", "Nones"),
        ("num(10, None) plural('True')", "10 Trues"),
    ]);
}

#[test]
fn inflect_keyword_args() {
    check_inflect(&[
        (
            "number_to_words(1234, andword='')",
            "one thousand, two hundred thirty-four",
        ),
        (
            "number_to_words(1234, andword='plus')",
            "one thousand, two hundred plus thirty-four",
        ),
        (
            "number_to_words('555_1202', group=1, zero='oh')",
            "five, five, five, one, two, oh, two",
        ),
    ]);
}

#[test]
fn name_error_in_strings() {
    let mut e = Engine::new();
    assert_eq!(e.inflect("plural('two')").unwrap(), "twoes");
    assert!(e.inflect("plural(two)").is_err());
}
