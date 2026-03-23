fn main() {
    println!("cargo::rerun-if-changed=src/gfxhook.cc");
    println!("cargo::rerun-if-changed=src/ui.cc");

    cc::Build::new()
        .cpp(true)
        .file("src/gfxhook.cc")
        .file("src/ui.cc")
        .compile("ui");
}