fn main() {
    // Copy the images to the output when generating documentation
    println!("cargo:rerun-if-changed=assets");

    // create the target/doc/assets directory if it doesn't exist
    std::fs::create_dir_all("target/doc/assets")
        .expect("Failed to create target/doc/assets directory when building documentation.");

    std::fs::copy("assets/icon.ico", "target/doc/assets/icon.ico")
        .expect("Failed to copy crate favicon when building documentation.");
    std::fs::copy("assets/icon.png", "target/doc/assets/icon.png")
        .expect("Failed to copy crate logo when building documentation.");
}
