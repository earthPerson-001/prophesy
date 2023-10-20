fn main() {
    glib_build_tools::compile_resources(
        &["data/resources", "data/ui", "data/icons", "data/styles", "data/images", "data/icons/scalable/actions"],
        "data/resources.gresource.xml",
        "prophesy.gresource",
    );
}
