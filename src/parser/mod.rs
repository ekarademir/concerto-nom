mod common;
mod namespace;
mod version;

pub use namespace::{namespace_parser, Namespace};
pub use version::{version_number_parser, version_parser, SemanticVersion, VersionNumber};

#[cfg(test)]
mod test {
    #[test]
    fn test_a() {
        assert!(true);
    }
}
