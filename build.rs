use glob::glob;
use std::path::{Path, PathBuf};
extern crate cc;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=parser");
    let mut cfg = cc::Build::new();
    cfg.include("parser").include("parser/include");
    let c_files = glob("./parser/*.c")
        .unwrap()
        .filter(|path| path.is_ok())
        .map(|path| path.unwrap());

    cfg.files(c_files);
    cfg.compile("libpg_query");
    Ok(())
}
