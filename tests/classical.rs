use inflect_rs::*;

fn check(e: &Engine, cases: &[(&str, Option<i64>, &str)]) {
    for &(word, count, expected) in cases {
        let got = match count {
            Some(n) => e.plural_noun_with_count(word, n),
            None => e.plural_noun(word),
        };
        assert_eq!(
            got, expected,
            "plural_noun({word:?}, {count:?}) with {:?}",
            e.classical
        );
    }
}

#[test]
fn classical_all() {
    let mut e = Engine::new();
    let modern = [
        ("error", Some(0), "errors"),
        ("wildebeest", None, "wildebeests"),
        ("brother", None, "brothers"),
        ("person", None, "people"),
        ("formula", None, "formulas"),
    ];
    let classical = [
        ("error", Some(0), "error"),
        ("wildebeest", None, "wildebeest"),
        ("Sally", None, "Sallys"),
        ("brother", None, "brethren"),
        ("person", None, "persons"),
        ("formula", None, "formulae"),
    ];

    check(&e, &modern);
    check(&e, &[("Sally", None, "Sallys")]);

    e.classical = Classical::all();
    check(&e, &classical);

    e.classical = Classical::none();
    check(&e, &modern);
    check(&e, &[("Sally", None, "Sallies")]);

    e.classical = Classical::all();
    check(&e, &classical);
}

#[test]
fn classical_ancient() {
    let mut e = Engine::new();
    check(&e, &[("formula", None, "formulas")]);
    e.classical.ancient = true;
    check(
        &e,
        &[
            ("formula", None, "formulae"),
            ("wildebeest", None, "wildebeests"),
            ("error", Some(0), "errors"),
            ("Sally", None, "Sallys"),
            ("brother", None, "brothers"),
            ("person", None, "people"),
        ],
    );
}

#[test]
fn classical_herd() {
    let mut e = Engine::new();
    check(&e, &[("wildebeest", None, "wildebeests")]);
    e.classical.herd = true;
    check(
        &e,
        &[
            ("wildebeest", None, "wildebeest"),
            ("formula", None, "formulas"),
            ("error", Some(0), "errors"),
            ("Sally", None, "Sallys"),
            ("brother", None, "brothers"),
            ("person", None, "people"),
        ],
    );
}

#[test]
fn classical_names() {
    let mut e = Engine::new();
    check(
        &e,
        &[("Sally", None, "Sallys"), ("Jones", Some(0), "Joneses")],
    );
    e.classical.names = true;
    check(
        &e,
        &[
            ("Sally", None, "Sallys"),
            ("Jones", Some(0), "Joneses"),
            ("wildebeest", None, "wildebeests"),
            ("formula", None, "formulas"),
            ("error", Some(0), "errors"),
            ("brother", None, "brothers"),
            ("person", None, "people"),
        ],
    );
}

#[test]
fn classical_persons() {
    let mut e = Engine::new();
    check(&e, &[("person", None, "people")]);
    e.classical.persons = true;
    check(
        &e,
        &[
            ("person", None, "persons"),
            ("wildebeest", None, "wildebeests"),
            ("formula", None, "formulas"),
            ("error", Some(0), "errors"),
            ("brother", None, "brothers"),
            ("Sally", None, "Sallys"),
            ("Jones", Some(0), "Joneses"),
        ],
    );
}

#[test]
fn classical_zero() {
    let mut e = Engine::new();
    check(&e, &[("error", Some(0), "errors")]);
    e.classical.zero = true;
    check(
        &e,
        &[
            ("error", Some(0), "error"),
            ("wildebeest", None, "wildebeests"),
            ("formula", None, "formulas"),
            ("person", None, "people"),
            ("brother", None, "brothers"),
            ("Sally", None, "Sallys"),
        ],
    );
}
