fn main() {
    glib_build_tools::compile_resources(
        &["data/ui", "data/styles", "data/images", "data/icons/scalable/actions"],
        "data/resources.gresource.xml",
        "prophesy.gresource",
    );
}
