pub const fn version_string() -> &'static str {
    concat!(
        env!("CARGO_PKG_VERSION"),
        "\n\nCopyright (c) 2025 Michael A Wright\n",
        "MIT License\n\n",
        "Repository: https://github.com/softwarewrighter/tutorial-editor\n",
        "Build Host: ",
        env!("BUILD_HOST"),
        "\nBuild Commit: ",
        env!("BUILD_COMMIT"),
        "\nBuild Time: ",
        env!("BUILD_TIME"),
    )
}
