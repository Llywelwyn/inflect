use inflect_rs::*;

#[test]
fn pl_si() {
    for classical in [Classical::none(), Classical::all()] {
        for word in ["Times", "Jones"] {
            let mut e = Engine::new();
            e.classical = classical;
            let plural = e.plural_noun_with_count(word, 2);
            assert_eq!(
                e.singular_noun_with_count(&plural, 1).as_deref(),
                Some(word),
                "{word} with {classical:?}"
            );
        }
    }
}
