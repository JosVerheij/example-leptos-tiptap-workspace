use std::io::Write;

// Note: used to build tiptap
pub fn main() {
    let root: std::path::PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    // Note: get rust workspace dir in stead of current crate dir
    // See: https://github.com/rust-lang/cargo/issues/3946
    let root = root.join("..");
    let public_dir = root.join("public");
    let js_dir = public_dir.join("js");

    std::fs::create_dir_all(js_dir.clone()).unwrap();
    println!("cargo:warning=js dir created");

    std::fs::File::create(js_dir.join("tiptap-bundle.min.js"))
        .unwrap()
        .write(leptos_tiptap_build::TIPTAP_BUNDLE_MIN_JS.as_bytes())
        .unwrap();
    println!("cargo:warning=tiptap-bundle.min.js written");

    std::fs::File::create(js_dir.join("tiptap.js"))
        .unwrap()
        .write(leptos_tiptap_build::TIPTAP_JS.as_bytes())
        .unwrap();
    println!("cargo:warning=tiptap.js written");
}
