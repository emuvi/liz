#[test]
fn rig_parse_whitespace_test() {
    use crate::liz_forms;
    use crate::liz_parse;
    crate::liz_debug::put_dbg_verbose_tells();
    let mut tester = liz_forms::kit_from(&["ab  cd"]);
    let result = liz_forms::kit_from(&["ab", "  ", "cd"]);
    liz_parse::rig_parse_all(&mut tester, vec![liz_parse::rig_whitespace()]);
    assert_eq!(tester, result);
}

#[test]
fn rig_parse_punctuation_test() {
    use crate::liz_forms;
    use crate::liz_parse;
    crate::liz_debug::put_dbg_verbose_tells();
    let mut tester = liz_forms::kit_from(&["ab!?cd"]);
    let result = liz_forms::kit_from(&["ab", "!", "?", "cd"]);
    liz_parse::rig_parse_all(&mut tester, vec![liz_parse::rig_punctuation()]);
    assert_eq!(tester, result);
}
