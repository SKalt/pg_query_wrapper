fn main() {
    extern crate cc;
    println!("cargo:rerun-if-changed=parser");
    cc::Build::new()
        .include("parser")
        .include("parser/include")
        .file("parser/pg_query.c")
        .compile("libpg_query");
}
