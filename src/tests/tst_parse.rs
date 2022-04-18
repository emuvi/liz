#[cfg(test)]
mod rig_parse_test {
    use crate::liz_forms;
    use crate::liz_parse;

    #[test]
    fn rig_parse_white_space_test() {
        let mut tester = liz_forms::kit_from(&["ab  cd"]);
        let result = liz_forms::kit_from(&["ab", "  ", "cd"]);
        liz_parse::rig_parse_all(&mut tester, vec![liz_parse::block_white_space()])
            .expect("Could not parse.");
        assert_eq!(tester, result);
    }

    #[test]
    fn rig_parse_punctuation_test() {
        let mut tester = liz_forms::kit_from(&["ab!?cd"]);
        let result = liz_forms::kit_from(&["ab", "!", "?", "cd"]);
        liz_parse::rig_parse_all(&mut tester, vec![liz_parse::block_punctuation()])
            .expect("Could not parse.");
        assert_eq!(tester, result);
    }

    #[test]
    fn rig_parse_single_quotes_test() {
        let mut tester = liz_forms::kit_from(&["ab cd 'abc  \\' de' fg '\\\\'abc"]);
        let result = liz_forms::kit_from(&["ab cd ", "'abc  \\' de'", " fg ", "'\\\\'", "abc"]);
        liz_parse::rig_parse_all(&mut tester, vec![liz_parse::block_single_quotes()])
            .expect("Could not parse.");
        assert_eq!(tester, result);
    }

    #[test]
    fn rig_parse_regex_test() {
        let mut tester = liz_forms::kit_from(&["ab cd abde ab"]);
        let result = liz_forms::kit_from(&["ab", " cd ", "ab", "de ", "ab"]);
        liz_parse::rig_parse_all(&mut tester, vec![liz_parse::block_regex("ab".into())])
            .expect("Could not parse.");
        assert_eq!(tester, result);
    }

    #[test]
    fn rig_parse_char_number_test() {
        let mut tester = liz_forms::kit_from(&["ab ' $ a'  $cd, $1 $21    $345 \" $4   \" end"]);
        let result = liz_forms::kit_from(&[
            "ab",
            " ",
            "' $ a'",
            "  ",
            "$",
            "cd",
            ",",
            " ",
            "$1",
            " ",
            "$21",
            "    ",
            "$345",
            " ",
            "\" $4   \"",
            " ",
            "end",
        ]);
        liz_parse::rig_parse_all(
            &mut tester,
            vec![
                liz_parse::block_double_quotes(),
                liz_parse::block_single_quotes(),
                liz_parse::block_white_space(),
                liz_parse::block_char_number('$'),
                liz_parse::block_punctuation(),
            ],
        )
        .expect("Could not parse.");
        assert_eq!(tester, result);
    }
}
