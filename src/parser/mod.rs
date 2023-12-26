pub mod common;
pub mod namespace;
pub mod version;

pub use namespace::namespace_definition_parser;
// pub use version::{version_number_parser, version_parser, SemanticVersion, VersionNumber};

#[cfg(test)]
mod test {
    #[test]
    fn test_a() {
        assert!(true);
    }
}
