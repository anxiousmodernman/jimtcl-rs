extern crate jimtcl_rs;

use jimtcl_rs::*;

#[test]
fn import_tests() {
    let i = jimtcl_rs::Interpreter::new();
    let code = "
    set a 1
    set b 2
    ";
    i.eval(code);
    assert_eq!(Some(JimVal::Str("1".to_string())), i.get_val("a"));
}
