use inflect_rs::*;

fn check(cases: &[(&str, &str)]) {
    let e = Engine::new();
    for (word, expected) in cases {
        assert_eq!(e.an(word), *expected, "an({word:?})");
    }
}

#[test]
fn an() {
    check(&[
        ("cat", "a cat"),
        ("ant", "an ant"),
        ("a", "an a"),
        ("b", "a b"),
        ("honest cat", "an honest cat"),
        ("dishonest cat", "a dishonest cat"),
        ("Honolulu sunset", "a Honolulu sunset"),
        ("mpeg", "an mpeg"),
        ("onetime holiday", "a onetime holiday"),
        ("Ugandan person", "a Ugandan person"),
        ("Ukrainian person", "a Ukrainian person"),
        ("Unabomber", "a Unabomber"),
        ("unanimous decision", "a unanimous decision"),
        ("US farmer", "a US farmer"),
        ("wild PIKACHU appeared", "a wild PIKACHU appeared"),
    ]);
}

#[test]
fn an_abbreviation() {
    check(&[
        ("YAML code block", "a YAML code block"),
        ("Core ML function", "a Core ML function"),
        ("JSON code block", "a JSON code block"),
    ]);
}
