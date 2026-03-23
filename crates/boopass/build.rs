fn main() {
    println!("cargo::rerun-if-changed=src/gfxhook.cc");

    cc::Build::new()
        .cpp(true)
        .file("src/gfxhook.cc")
        .compile("ui");
}