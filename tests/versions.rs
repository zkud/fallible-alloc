#[test]
fn readme_version_is_correct() {
    version_sync::assert_markdown_deps_updated!("README.md");
}

#[test]
fn html_root_version_is_correct() {
    version_sync::assert_html_root_url_updated!("src/lib.rs");
}
