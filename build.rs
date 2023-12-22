fn main() {
    // Build the C components
    cc::Build::new().file("src/c/aes.c").compile("aes");
    cc::Build::new().file("src/c/glitch.c").compile("glitch");
}
