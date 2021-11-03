include!(concat!(env!("OUT_DIR"), "/test_mod.rs"));
fn main() {
    let x: test_mod::TestStruct3 = test_mod::TestStruct3::new();
    x.test_method_self();
    x.test_field;
}
