fn main() {
    println!("cargo::rerun-if-changed=src/gfxhook.cc");
    println!("cargo::rerun-if-changed=src/ui.cc");

    cc::Build::new()
        .cpp(true)
        .includes([
            "src/imgui",
            "src/imgui/backends"
        ])
        .files([
            "src/gfxhook.cc",
            "src/ui.cc",
            "src/imgui/imgui.cpp",
            "src/imgui/imgui_widgets.cpp",
            "src/imgui/imgui_tables.cpp",
            "src/imgui/imgui_draw.cpp",
            "src/imgui/imgui_demo.cpp",
            "src/imgui/backends/imgui_impl_dx10.cpp",
            "src/imgui/backends/imgui_impl_win32.cpp"
        ])
        .compile("ui");
}