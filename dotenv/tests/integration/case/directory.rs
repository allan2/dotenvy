use crate::util::*;
use dotenvy_test_util::*;

#[test]
fn child_dir() {
    let mut testenv = TestEnv::init_with_envfile(KEYVAL_1);
    testenv.add_child_dir("child");
    testenv.set_work_dir("child");
    test_key_1_only(&testenv);
}

#[test]
fn child_dir_no_envfile() {
    let mut testenv = TestEnv::init();
    testenv.add_child_dir("child");
    testenv.set_work_dir("child");
    test_err_not_found(&testenv);
}

#[test]
fn parent_dir_not_found() {
    let mut testenv = TestEnv::init();
    testenv.add_child_dir("child");
    testenv.add_envfile("child/.env", KEYVAL_1);
    test_err_not_found(&testenv);
}

#[test]
fn sibling_dir_not_found() {
    let mut testenv = TestEnv::init();
    testenv.add_child_dir("brother");
    testenv.add_child_dir("sister");
    testenv.add_envfile("brother/.env", KEYVAL_1);
    testenv.set_work_dir("sister");
    test_err_not_found(&testenv);
}

#[test]
fn grandchild_dir() {
    let mut testenv = TestEnv::init_with_envfile(KEYVAL_1);
    testenv.add_child_dir("child/grandchild");
    testenv.set_work_dir("child/grandchild");
    test_key_1_only(&testenv);
}
