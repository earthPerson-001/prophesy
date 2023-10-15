fn main() {
    glib_build_tools::compile_resources(
        &["data/resources", "data/ui", "data/assets/icons"],
        "data/resources/resources.gresource.xml",
        "prophesy.gresource",
    );
}