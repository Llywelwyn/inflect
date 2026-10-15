use inflect_rs::*;

#[test]
fn unicode_plural() {
    let e = Engine::new();
    for (singular, plural) in [("cliché", "clichés"), ("ångström", "ångströms")] {
        assert_eq!(e.plural(singular), plural);
    }
}
