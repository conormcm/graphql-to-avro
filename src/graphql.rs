use graphql_parser::{query, schema};

// re-export some GraphQL types with the string type fixed
pub type Definition<'a> = schema::Definition<'a, String>;
pub type Directive<'a> = schema::Directive<'a, String>;
pub type Document<'a> = schema::Document<'a, String>;
pub type Field<'a> = schema::Field<'a, String>;
pub type ObjectType<'a> = schema::ObjectType<'a, String>;
pub type ParseError = schema::ParseError;
pub type Type<'a> = query::Type<'a, String>;
pub type TypeDefinition<'a> = schema::TypeDefinition<'a, String>;
pub type Value<'a> = schema::Value<'a, String>;

/// Extend GraphQL's Document struct with some useful functions
pub trait DocumentExt<'a> {
    fn object_type(&'a self, name: &str) -> Option<&'a ObjectType>;
    fn first_object_type(&'a self) -> Option<&'a ObjectType>;
}

impl<'a> DocumentExt<'a> for Document<'a> {
    fn object_type(&'a self, name: &str) -> Option<&'a ObjectType> {
        self.definitions
            .iter()
            .find_map(|definition| match definition {
                Definition::TypeDefinition(TypeDefinition::Object(object_type))
                    if object_type.name == name =>
                {
                    Some(object_type)
                }
                _ => None,
            })
    }

    fn first_object_type(&'a self) -> Option<&'a ObjectType> {
        self.definitions
            .iter()
            .find_map(|definition| match definition {
                Definition::TypeDefinition(TypeDefinition::Object(object_type)) => {
                    Some(object_type)
                }
                _ => None,
            })
    }
}

/// Extend GraphQL's ObjectType struct with some useful functions
pub trait ObjectTypeExt<'a> {
    fn directive(&'a self, name: &str) -> Option<&'a Directive>;
}

impl<'a> ObjectTypeExt<'a> for ObjectType<'a> {
    fn directive(&'a self, name: &str) -> Option<&'a Directive> {
        self.directives
            .iter()
            .find(|directive| directive.name == name)
    }
}

/// Extend GraphQL's Field struct with some useful functions
pub trait FieldExt<'a> {
    fn directive(&'a self, name: &str) -> Option<&'a Directive>;
}

impl<'a> FieldExt<'a> for Field<'a> {
    fn directive(&'a self, name: &str) -> Option<&'a Directive> {
        self.directives
            .iter()
            .find(|directive| directive.name == name)
    }
}

/// Extend GraphQL's Directive struct with some useful functions
pub trait DirectiveExt<'a> {
    fn argument(&'a self, name: &str) -> Option<&'a Value>;
}

impl<'a> DirectiveExt<'a> for Directive<'a> {
    fn argument(&'a self, name: &str) -> Option<&'a Value> {
        self.arguments
            .iter()
            .find_map(|argument| (argument.0 == name).then(|| &argument.1))
    }
}
