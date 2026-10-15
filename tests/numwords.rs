use inflect_rs::*;

fn with_threshold(threshold: i64) -> NumberToWordsOptions {
    NumberToWordsOptions {
        threshold: Some(threshold),
        ..Default::default()
    }
}

fn with_group(group: u8) -> NumberToWordsOptions {
    NumberToWordsOptions {
        group,
        ..Default::default()
    }
}

#[test]
fn loop_thresholds() {
    let e = Engine::new();
    for thresh in 0..=20 {
        for n in 0..=20 {
            let threshed = e.number_to_words_with(n, &with_threshold(thresh)).unwrap();
            let numwords = e.number_to_words(n).unwrap();
            if n <= thresh {
                assert_eq!(numwords, threshed);
            } else {
                assert_eq!(threshed, n.to_string());
            }
        }
    }
}

#[test]
fn lines() {
    let e = Engine::new();
    let opts = with_threshold(500);
    let no_comma = NumberToWordsOptions {
        comma: String::new(),
        ..with_threshold(500)
    };

    for (n, with_comma, without_comma) in [
        (999, "999", "999"),
        (1000, "1,000", "1000"),
        (10000, "10,000", "10000"),
        (100000, "100,000", "100000"),
        (1000000, "1,000,000", "1000000"),
    ] {
        assert_eq!(e.number_to_words_with(n, &opts).unwrap(), with_comma);
        assert_eq!(e.number_to_words_with(n, &no_comma).unwrap(), without_comma);
    }

    for (n, with_comma, without_comma) in [
        (999.3, "999.3", "999.3"),
        (1000.3, "1,000.3", "1000.3"),
        (10000.3, "10,000.3", "10000.3"),
        (100000.3, "100,000.3", "100000.3"),
        (1000000.3, "1,000,000.3", "1000000.3"),
    ] {
        assert_eq!(e.number_to_words_with(n, &opts).unwrap(), with_comma);
        assert_eq!(e.number_to_words_with(n, &no_comma).unwrap(), without_comma);
    }
}

const NW: &[&[&str]] = &[
    &["0", "zero", "zero", "zero", "zero", "zeroth"],
    &["1", "one", "one", "one", "one", "first"],
    &["2", "two", "two", "two", "two", "second"],
    &["3", "three", "three", "three", "three", "third"],
    &["4", "four", "four", "four", "four", "fourth"],
    &["5", "five", "five", "five", "five", "fifth"],
    &["6", "six", "six", "six", "six", "sixth"],
    &["7", "seven", "seven", "seven", "seven", "seventh"],
    &["8", "eight", "eight", "eight", "eight", "eighth"],
    &["9", "nine", "nine", "nine", "nine", "ninth"],
    &["10", "ten", "one, zero", "ten", "ten", "tenth"],
    &["11", "eleven", "one, one", "eleven", "eleven", "eleventh"],
    &["12", "twelve", "one, two", "twelve", "twelve", "twelfth"],
    &[
        "13",
        "thirteen",
        "one, three",
        "thirteen",
        "thirteen",
        "thirteenth",
    ],
    &[
        "14",
        "fourteen",
        "one, four",
        "fourteen",
        "fourteen",
        "fourteenth",
    ],
    &[
        "15",
        "fifteen",
        "one, five",
        "fifteen",
        "fifteen",
        "fifteenth",
    ],
    &[
        "16",
        "sixteen",
        "one, six",
        "sixteen",
        "sixteen",
        "sixteenth",
    ],
    &[
        "17",
        "seventeen",
        "one, seven",
        "seventeen",
        "seventeen",
        "seventeenth",
    ],
    &[
        "18",
        "eighteen",
        "one, eight",
        "eighteen",
        "eighteen",
        "eighteenth",
    ],
    &[
        "19",
        "nineteen",
        "one, nine",
        "nineteen",
        "nineteen",
        "nineteenth",
    ],
    &["20", "twenty", "two, zero", "twenty", "twenty", "twentieth"],
    &[
        "21",
        "twenty-one",
        "two, one",
        "twenty-one",
        "twenty-one",
        "twenty-first",
    ],
    &[
        "29",
        "twenty-nine",
        "two, nine",
        "twenty-nine",
        "twenty-nine",
        "twenty-ninth",
    ],
    &[
        "99",
        "ninety-nine",
        "nine, nine",
        "ninety-nine",
        "ninety-nine",
        "ninety-ninth",
    ],
    &[
        "100",
        "one hundred",
        "one, zero, zero",
        "ten, zero",
        "one zero zero",
        "one hundredth",
    ],
    &[
        "101",
        "one hundred and one",
        "one, zero, one",
        "ten, one",
        "one zero one",
        "one hundred and first",
    ],
    &[
        "110",
        "one hundred and ten",
        "one, one, zero",
        "eleven, zero",
        "one ten",
        "one hundred and tenth",
    ],
    &[
        "111",
        "one hundred and eleven",
        "one, one, one",
        "eleven, one",
        "one eleven",
        "one hundred and eleventh",
    ],
    &[
        "900",
        "nine hundred",
        "nine, zero, zero",
        "ninety, zero",
        "nine zero zero",
        "nine hundredth",
    ],
    &[
        "999",
        "nine hundred and ninety-nine",
        "nine, nine, nine",
        "ninety-nine, nine",
        "nine ninety-nine",
        "nine hundred and ninety-ninth",
    ],
    &[
        "1000",
        "one thousand",
        "one, zero, zero, zero",
        "ten, zero zero",
        "one zero zero, zero",
        "one thousandth",
    ],
    &[
        "1001",
        "one thousand and one",
        "one, zero, zero, one",
        "ten, zero one",
        "one zero zero, one",
        "one thousand and first",
    ],
    &[
        "1010",
        "one thousand and ten",
        "one, zero, one, zero",
        "ten, ten",
        "one zero one, zero",
        "one thousand and tenth",
    ],
    &[
        "1100",
        "one thousand, one hundred",
        "one, one, zero, zero",
        "eleven, zero zero",
        "one ten, zero",
        "one thousand, one hundredth",
    ],
    &[
        "2000",
        "two thousand",
        "two, zero, zero, zero",
        "twenty, zero zero",
        "two zero zero, zero",
        "two thousandth",
    ],
    &[
        "10000",
        "ten thousand",
        "one, zero, zero, zero, zero",
        "ten, zero zero, zero",
        "one zero zero, zero zero",
        "ten thousandth",
    ],
    &[
        "100000",
        "one hundred thousand",
        "one, zero, zero, zero, zero, zero",
        "ten, zero zero, zero zero",
        "one zero zero, zero zero zero",
        "one hundred thousandth",
    ],
    &[
        "100001",
        "one hundred thousand and one",
        "one, zero, zero, zero, zero, one",
        "ten, zero zero, zero one",
        "one zero zero, zero zero one",
        "one hundred thousand and first",
    ],
    &[
        "123456",
        "one hundred and twenty-three thousand, four hundred and fifty-six",
        "one, two, three, four, five, six",
        "twelve, thirty-four, fifty-six",
        "one twenty-three, four fifty-six",
        "one hundred and twenty-three thousand, four hundred and fifty-sixth",
    ],
    &[
        "0123456",
        "one hundred and twenty-three thousand, four hundred and fifty-six",
        "zero, one, two, three, four, five, six",
        "zero one, twenty-three, forty-five, six",
        "zero twelve, three forty-five, six",
        "one hundred and twenty-three thousand, four hundred and fifty-sixth",
    ],
    &[
        "1234567",
        "one million, two hundred and thirty-four thousand, five hundred and sixty-seven",
        "one, two, three, four, five, six, seven",
        "twelve, thirty-four, fifty-six, seven",
        "one twenty-three, four fifty-six, seven",
        "one million, two hundred and thirty-four thousand, five hundred and sixty-seventh",
    ],
    &[
        "12345678",
        "twelve million, three hundred and forty-five thousand, six hundred and seventy-eight",
        "one, two, three, four, five, six, seven, eight",
        "twelve, thirty-four, fifty-six, seventy-eight",
        "one twenty-three, four fifty-six, seventy-eight",
        "twelve million, three hundred and forty-five thousand, six hundred and seventy-eighth",
    ],
    &[
        "12_345_678",
        "twelve million, three hundred and forty-five thousand, six hundred and seventy-eight",
        "one, two, three, four, five, six, seven, eight",
        "twelve, thirty-four, fifty-six, seventy-eight",
        "one twenty-three, four fifty-six, seventy-eight",
    ],
    &[
        "1234,5678",
        "twelve million, three hundred and forty-five thousand, six hundred and seventy-eight",
        "one, two, three, four, five, six, seven, eight",
        "twelve, thirty-four, fifty-six, seventy-eight",
        "one twenty-three, four fifty-six, seventy-eight",
    ],
    &[
        "1234567890",
        "one billion, two hundred and thirty-four million, five hundred \
         and sixty-seven thousand, eight hundred and ninety",
        "one, two, three, four, five, six, seven, eight, nine, zero",
        "twelve, thirty-four, fifty-six, seventy-eight, ninety",
        "one twenty-three, four fifty-six, seven eighty-nine, zero",
        "one billion, two hundred and thirty-four million, five hundred \
         and sixty-seven thousand, eight hundred and ninetieth",
    ],
    &[
        "123456789012345",
        "one hundred and twenty-three trillion, four hundred and \
         fifty-six billion, seven hundred and eighty-nine million, twelve \
         thousand, three hundred and forty-five",
        "one, two, three, four, five, six, seven, eight, nine, zero, one, \
         two, three, four, five",
        "twelve, thirty-four, fifty-six, seventy-eight, ninety, twelve, \
         thirty-four, five",
        "one twenty-three, four fifty-six, seven eighty-nine, \
         zero twelve, three forty-five",
        "one hundred and twenty-three trillion, four hundred and \
         fifty-six billion, seven hundred and eighty-nine million, \
         twelve thousand, three hundred and forty-fifth",
    ],
    &[
        "12345678901234567890",
        "twelve quintillion, three hundred and forty-five quadrillion, \
         six hundred and seventy-eight trillion, nine hundred and one \
         billion, two hundred and thirty-four million, five hundred and \
         sixty-seven thousand, eight hundred and ninety",
        "one, two, three, four, five, six, seven, eight, nine, zero, one, \
         two, three, four, five, six, seven, eight, nine, zero",
        "twelve, thirty-four, fifty-six, seventy-eight, ninety, twelve, \
         thirty-four, fifty-six, seventy-eight, ninety",
        "one twenty-three, four fifty-six, seven eighty-nine, \
         zero twelve, three forty-five, six seventy-eight, ninety",
        "twelve quintillion, three hundred and forty-five quadrillion, \
         six hundred and seventy-eight trillion, nine hundred and one \
         billion, two hundred and thirty-four million, five hundred and \
         sixty-seven thousand, eight hundred and ninetieth",
    ],
    &[
        "0.987654",
        "zero point nine eight seven six five four",
        "zero, point, nine, eight, seven, six, five, four",
        "zero, point, ninety-eight, seventy-six, fifty-four",
        "zero, point, nine eighty-seven, six fifty-four",
        "zeroth point nine eight seven six five four",
        "zero point nine eight seven six five fourth",
    ],
    &[
        ".987654",
        "point nine eight seven six five four",
        "point, nine, eight, seven, six, five, four",
        "point, ninety-eight, seventy-six, fifty-four",
        "point, nine eighty-seven, six fifty-four",
        "point nine eight seven six five four",
        "point nine eight seven six five fourth",
    ],
    &[
        "9.87654",
        "nine point eight seven six five four",
        "nine, point, eight, seven, six, five, four",
        "nine, point, eighty-seven, sixty-five, four",
        "nine, point, eight seventy-six, fifty-four",
        "ninth point eight seven six five four",
        "nine point eight seven six five fourth",
    ],
    &[
        "98.7654",
        "ninety-eight point seven six five four",
        "nine, eight, point, seven, six, five, four",
        "ninety-eight, point, seventy-six, fifty-four",
        "ninety-eight, point, seven sixty-five, four",
        "ninety-eighth point seven six five four",
        "ninety-eight point seven six five fourth",
    ],
    &[
        "987.654",
        "nine hundred and eighty-seven point six five four",
        "nine, eight, seven, point, six, five, four",
        "ninety-eight, seven, point, sixty-five, four",
        "nine eighty-seven, point, six fifty-four",
        "nine hundred and eighty-seventh point six five four",
        "nine hundred and eighty-seven point six five fourth",
    ],
    &[
        "9876.54",
        "nine thousand, eight hundred and seventy-six point five four",
        "nine, eight, seven, six, point, five, four",
        "ninety-eight, seventy-six, point, fifty-four",
        "nine eighty-seven, six, point, fifty-four",
        "nine thousand, eight hundred and seventy-sixth point five four",
        "nine thousand, eight hundred and seventy-six point five fourth",
    ],
    &[
        "98765.4",
        "ninety-eight thousand, seven hundred and sixty-five point four",
        "nine, eight, seven, six, five, point, four",
        "ninety-eight, seventy-six, five, point, four",
        "nine eighty-seven, sixty-five, point, four",
        "ninety-eight thousand, seven hundred and sixty-fifth point four",
        "ninety-eight thousand, seven hundred and sixty-five point fourth",
    ],
    &[
        "101.202.303",
        "one hundred and one point two zero two three zero three",
        "one, zero, one, point, two, zero, two, point, three, zero, three",
        "ten, one, point, twenty, two, point, thirty, three",
        "one zero one, point, two zero two, point, three zero three",
    ],
    &[
        "98765.",
        "ninety-eight thousand, seven hundred and sixty-five point",
        "nine, eight, seven, six, five, point",
        "ninety-eight, seventy-six, five, point",
        "nine eighty-seven, sixty-five, point",
    ],
];

#[test]
fn array() {
    let e = Engine::new();
    for row in NW {
        let n = row[0];
        assert_eq!(e.number_to_words(n).unwrap(), row[1], "{n}");
        for group in 1..=3u8 {
            assert_eq!(
                e.number_to_words_with(n, &with_group(group)).unwrap(),
                row[group as usize + 1],
                "{n} group={group}"
            );
        }
        if let Some(&ordinal_words) = row.get(5) {
            assert_eq!(
                e.number_to_words(e.ordinal(n)).unwrap(),
                ordinal_words,
                "number_to_words(ordinal({n}))"
            );
            let words_ordinal = row.get(6).copied().unwrap_or(ordinal_words);
            assert_eq!(
                e.ordinal(e.number_to_words(n).unwrap()),
                words_ordinal,
                "ordinal(number_to_words({n}))"
            );
        }
    }
}

#[test]
fn issue_131() {
    let e = Engine::new();
    for suffix in ["th", "st", "nd", "rd"] {
        assert_eq!(e.number_to_words(suffix).unwrap(), "zero");
    }
}
