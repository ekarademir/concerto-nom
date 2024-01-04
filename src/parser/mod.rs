pub mod common;
pub mod declaration;
pub mod error;
pub mod namespace;
pub mod property;
pub mod version;

use nom::{
    branch::alt, character::complete::multispace0, error::context, multi::fold_many0,
    sequence::delimited, IResult, Parser,
};

/// Concerto parse result type
pub type CResult<I, O> = IResult<I, O, error::CError<I>>;

#[derive(Debug, PartialEq, Clone)]
pub struct Model {
    pub namespace: namespace::Namespace,
    pub declarations: Vec<declaration::Declaration>,
}

enum Definition {
    Namespace(namespace::Namespace),
    Declaration(declaration::Declaration),
}

struct ModelBuilder {
    pub namespace: Option<namespace::Namespace>,
    pub declarations: Vec<declaration::Declaration>,
}

impl ModelBuilder {
    pub fn new() -> Self {
        Self {
            namespace: None,
            declarations: Vec::new(),
        }
    }

    pub fn with_namespace(&mut self, ns: namespace::Namespace) -> &Self {
        self.namespace = Some(ns);
        self
    }

    pub fn add_declaration(&mut self, dec: declaration::Declaration) -> &Self {
        self.declarations.push(dec);
        self
    }

    pub fn build(self) -> Model {
        Model {
            namespace: self.namespace.unwrap(),
            declarations: self.declarations,
        }
    }
}

pub fn model<'a>(input: &'a str) -> CResult<&'a str, Model> {
    let definition = alt((
        namespace::namespace_identifier.map(|ns| Definition::Namespace(ns)),
        declaration::declaration.map(|dec| Definition::Declaration(dec)),
    ));
    let definitions = fold_many0(
        delimited(multispace0, definition, multispace0),
        Vec::new,
        |mut acc, item| {
            acc.push(item);
            acc
        },
    );
    context(
        "Model",
        definitions.map(|defs| {
            let mut model_builder = ModelBuilder::new();
            for def in defs {
                match def {
                    Definition::Declaration(d) => {
                        model_builder.add_declaration(d);
                    }
                    Definition::Namespace(ns) => {
                        model_builder.with_namespace(ns);
                    }
                }
            }
            model_builder.build()
        }),
    )(input)
}
