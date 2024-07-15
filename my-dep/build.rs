fn main() {
    #[cfg(all(not(clippy), feature = "feature1", feature = "feature2"))]
    {
        compile_error!("feature1 and feature2 cannot be used together");
    }
}
