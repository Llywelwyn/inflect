use inflect_rs::*;

fn cmp(s: &str) -> Option<Comparison> {
    match s {
        "eq" => Some(Comparison::Eq),
        "p:s" => Some(Comparison::PluralSingular),
        "s:p" => Some(Comparison::SingularPlural),
        "p:p" => Some(Comparison::PluralPlural),
        "" => None,
        _ => unreachable!("{s}"),
    }
}

fn words_with(e: &Engine, n: impl std::fmt::Display, options: NumberToWordsOptions) -> String {
    e.number_to_words_with(n, &options).unwrap()
}

#[test]
fn classical() {
    let mut e = Engine::new();
    assert_eq!(e.classical, Classical::default());
    assert_eq!(
        Classical::default(),
        Classical {
            all: false,
            zero: false,
            herd: false,
            names: true,
            persons: false,
            ancient: false,
        }
    );
    assert_eq!(
        Classical::all(),
        Classical {
            all: true,
            zero: true,
            herd: true,
            names: true,
            persons: true,
            ancient: true,
        }
    );
    assert_eq!(
        Classical::none(),
        Classical {
            all: false,
            zero: false,
            herd: false,
            names: false,
            persons: false,
            ancient: false,
        }
    );

    e.classical = Classical::none();
    e.classical.names = true;
    e.classical.zero = true;
    assert_eq!(
        e.classical,
        Classical {
            zero: true,
            ..Classical::default()
        }
    );

    e.classical = Classical::none();
    e.classical.names = true;
    e.classical.zero = false;
    assert_eq!(e.classical, Classical::default());
}

#[test]
fn num() {
    let mut e = Engine::new();
    assert_eq!(e.persistent_count(), None);

    assert_eq!(e.num(None), "");
    assert_eq!(e.persistent_count(), None);

    assert_eq!(e.num(Some(3)), "3");
    assert_eq!(e.persistent_count(), Some(3));

    e.num(None);
    assert_eq!(e.persistent_count(), None);
}

#[test]
fn inflect() {
    let mut e = Engine::new();
    for (txt, ans) in [
        ("num(1)", "1"),
        ("num(1,0)", ""),
        ("num(1,1)", "1"),
        ("num(1)   ", "1   "),
        ("   num(1)   ", "   1   "),
        ("num(3) num(1)", "3 1"),
    ] {
        assert_eq!(e.inflect(txt).unwrap(), ans, "inflect({txt:?})");
    }

    for (txt, ans) in [
        ("plural('rock')", "rocks"),
        ("plural('rock')  plural('child')", "rocks  children"),
        (
            "num(2) plural('rock')  plural('child')",
            "2 rocks  children",
        ),
        (
            "plural('rock') plural_noun('rock') plural_verb('rocks') plural_adj('big') a('ant')",
            "rocks rocks rock big an ant",
        ),
        (
            "an('rock') no('cat') ordinal(3) number_to_words(1234) present_participle('runs')",
            "a rock no cats 3rd one thousand, two hundred and thirty-four running",
        ),
        (
            "a('cat',0) a('cat',1) a('cat',2) a('cat', 2)",
            "0 cat a cat 2 cat 2 cat",
        ),
    ] {
        assert_eq!(e.inflect(txt).unwrap(), ans, "inflect({txt:?})");
    }
}

#[test]
fn user_input_fns() {
    let mut e = Engine::new();

    e.defnoun("VAX", "VAXen").unwrap();
    assert_eq!(e.plural("VAX"), "VAXEN");

    e.defnoun("cow", "cows|kine").unwrap();
    assert_eq!(e.plural("cow"), "cows");
    e.classical = Classical::all();
    assert_eq!(e.plural("cow"), "kine");

    e.defnoun("(.+i)o", "$1i").unwrap();
    assert_eq!(e.plural("studio"), "studii");

    e.defnoun("aviatrix", "aviatrices").unwrap();
    assert_eq!(e.plural("aviatrix"), "aviatrices");
    e.defnoun("aviatrix", "aviatrixes").unwrap();
    assert_eq!(e.plural("aviatrix"), "aviatrixes");

    e.defnoun("(cat)", "$1s").unwrap();
    assert_eq!(e.plural("cat"), "cats");

    assert!(matches!(
        e.defnoun("(??", "x"),
        Err(Error::BadUserDefinedPattern(_))
    ));

    e.defadj("hir", "their").unwrap();
    assert_eq!(e.plural("hir"), "their");

    e.defa("h").unwrap();
    assert_eq!(e.a("h"), "a h");

    e.defan("horrendous.*").unwrap();
    assert_eq!(e.a("horrendously"), "an horrendously");
}

#[test]
fn user_input_defverb() {
    let mut e = Engine::new();
    e.defverb("will", "shall", "will", "will", "will", "will")
        .unwrap();
    assert_eq!(e.plural("will"), "will");
}

#[test]
fn postprocess_and_partition() {
    let e = Engine::new();
    for (word, plural) in [
        ("cow", "cows"),
        ("I", "we"),
        ("COW", "COWS"),
        ("Cow", "Cows"),
        ("Entry", "Entrys"),
        ("can of Coke", "cans of Coke"),
        (" cow ", " cows "),
        ("   cow", "   cows"),
        ("cow   ", "cows   "),
        ("  cow   ", "  cows   "),
        ("bottle of beer", "bottles of beer"),
    ] {
        assert_eq!(e.plural(word), plural, "plural({word:?})");
    }
}

#[test]
fn pl() {
    let mut e = Engine::new();
    for (sing, plur) in [
        ("cow", "cows"),
        ("thought", "thoughts"),
        ("mouse", "mice"),
        ("knife", "knives"),
        ("knifes", "knife"),
        (" cat  ", " cats  "),
        ("court martial", "courts martial"),
        ("a", "some"),
        ("carmen", "carmina"),
        ("quartz", "quartzes"),
        ("care", "cares"),
    ] {
        assert_eq!(e.plural(sing), plur, "plural({sing:?})");
    }
    for (sing, plur) in [
        ("cow", "cows"),
        ("thought", "thoughts"),
        ("snooze", "snoozes"),
    ] {
        assert_eq!(e.plural_noun(sing), plur, "plural_noun({sing:?})");
    }
    for (sing, plur) in [("runs", "run"), ("thought", "thought"), ("eyes", "eye")] {
        assert_eq!(e.plural_verb(sing), plur, "plural_verb({sing:?})");
    }
    for (sing, plur) in [
        ("a", "some"),
        ("this", "these"),
        ("that", "those"),
        ("my", "our"),
        ("cat's", "cats'"),
        ("child's", "children's"),
    ] {
        assert_eq!(e.plural_adj(sing), plur, "plural_adj({sing:?})");
    }

    for (sing, num, plur) in [
        ("cow", Count::from(1), "cow"),
        ("cow", Count::from(2), "cows"),
        ("cow", Count::from("one"), "cow"),
        ("cow", Count::from("each"), "cow"),
        ("cow", Count::from("two"), "cows"),
        ("cow", Count::from(0), "cows"),
        ("cow", Count::from("zero"), "cows"),
        ("runs", Count::from(0), "run"),
        ("runs", Count::from(1), "runs"),
        ("am", Count::from(0), "are"),
    ] {
        assert_eq!(e.plural_with_count(sing, num), plur, "plural({sing:?})");
    }

    e.classical.zero = true;
    assert_eq!(e.plural_with_count("cow", 0), "cow");
    assert_eq!(e.plural_with_count("cow", "zero"), "cow");
    assert_eq!(e.plural_with_count("runs", 0), "runs");
    assert_eq!(e.plural_with_count("am", 0), "am");
    assert_eq!(e.plural_verb_with_count("runs", 1), "runs");

    assert_eq!(e.plural("die"), "dice");
    assert_eq!(e.plural_noun("die"), "dice");
}

#[test]
fn sinoun() {
    let mut e = Engine::new();
    for (sing, plur) in [("cat", "cats"), ("die", "dice"), ("goose", "geese")] {
        assert_eq!(e.singular_noun(plur).as_deref(), Some(sing));
        assert_eq!(
            e.inflect(&format!("singular_noun('{plur}')")).unwrap(),
            sing
        );
    }

    assert_eq!(
        e.singular_noun_with_count("cats", 2).as_deref(),
        Some("cats")
    );
    assert_eq!(
        e.singular_noun_with_count("open valves", 2).as_deref(),
        Some("open valves")
    );

    for (plur, sing) in [
        ("zombies", "zombie"),
        ("shoes", "shoe"),
        ("dancing shoes", "dancing shoe"),
        ("Matisses", "Matisse"),
        ("bouillabaisses", "bouillabaisse"),
        ("quartzes", "quartz"),
        ("Nietzsches", "Nietzsche"),
        ("aches", "ache"),
        ("Clives", "Clive"),
        ("weaves", "weave"),
    ] {
        assert_eq!(
            e.singular_noun(plur).as_deref(),
            Some(sing),
            "singular_noun({plur:?})"
        );
    }

    assert_eq!(e.singular_noun("status"), None);
    assert_eq!(e.singular_noun("hiatus"), None);
}

#[test]
fn gender() {
    let mut e = Engine::new();
    for (gender, cases) in [
        (
            Gender::Feminine,
            [
                ("she", "they"),
                ("herself", "themselves"),
                ("hers", "theirs"),
                ("to her", "to them"),
                ("to herself", "to themselves"),
            ],
        ),
        (
            Gender::Masculine,
            [
                ("he", "they"),
                ("himself", "themselves"),
                ("his", "theirs"),
                ("to him", "to them"),
                ("to himself", "to themselves"),
            ],
        ),
        (
            Gender::GenderNeutral,
            [
                ("they", "they"),
                ("themself", "themselves"),
                ("theirs", "theirs"),
                ("to them", "to them"),
                ("to themself", "to themselves"),
            ],
        ),
        (
            Gender::Neuter,
            [
                ("it", "they"),
                ("itself", "themselves"),
                ("its", "theirs"),
                ("to it", "to them"),
                ("to itself", "to themselves"),
            ],
        ),
    ] {
        e.set_gender(gender);
        assert_eq!(e.gender(), gender);
        for (sing, plur) in cases {
            assert_eq!(
                e.singular_noun(plur).as_deref(),
                Some(sing),
                "singular_noun({plur:?}) with {gender:?}"
            );
            assert_eq!(
                e.inflect(&format!("singular_noun('{plur}')")).unwrap(),
                sing
            );
        }
    }

    assert!("male".parse::<Gender>().is_err());

    for (sing, plur, gen) in [
        ("it", "they", "neuter"),
        ("she", "they", "feminine"),
        ("he", "they", "masculine"),
        ("they", "they", "gender-neutral"),
        ("she or he", "they", "feminine or masculine"),
        ("he or she", "they", "masculine or feminine"),
    ] {
        let gender: Gender = gen.parse().unwrap();
        assert_eq!(gender.to_string(), gen);
        assert_eq!(
            e.singular_noun_with_gender(plur, gender).as_deref(),
            Some(sing)
        );
    }

    assert_eq!(
        "unknown gender".parse::<Gender>(),
        Err(Error::BadGender("unknown gender".to_string()))
    );
}

fn check_compare(f: fn(&Engine, &str, &str) -> Option<Comparison>, cases: &[(&str, &str, &str)]) {
    for (a, b, res) in cases {
        assert_eq!(f(&Engine::new(), a, b), cmp(res), "compare({a:?}, {b:?})");
    }
}

#[test]
fn compare_simple() {
    check_compare(
        Engine::compare,
        &[
            ("index", "index", "eq"),
            ("index", "indexes", "s:p"),
            ("index", "indices", "s:p"),
            ("indexes", "index", "p:s"),
            ("indices", "index", "p:s"),
            ("indices", "indexes", "p:p"),
            ("indexes", "indices", "p:p"),
            ("indices", "indices", "eq"),
            ("inverted index", "inverted indices", "s:p"),
            ("inverted indices", "inverted index", "p:s"),
            ("inverted indexes", "inverted indices", "p:p"),
            ("opuses", "opera", "p:p"),
            ("opera", "opuses", "p:p"),
            ("brothers", "brethren", "p:p"),
            ("cats", "cats", "eq"),
            ("base", "basis", ""),
            ("syrinx", "syringe", ""),
            ("she", "he", ""),
            ("opus", "operas", ""),
            ("taxi", "taxes", ""),
            ("time", "Times", ""),
            ("time", "times", "s:p"),
            ("courts martial", "court martial", "p:s"),
            ("my", "my", "eq"),
            ("my", "our", "s:p"),
            ("our", "our", "eq"),
        ],
    );
}

#[test]
fn compare_nouns() {
    check_compare(
        Engine::compare_nouns,
        &[
            ("index", "index", "eq"),
            ("index", "indexes", "s:p"),
            ("index", "indices", "s:p"),
            ("indexes", "index", "p:s"),
            ("indices", "index", "p:s"),
            ("indices", "indexes", "p:p"),
            ("indexes", "indices", "p:p"),
            ("indices", "indices", "eq"),
            ("inverted index", "inverted indices", "s:p"),
            ("inverted indices", "inverted index", "p:s"),
            ("inverted indexes", "inverted indices", "p:p"),
            ("stigmata", "stigmas", "p:p"),
            ("phalanxes", "phalanges", "p:p"),
        ],
    );
}

#[test]
fn compare_verbs() {
    check_compare(
        Engine::compare_verbs,
        &[
            ("runs", "runs", "eq"),
            ("runs", "run", "s:p"),
            ("run", "run", "eq"),
        ],
    );
}

#[test]
fn compare_adjectives() {
    check_compare(
        Engine::compare_adjs,
        &[
            ("my", "my", "eq"),
            ("my", "our", "s:p"),
            ("our", "our", "eq"),
            ("indexes's", "indices's", "p:p"),
            ("indices's", "indexes's", "p:p"),
            ("indexes'", "indices's", "p:p"),
            ("indexes's", "indices'", "p:p"),
            ("indexes's", "indexes's", "eq"),
            ("dogmas's", "dogmata's", "p:p"),
            ("dogmas'", "dogmata'", "p:p"),
            ("indexes'", "indices'", "p:p"),
        ],
    );
}

#[test]
fn comparison_as_str() {
    for s in ["eq", "p:s", "s:p", "p:p"] {
        assert_eq!(cmp(s).unwrap().as_str(), s);
    }
}

#[test]
fn count() {
    let mut e = Engine::new();
    for (count, plural) in [
        (Count::from(1), "cat"),
        (Count::from(2), "cats"),
        (Count::from(0), "cats"),
        (Count::from(87), "cats"),
        (Count::from(-7), "cats"),
        (Count::from("1"), "cat"),
        (Count::from("2"), "cats"),
        (Count::from("0"), "cats"),
        (Count::from("no"), "cats"),
        (Count::from("zero"), "cats"),
        (Count::from("nil"), "cats"),
        (Count::from("a"), "cat"),
        (Count::from("an"), "cat"),
        (Count::from("one"), "cat"),
        (Count::from("each"), "cat"),
        (Count::from("every"), "cat"),
        (Count::from("this"), "cat"),
        (Count::from("that"), "cat"),
        (Count::from("dummy"), "cats"),
    ] {
        assert_eq!(e.plural_with_count("cat", count), plural);
    }

    e.num(Some(1));
    assert_eq!(e.plural("cat"), "cat");
    e.num(Some(3));
    assert_eq!(e.plural("cat"), "cats");
}

#[test]
fn plnoun() {
    let mut e = Engine::new();
    for (sing, plur) in [
        ("tuna", "tuna"),
        ("TUNA", "TUNA"),
        ("swordfish", "swordfish"),
        ("Governor General", "Governors General"),
        ("Governor-General", "Governors-General"),
        ("Major General", "Major Generals"),
        ("Major-General", "Major-Generals"),
        ("mother in law", "mothers in law"),
        ("mother-in-law", "mothers-in-law"),
        ("about me", "about us"),
        ("to it", "to them"),
        ("from it", "from them"),
        ("with it", "with them"),
        ("I", "we"),
        ("you", "you"),
        ("me", "us"),
        ("mine", "ours"),
        ("child", "children"),
        ("brainchild", "brainchilds"),
        ("human", "humans"),
        ("soliloquy", "soliloquies"),
        ("chairwoman", "chairwomen"),
        ("goose", "geese"),
        ("tooth", "teeth"),
        ("foot", "feet"),
        ("forceps", "forceps"),
        ("protozoon", "protozoa"),
        ("czech", "czechs"),
        ("codex", "codices"),
        ("radix", "radices"),
        ("bacterium", "bacteria"),
        ("alumnus", "alumni"),
        ("criterion", "criteria"),
        ("alumna", "alumnae"),
        ("bias", "biases"),
        ("quiz", "quizzes"),
        ("fox", "foxes"),
        ("shelf", "shelves"),
        ("leaf", "leaves"),
        ("midwife", "midwives"),
        ("scarf", "scarves"),
        ("key", "keys"),
        ("Sally", "Sallys"),
        ("sally", "sallies"),
        ("ado", "ados"),
        ("auto", "autos"),
        ("alto", "altos"),
        ("zoo", "zoos"),
        ("tomato", "tomatoes"),
    ] {
        assert_eq!(e.plural_noun(sing), plur, "plural_noun({sing:?})");
        assert_eq!(
            e.singular_noun(plur).as_deref(),
            Some(sing),
            "singular_noun({plur:?})"
        );
    }

    for (sing, plur) in [
        ("son of a gun", "sons of guns"),
        ("son-of-a-gun", "sons-of-guns"),
        ("basis", "bases"),
        ("Jess", "Jesses"),
    ] {
        assert_eq!(e.plural_noun(sing), plur, "plural_noun({sing:?})");
    }

    e.num(Some(1));
    assert_eq!(e.plural_noun("cat"), "cat");
    e.num(Some(3));

    e.classical.herd = true;
    assert_eq!(e.plural_noun("swine"), "swine");
    e.classical.herd = false;
    assert_eq!(e.plural_noun("swine"), "swines");
    e.classical.persons = true;
    assert_eq!(e.plural_noun("chairperson"), "chairpersons");
    e.classical.persons = false;
    assert_eq!(e.plural_noun("chairperson"), "chairpeople");
    e.classical.ancient = true;
    assert_eq!(e.plural_noun("formula"), "formulae");
    e.classical.ancient = false;
    assert_eq!(e.plural_noun("formula"), "formulas");

    e.classical = Classical::all();
    for (sing, plur) in [
        ("matrix", "matrices"),
        ("gateau", "gateaux"),
        ("millieu", "millieux"),
        ("syrinx", "syringes"),
        ("stamen", "stamina"),
        ("apex", "apices"),
        ("appendix", "appendices"),
        ("maximum", "maxima"),
        ("focus", "foci"),
        ("status", "status"),
        ("aurora", "aurorae"),
        ("soma", "somata"),
        ("iris", "irides"),
        ("solo", "soli"),
        ("oxymoron", "oxymora"),
        ("goy", "goyim"),
        ("afrit", "afriti"),
    ] {
        assert_eq!(e.plural_noun(sing), plur, "plural_noun({sing:?})");
    }
}

#[test]
fn classical_pl() {
    let mut e = Engine::new();
    e.classical = Classical::all();
    for (sing, plur) in [("brother", "brethren"), ("dogma", "dogmata")] {
        assert_eq!(e.plural(sing), plur);
    }
}

#[test]
fn pl_special_verb() {
    let mut e = Engine::new();
    assert_eq!(e.plural_verb("am"), "are");
    assert_eq!(e.plural_verb_with_count("am", 0), "are");
    assert_eq!(e.plural_verb_with_count("runs", 0), "run");

    e.classical.zero = true;
    assert_eq!(e.plural_verb_with_count("am", 0), "am");
    assert_eq!(e.plural_verb_with_count("am", 1), "am");
    assert_eq!(e.plural_verb_with_count("am", 2), "are");
    assert_eq!(e.plural_verb_with_count("runs", 0), "runs");
    for (verb, plural) in [
        ("am going to", "are going to"),
        ("did", "did"),
        ("wasn't", "weren't"),
        ("shouldn't", "shouldn't"),
        ("bias", "bias"),
        ("news", "news"),
        ("Jess", "Jess"),
        ("brushes", "brush"),
        ("fixes", "fix"),
        ("quizzes", "quiz"),
        ("fizzes", "fizz"),
        ("dresses", "dress"),
        ("flies", "fly"),
        ("canoes", "canoe"),
        ("horseshoes", "horseshoe"),
        ("does", "do"),
        ("zzzoes", "zzzo"),
        ("runs", "run"),
    ] {
        assert_eq!(e.plural_verb(verb), plural, "plural_verb({verb:?})");
    }
}

#[test]
fn pl_general_verb() {
    let e = Engine::new();
    assert_eq!(e.plural_verb("acts"), "act");
    assert_eq!(e.plural_verb("act"), "act");
    assert_eq!(e.plural_verb("saw"), "saw");
    assert_eq!(e.plural_verb_with_count("runs", 1), "runs");
}

#[test]
fn pl_special_adjective() {
    let e = Engine::new();
    for (adj, plur) in [
        ("a", "some"),
        ("my", "our"),
        ("John's", "Johns'"),
        ("tuna's", "tuna's"),
        ("TUNA's", "TUNA's"),
        ("bad", "bad"),
        ("'", "'"),
    ] {
        assert_eq!(e.plural_adj(adj), plur, "plural_adj({adj:?})");
    }
}

#[test]
fn a() {
    let e = Engine::new();
    for (sing, plur) in [
        ("cat", "a cat"),
        ("euphemism", "a euphemism"),
        ("Euler number", "an Euler number"),
        ("hour", "an hour"),
        ("houri", "a houri"),
        ("nth", "an nth"),
        ("rth", "an rth"),
        ("sth", "an sth"),
        ("xth", "an xth"),
        ("ant", "an ant"),
        ("book", "a book"),
        ("RSPCA", "an RSPCA"),
        ("SONAR", "a SONAR"),
        ("FJO", "a FJO"),
        ("FJ", "an FJ"),
        ("NASA", "a NASA"),
        ("UN", "a UN"),
        ("yak", "a yak"),
        ("yttrium", "an yttrium"),
        ("a elephant", "an elephant"),
        ("a giraffe", "a giraffe"),
        ("an ewe", "a ewe"),
        ("a orangutan", "an orangutan"),
        ("R.I.P.", "an R.I.P."),
        ("C.O.D.", "a C.O.D."),
        ("e-mail", "an e-mail"),
        ("X-ray", "an X-ray"),
        ("T-square", "a T-square"),
        ("LCD", "an LCD"),
        ("XML", "an XML"),
        ("YWCA", "a YWCA"),
        ("LED", "a LED"),
        ("OPEC", "an OPEC"),
        ("FAQ", "a FAQ"),
        ("UNESCO", "a UNESCO"),
        ("a", "an a"),
        ("an", "an an"),
        ("an ant", "an ant"),
        ("a cat", "a cat"),
        ("an cat", "a cat"),
        ("a ant", "an ant"),
    ] {
        assert_eq!(e.a(sing), plur, "a({sing:?})");
        assert_eq!(e.an(sing), plur, "an({sing:?})");
    }
}

#[test]
fn a_alt() {
    let e = Engine::new();
    assert_eq!(e.a_with_count("cat", 1), "a cat");
    assert_eq!(e.a_with_count("cat", 2), "2 cat");
}

#[test]
fn no() {
    let mut e = Engine::new();
    assert_eq!(e.no("cat"), "no cats");
    assert_eq!(e.no_with_count("cat", 3), "3 cats");
    assert_eq!(e.no_with_count("cat", "three"), "three cats");
    assert_eq!(e.no_with_count("cat", 1), "1 cat");
    assert_eq!(e.no_with_count("cat", "one"), "one cat");
    assert_eq!(e.no("mouse"), "no mice");
    e.num(Some(3));
    assert_eq!(e.no("cat"), "3 cats");
}

#[test]
fn prespart() {
    let e = Engine::new();
    for (sing, plur) in [
        ("runs", "running"),
        ("dies", "dying"),
        ("glues", "gluing"),
        ("eyes", "eying"),
        ("skis", "skiing"),
        ("names", "naming"),
        ("sees", "seeing"),
        ("hammers", "hammering"),
        ("bats", "batting"),
        ("eats", "eating"),
        ("loves", "loving"),
        ("spies", "spying"),
        ("hoes", "hoeing"),
        ("alibis", "alibiing"),
        ("is", "being"),
        ("are", "being"),
        ("had", "having"),
        ("has", "having"),
    ] {
        assert_eq!(
            e.present_participle(sing),
            plur,
            "present_participle({sing:?})"
        );
    }
}

#[test]
fn ordinal() {
    let e = Engine::new();
    for (num, ord) in [
        ("1", "1st"),
        ("2", "2nd"),
        ("3", "3rd"),
        ("4", "4th"),
        ("10", "10th"),
        ("28", "28th"),
        ("100", "100th"),
        ("101", "101st"),
        ("1000", "1000th"),
        ("1001", "1001st"),
        ("0", "0th"),
        ("one", "first"),
        ("two", "second"),
        ("four", "fourth"),
        ("twenty", "twentieth"),
        ("one hundered", "one hunderedth"),
        ("one hundered and one", "one hundered and first"),
        ("zero", "zeroth"),
        ("n", "nth"),
    ] {
        assert_eq!(e.ordinal(num), ord, "ordinal({num:?})");
    }
}

#[test]
fn numwords_out_of_range() {
    let e = Engine::new();
    assert_eq!(
        e.number_to_words(format!("1{}", "0".repeat(33))).unwrap(),
        "one decillion"
    );
    assert_eq!(
        e.number_to_words(format!("1{}", "0".repeat(36))),
        Err(Error::NumOutOfRange)
    );
}

#[test]
fn numwords() {
    let e = Engine::new();

    for (n, word) in [
        ("1", "one"),
        ("10", "ten"),
        ("100", "one hundred"),
        ("1000", "one thousand"),
        ("10000", "ten thousand"),
        ("100000", "one hundred thousand"),
        ("1000000", "one million"),
        ("10000000", "ten million"),
        ("+10", "plus ten"),
        ("-10", "minus ten"),
        ("10.", "ten point"),
        (".10", "point one zero"),
        ("1.23", "one point two three"),
    ] {
        assert_eq!(
            e.number_to_words(n).unwrap(),
            word,
            "number_to_words({n:?})"
        );
    }

    for (n, txt) in [
        (3, "three bottles of beer on the wall"),
        (2, "two bottles of beer on the wall"),
        (1, "a solitary bottle of beer on the wall"),
        (0, "no more bottles of beer on the wall"),
    ] {
        let opts = NumberToWordsOptions {
            one: "a solitary".into(),
            zero: "no more".into(),
            ..Default::default()
        };
        assert_eq!(
            format!(
                "{}{}",
                words_with(&e, n, opts),
                e.plural_with_count(" bottle of beer on the wall", n)
            ),
            txt
        );
    }

    let zero_one = NumberToWordsOptions {
        one: "one".into(),
        zero: "zero".into(),
        ..Default::default()
    };
    assert_eq!(words_with(&e, 0, zero_one), "zero");

    let defaults = NumberToWordsOptions::default();
    assert_eq!(
        e.number_to_words("1234").unwrap(),
        "one thousand, two hundred and thirty-four"
    );
    assert_eq!(
        e.number_to_words_list("1234", &defaults).unwrap(),
        ["one thousand", "two hundred and thirty-four"]
    );
    assert_eq!(
        e.number_to_words_list("1234567", &defaults).unwrap(),
        [
            "one million",
            "two hundred and thirty-four thousand",
            "five hundred and sixty-seven",
        ]
    );
    assert_eq!(
        e.number_to_words_list("+10", &defaults).unwrap(),
        ["plus", "ten"]
    );

    let andword = |w: &str| NumberToWordsOptions {
        andword: w.into(),
        ..Default::default()
    };
    assert_eq!(
        words_with(&e, "1234", andword("")),
        "one thousand, two hundred thirty-four"
    );
    assert_eq!(
        words_with(&e, "1234", andword("plus")),
        "one thousand, two hundred plus thirty-four"
    );

    assert_eq!(e.number_to_words(e.ordinal("21")).unwrap(), "twenty-first");

    let threshold = |t: i64| NumberToWordsOptions {
        threshold: Some(t),
        ..Default::default()
    };
    for (n, word) in [
        ("9", "nine"),
        ("10", "ten"),
        ("11", "11"),
        ("1000", "1,000"),
        ("123", "123"),
        ("1234", "1,234"),
        ("1234.5678", "1,234.5678"),
    ] {
        assert_eq!(words_with(&e, n, threshold(10)), word, "{n} threshold=10");
    }

    let no_decimal = || NumberToWordsOptions {
        decimal: None,
        ..Default::default()
    };
    assert_eq!(words_with(&e, "1", no_decimal()), "one");
    assert_eq!(
        words_with(&e, "1234.5678", no_decimal()),
        "twelve million, three hundred and forty-five thousand, six hundred and seventy-eight"
    );
}

#[test]
fn numwords_group_chunking_error() {
    let e = Engine::new();
    let opts = NumberToWordsOptions {
        group: 4,
        ..Default::default()
    };
    assert_eq!(
        e.number_to_words_with("1234", &opts),
        Err(Error::BadChunkingOption)
    );
}

#[test]
fn numwords_group() {
    let e = Engine::new();
    let group = |g: u8| NumberToWordsOptions {
        group: g,
        ..Default::default()
    };
    for (input, opts, expect) in [
        ("12345", group(2), "twelve, thirty-four, five"),
        ("123456", group(3), "one twenty-three, four fifty-six"),
        ("12345", group(1), "one, two, three, four, five"),
        (
            "1234th",
            NumberToWordsOptions {
                andword: "and".into(),
                ..group(0)
            },
            "one thousand, two hundred and thirty-fourth",
        ),
        (
            "1234th",
            group(0),
            "one thousand, two hundred and thirty-fourth",
        ),
        ("120", group(2), "twelve, zero"),
        (
            "120",
            NumberToWordsOptions {
                zero: "oh".into(),
                one: "unity".into(),
                ..group(2)
            },
            "twelve, oh",
        ),
        (
            "555_1202",
            NumberToWordsOptions {
                zero: "oh".into(),
                ..group(1)
            },
            "five, five, five, one, two, oh, two",
        ),
        (
            "555_1202",
            NumberToWordsOptions {
                one: "unity".into(),
                ..group(1)
            },
            "five, five, five, unity, two, zero, two",
        ),
        (
            "123.456",
            NumberToWordsOptions {
                decimal: Some("mark".into()),
                one: "one".into(),
                ..group(1)
            },
            "one, two, three, mark, four, five, six",
        ),
    ] {
        assert_eq!(
            words_with(&e, input, opts.clone()),
            expect,
            "{input} {opts:?}"
        );
    }
}

#[test]
fn wordlist() {
    let e = Engine::new();
    let conj = |c: &str| JoinOptions {
        conj: c.into(),
        ..Default::default()
    };
    let conj_unspaced = |c: &str| JoinOptions {
        conj_spaced: false,
        ..conj(c)
    };

    assert_eq!(e.join(&[]), "");
    assert_eq!(e.join(&["apple"]), "apple");
    assert_eq!(e.join(&["apple", "banana"]), "apple and banana");
    assert_eq!(
        e.join(&["apple", "banana", "carrot"]),
        "apple, banana, and carrot"
    );
    assert_eq!(
        e.join(&["apple", "1,000", "carrot"]),
        "apple; 1,000; and carrot"
    );

    let three = ["apple", "banana", "carrot"];
    let two = ["apple", "banana"];
    for (words, opts, expected) in [
        (
            &["apple", "1,000", "carrot"][..],
            JoinOptions {
                sep: Some(",".into()),
                ..Default::default()
            },
            "apple, 1,000, and carrot",
        ),
        (
            &three[..],
            JoinOptions {
                final_sep: Some("".into()),
                ..Default::default()
            },
            "apple, banana and carrot",
        ),
        (
            &three[..],
            JoinOptions {
                final_sep: Some(";".into()),
                ..Default::default()
            },
            "apple, banana; and carrot",
        ),
        (&three[..], conj("or"), "apple, banana, or carrot"),
        (&two[..], conj(" or "), "apple  or  banana"),
        (&two[..], conj("&"), "apple & banana"),
        (&two[..], conj_unspaced("&"), "apple&banana"),
        (&two[..], conj_unspaced("& "), "apple& banana"),
        (&three[..], conj(" or "), "apple, banana,  or  carrot"),
        (&three[..], conj("+"), "apple, banana, + carrot"),
        (&three[..], conj("&"), "apple, banana, & carrot"),
        (&three[..], conj_unspaced("&"), "apple, banana,&carrot"),
        (&three[..], conj_unspaced(" &"), "apple, banana, &carrot"),
    ] {
        assert_eq!(e.join_with(words, &opts), expected, "{words:?} {opts:?}");
    }
}

#[test]
fn doc_examples() {
    let mut e = Engine::new();
    assert_eq!(e.plural_noun("I"), "we");
    assert_eq!(e.plural_verb("saw"), "saw");
    assert_eq!(e.plural_adj("my"), "our");
    assert_eq!(e.plural_noun("saw"), "saws");
    assert_eq!(e.plural("was"), "were");
    assert_eq!(e.plural_with_count("was", 1), "was");
    assert_eq!(e.plural_verb_with_count("was", 2), "were");
    assert_eq!(e.plural_verb("was"), "were");
    assert_eq!(e.plural_verb_with_count("was", 1), "was");

    for (errors, txt) in [
        (0, "There were no errors"),
        (1, "There was 1 error"),
        (2, "There were 2 errors"),
    ] {
        assert_eq!(
            format!(
                "There {}{}",
                e.plural_verb_with_count("was", errors),
                e.no_with_count(" error", errors)
            ),
            txt
        );
        assert_eq!(
            e.inflect(&format!(
                "There plural_verb('was',{errors}) no('error',{errors})"
            ))
            .unwrap(),
            txt
        );
    }

    for (num1, num2, txt) in [(1, 2, "I saw 2 saws"), (2, 1, "we saw 1 saw")] {
        e.num(Some(num1));
        let pronoun = e.plural("I");
        let verb = e.plural_verb(" saw");
        let shown = e.num(Some(num2));
        let noun = e.plural_noun(" saw");
        assert_eq!(format!("{pronoun}{verb} {shown}{noun}"), txt);

        assert_eq!(
            e.inflect(&format!(
                "num({num1}, False)plural('I') plural_verb('saw') num({num2}) plural_noun('saw')"
            ))
            .unwrap(),
            txt
        );
    }

    assert_eq!(e.a("a cat"), "a cat");

    let one_as_a = NumberToWordsOptions {
        one: "a".into(),
        ..Default::default()
    };
    for (word, txt) in [
        ("cat", "a cat"),
        ("aardvark", "an aardvark"),
        ("ewe", "a ewe"),
        ("hour", "an hour"),
    ] {
        let n = e.number_to_words_with(1, &one_as_a).unwrap();
        assert_eq!(e.a(&format!("{n} {word}")), txt);
    }
}
