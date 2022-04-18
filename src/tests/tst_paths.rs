#[test]
fn path_parts_test() {
    use crate::rux_paths;
    let tester = rux_paths::path_parts("/home/pointel/test");
    assert_eq!(tester.len(), 4);
    assert_eq!(tester[0], "/");
    assert_eq!(tester[1], "home");
    assert_eq!(tester[2], "pointel");
    assert_eq!(tester[3], "test");
    let tester = rux_paths::path_parts("pointel/test");
    assert_eq!(tester.len(), 2);
    assert_eq!(tester[0], "pointel");
    assert_eq!(tester[1], "test");
    let tester = rux_paths::path_parts("./pointel/test");
    assert_eq!(tester.len(), 3);
    assert_eq!(tester[0], ".");
    assert_eq!(tester[1], "pointel");
    assert_eq!(tester[2], "test");
    let tester = rux_paths::path_parts("C:\\pointel\\test");
    assert_eq!(tester.len(), 3);
    assert_eq!(tester[0], "C:");
    assert_eq!(tester[1], "pointel");
    assert_eq!(tester[2], "test");
}

#[test]
fn path_parts_join_test() {
    use crate::rux_paths;
    let tester = rux_paths::path_parts("/home/pointel/test");
    let expect = "/home/pointel/test";
    let result = rux_paths::path_parts_join(tester.as_slice());
    assert_eq!(result, expect);
    let tester = rux_paths::path_parts("C:\\pointel\\test");
    let expect = "C:\\pointel\\test";
    let result = rux_paths::path_parts_join(tester.as_slice());
    assert_eq!(result, expect);
    let tester = rux_paths::path_parts("pointel/test");
    let expect = format!("pointel{}test", rux_paths::os_sep());
    let result = rux_paths::path_parts_join(tester.as_slice());
    assert_eq!(result, expect);
    let tester = rux_paths::path_parts("./pointel/test");
    let expect = format!(".{}pointel{}test", rux_paths::os_sep(), rux_paths::os_sep());
    let result = rux_paths::path_parts_join(tester.as_slice());
    assert_eq!(result, expect);
    let tester = rux_paths::path_parts("../../pointel/test");
    let expect = format!(
        "..{}..{}pointel{}test",
        rux_paths::os_sep(),
        rux_paths::os_sep(),
        rux_paths::os_sep()
    );
    let result = rux_paths::path_parts_join(tester.as_slice());
    assert_eq!(result, expect);
}

#[test]
fn path_absolute_test() {
    use crate::rux_paths;
    let wd = rux_paths::wd().unwrap();
    let tester = "test";
    let expect = format!("{}{}test", wd, rux_paths::os_sep());
    let result = rux_paths::path_absolute(tester).unwrap();
    assert_eq!(result, expect);
    let tester = "./test";
    let expect = format!("{}{}test", wd, rux_paths::os_sep());
    let result = rux_paths::path_absolute(tester).unwrap();
    assert_eq!(result, expect);
}