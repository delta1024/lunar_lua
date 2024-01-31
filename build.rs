use std::{env, path::PathBuf};

fn  main() {
    println!("cargo:rustc-link-search=/usr/lib64/lua");

    println!("cargo:rustc-link-lib=lua");
    
    let lua= bindgen::Builder::default()
    .header("includes/lua.h")
    .generate()
    .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    lua.write_to_file(out_path.join("src/raw_lua.rs")).expect("Couldn't write bindings");
    for path in &["includes/lua.h", "build.rs"] {
        println!("cargo:rerun-if-changed={}", path);
    }
}
