mod common;
mod namespace;
mod version;

pub use namespace::{fqn_parser, namespace_parser, FullyQualifiedName, Namespace};
pub use version::{version_number_parser, version_parser, SemanticVersion, VersionNumber};

#[cfg(test)]
mod test {
    #[test]
    fn test_a() {
        assert!(true);
    }
}
