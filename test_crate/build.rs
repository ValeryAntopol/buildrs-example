use std::{fs, path, env};

fn main() {
    let content = "\
    pub mod test_mod {
        pub fn test_fun() {}
    }
    ";

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let path = path::Path::new(&out_dir).join("test_mod.rs");
    fs::write(&path, content).unwrap();
}
