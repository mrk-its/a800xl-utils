fn main() {
    cc::Build::new()
        .compiler("mos-a800xl-clang")
        .file("src/cio.S")
        .compile("cio");
}
