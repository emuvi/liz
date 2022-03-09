#[test]
fn rig_group_equals_any_test() {
    use crate::liz_forms;
    use crate::liz_group;
    let mut tester = liz_forms::kit_from(&["ab", "cd", "$", "ef", "$"]);
    let result = liz_forms::kit_from(&["ab", "cd", "$ef", "$"]);
    liz_group::rig_group_all(
        &mut tester,
        vec![liz_group::group_pair(
            liz_group::group_equals("$".into()),
            liz_group::group_any(),
        )],
        false
    )
    .expect("Could not parse.");
    assert_eq!(tester, result);
}
