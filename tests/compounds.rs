use inflect_rs::*;

fn check_plurals(e: &Engine, cases: &[(&str, &str)]) {
    for (singular, plural) in cases {
        assert_eq!(e.plural(singular), *plural, "plural({singular:?})");
    }
}

#[test]
fn compound_singulars_unchanged() {
    let e = Engine::new();
    for word in [
        "hello-out-there",
        "hello out there",
        "continue-to-operate",
        "case of diapers",
    ] {
        assert_eq!(e.singular_noun(word).as_deref(), Some(word));
    }
}

#[test]
fn unit_handling_degree() {
    check_plurals(
        &Engine::new(),
        &[
            ("degree celsius", "degrees celsius"),
            ("degree fahrenheit", "degrees fahrenheit"),
            ("degree rankine", "degrees rankine"),
            ("degree fahrenheit second", "degree fahrenheit seconds"),
        ],
    );
}

#[test]
fn unit_handling_fractional() {
    check_plurals(
        &Engine::new(),
        &[
            ("pound per square inch", "pounds per square inch"),
            ("metre per second", "metres per second"),
            ("kilometre per hour", "kilometres per hour"),
            ("cubic metre per second", "cubic metres per second"),
            ("dollar a year", "dollars a year"),
            ("foot per square second", "feet per square second"),
            ("mother-in-law per lifetime", "mothers-in-law per lifetime"),
            (
                "pound-force per square inch",
                "pounds-force per square inch",
            ),
        ],
    );
}

#[test]
fn unit_handling_combined() {
    check_plurals(
        &Engine::new(),
        &[
            (
                "watt per square meter degree celsius",
                "watts per square meter degree celsius",
            ),
            ("degree celsius per hour", "degrees celsius per hour"),
            (
                "degree fahrenheit hour square foot per btuit inch",
                "degree fahrenheit hour square feet per btuit inch",
            ),
        ],
    );
}

#[test]
fn unit_open_compound_nouns() {
    check_plurals(
        &Engine::new(),
        &[
            ("high school", "high schools"),
            ("master genie", "master genies"),
            ("MASTER genie", "MASTER genies"),
            ("Blood brother", "Blood brothers"),
            ("prima donna", "prima donnas"),
            ("prima DONNA", "prima DONNAS"),
        ],
    );
}

#[test]
fn unit_open_compound_nouns_classical() {
    let mut e = Engine::new();
    e.classical = Classical::all();
    check_plurals(
        &e,
        &[
            ("master genie", "master genii"),
            ("MASTER genie", "MASTER genii"),
            ("Blood brother", "Blood brethren"),
            ("prima donna", "prime donne"),
            ("prima DONNA", "prime DONNE"),
        ],
    );
}
