#[test]
fn rig_parse_white_space_test() {
    use crate::liz_forms;
    use crate::liz_parse;
    let mut tester = liz_forms::kit_from(&["ab  cd"]);
    let result = liz_forms::kit_from(&["ab", "  ", "cd"]);
    liz_parse::rig_parse_all(&mut tester, vec![liz_parse::block_white_space()])
        .expect("Could not parse.");
    assert_eq!(tester, result);
}

#[test]
fn rig_parse_punctuation_test() {
    use crate::liz_forms;
    use crate::liz_parse;
    let mut tester = liz_forms::kit_from(&["ab!?cd"]);
    let result = liz_forms::kit_from(&["ab", "!", "?", "cd"]);
    liz_parse::rig_parse_all(&mut tester, vec![liz_parse::block_punctuation()])
        .expect("Could not parse.");
    assert_eq!(tester, result);
}

#[test]
fn rig_parse_single_quotes_test() {
    use crate::liz_forms;
    use crate::liz_parse;
    let mut tester = liz_forms::kit_from(&["ab cd 'abc  \\' de' fg '\\\\'abc"]);
    let result = liz_forms::kit_from(&["ab cd ", "'abc  \\' de'", " fg ", "'\\\\'", "abc"]);
    liz_parse::rig_parse_all(&mut tester, vec![liz_parse::block_single_quotes()])
        .expect("Could not parse.");
    assert_eq!(tester, result);
}
