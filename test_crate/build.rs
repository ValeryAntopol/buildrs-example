use std::{fs, path, env};

fn main() {
    let content = "\
    pub mod test_mod {
        pub struct TestStruct3 {
            pub test_field: i32,
        }
        impl TestStruct3 {
            pub fn new() -> Self {Self{test_field: 0,}}
            pub fn test_method() {}
            pub fn test_method_self(&self) -> &Self {&self}
        }
        pub fn test_func() {}
    }
    ";

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let path = path::Path::new(&out_dir).join("test_mod.rs");
    fs::write(&path, content).unwrap();
}
