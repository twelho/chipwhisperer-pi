fn main() {
    cc::Build::new()
        .file("src/c/aes.c")
        .compile("aes");
}
