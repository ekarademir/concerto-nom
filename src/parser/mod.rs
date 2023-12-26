mod common;
mod namespace;
mod version;

pub use namespace::{namespace_parser, NamespaceVersion};
pub use version::{version_number_parser, version_parser, SemanticVersion, VersionNumber};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum BuiltIn {
    Namespace(NamespaceVersion),
}

#[cfg(test)]
mod test {
    #[test]
    fn test_a() {
        assert!(true);
    }
}
