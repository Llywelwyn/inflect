use inflect_rs::*;

fn opts(final_sep: Option<&str>, conj: Option<&str>) -> JoinOptions {
    JoinOptions {
        final_sep: final_sep.map(String::from),
        conj: conj.unwrap_or("and").to_string(),
        ..Default::default()
    }
}

fn check(words: &[&str], expected: [&str; 5]) {
    let e = Engine::new();
    let variants = [
        opts(None, None),
        opts(Some(""), None),
        opts(Some("..."), None),
        opts(Some("..."), Some("")),
        opts(None, Some("or")),
    ];
    for (o, want) in variants.iter().zip(expected) {
        assert_eq!(e.join_with(words, o), want, "{words:?} with {o:?}");
    }
}

#[test]
fn join() {
    let e = Engine::new();
    assert_eq!(
        e.join(&["apple", "banana", "carrot"]),
        "apple, banana, and carrot"
    );

    check(
        &["apple", "banana", "carrot"],
        [
            "apple, banana, and carrot",
            "apple, banana and carrot",
            "apple, banana... and carrot",
            "apple, banana... carrot",
            "apple, banana, or carrot",
        ],
    );

    check(
        &["apple,fuji", "banana", "carrot"],
        [
            "apple,fuji; banana; and carrot",
            "apple,fuji; banana and carrot",
            "apple,fuji; banana... and carrot",
            "apple,fuji; banana... carrot",
            "apple,fuji; banana; or carrot",
        ],
    );

    check(
        &["apple", "carrot"],
        [
            "apple and carrot",
            "apple and carrot",
            "apple and carrot",
            "apple carrot",
            "apple or carrot",
        ],
    );
    let mut o = opts(Some("..."), Some(""));
    o.conj_spaced = false;
    assert_eq!(e.join_with(&["apple", "carrot"], &o), "applecarrot");

    check(&["carrot"], ["carrot"; 5]);
}
