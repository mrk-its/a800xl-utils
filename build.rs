#[cfg(feature = "docs-rs")]
fn main() {} // Skip the script when the doc is building

#[cfg(not(feature = "docs-rs"))]
fn main() {
    cc::Build::new()
        .compiler("mos-a800xl-clang")
        .file("src/cio.S")
        .compile("cio");
}
