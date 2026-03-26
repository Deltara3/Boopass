fn main() {
    println!("cargo::rerun-if-changed=src/imgui_bridge.cc");

    cc::Build::new()
        .cpp(true)
        .includes([
            "src/imgui",
            "src/imgui/backends"
        ])
        .files([
            "src/imgui_bridge.cc",
            "src/imgui/imgui.cpp",
            "src/imgui/imgui_widgets.cpp",
            "src/imgui/imgui_tables.cpp",
            "src/imgui/imgui_draw.cpp",
            "src/imgui/backends/imgui_impl_dx10.cpp",
            "src/imgui/backends/imgui_impl_win32.cpp"
        ])
        .compile("ui");
}