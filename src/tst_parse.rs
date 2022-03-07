#[test]
fn rig_split_whitespace_test() {
    use crate::liz_forms;
    use crate::liz_parse;
    crate::liz_debug::put_dbg_verbose_tells();
    let mut tester = liz_forms::kit_new(&["ab  cd"]);
    let result = liz_forms::kit_new(&["ab", "  ", "cd"]);
    liz_parse::rig_split_whitespace(&mut tester);
    assert_eq!(tester, result);
}

#[test]
fn rig_group_whitespace_test() {
    use crate::liz_forms;
    use crate::liz_parse;
    crate::liz_debug::put_dbg_verbose_tells();
    let mut tester = liz_forms::kit_new(&["ab", "  ", "cd"]);
    let result = liz_forms::kit_new(&["ab  cd"]);
    liz_parse::rig_group_whitespace(&mut tester);
    assert_eq!(tester, result);
}