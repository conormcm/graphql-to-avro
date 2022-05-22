use serde_json::Value as JsonValue;

use crate::avro;
use crate::graphql;
use crate::graphql::{DirectiveExt, DocumentExt, FieldExt, ObjectTypeExt};

#[derive(Debug)]
pub enum Error {
    UnknownType(String),
}

/// Converts a named type reference in GraphQL to an Avro schema
///
/// If the referenced type is user defined it will be outputted in full.
/// Simply referencing user defined types by name is not supported yet.
pub fn named_type<'a>(
    name: &str,
    graphql_document: &'a graphql::Document<'a>,
) -> Result<avro::Schema, Error> {
    match name {
        // GraphQL scalars, primitives listed in the spec (sans ID)
        "Boolean" => Ok(avro::PrimitiveType::Boolean.schema()),
        "Int" => Ok(avro::PrimitiveType::Boolean.schema()),
        "Float" => Ok(avro::PrimitiveType::Boolean.schema()),
        "String" => Ok(avro::PrimitiveType::String.schema()),

        // custom types for mapping remaining Avro primitives
        "Long" => Ok(avro::PrimitiveType::Long.schema()),
        "Double" => Ok(avro::PrimitiveType::Double.schema()),
        "Bytes" => Ok(avro::PrimitiveType::Bytes.schema()),
        "UUID" => Ok(avro::LogicalType::uuid().schema()),

        // user defined types
        name => match graphql_document.object_type(name) {
            Some(object_type) => {
                // TODO: beware of infinite recursion
                let avro_record = record(object_type, graphql_document)?;
                Ok(avro_record.schema())
            }
            None => Err(Error::UnknownType(name.to_string())),
        },
    }
}

/// Optionally makes an Avro schema a union with null
pub fn choose_nullability(avro_schema: avro::Schema, nullable: bool) -> avro::Schema {
    match nullable {
        false => avro_schema,
        true => {
            avro::UnionType::new(vec![avro::PrimitiveType::Null.schema(), avro_schema]).schema()
        }
    }
}

/// Maps a GraphQl type reference to an Avro schema
pub fn type_reference<'a>(
    graphql_type: &graphql::Type,
    graphql_document: &'a graphql::Document<'a>,
    item_name: Option<String>,
    nullable: bool,
) -> Result<avro::Schema, Error> {
    match graphql_type {
        graphql::Type::NonNullType(inner_type) => {
            type_reference(inner_type, graphql_document, item_name, false)
        }
        graphql::Type::NamedType(name) => {
            let avro_schema = named_type(name, graphql_document)?;
            Ok(choose_nullability(avro_schema, nullable))
        }
        graphql::Type::ListType(inner_type) => {
            let avro_array = array(inner_type, graphql_document, item_name)?;
            Ok(choose_nullability(avro_array.schema(), nullable))
        }
    }
}

/// Converts a GraphQL field definition to an Avro field definition
pub fn field<'a>(
    graphql_field: &'a graphql::Field<'a>,
    graphql_document: &'a graphql::Document<'a>,
) -> Result<avro::FieldDefinition, Error> {
    // TODO: validate item, must have a name string field
    let item_name = graphql_field
        .directive("item")
        .map(|item| match item.argument("name") {
            Some(graphql::Value::String(name)) => name.clone(),
            _ => "".to_string(),
        });
    let avro_schema = type_reference(&graphql_field.field_type, graphql_document, item_name, true)?;

    Ok(avro::FieldDefinition::new(
        graphql_field.name.clone(),
        avro_schema,
        // TODO: defaults for the actual type, allow specifying the value
        graphql_field.directive("default").map(|_| JsonValue::Null),
    ))
}

/// Converts a GraphQL record definition to an Avro record definition
pub fn record<'a>(
    object_type: &'a graphql::ObjectType<'a>,
    graphql_document: &'a graphql::Document<'a>,
) -> Result<avro::RecordDefinition, Error> {
    let avro_fields = object_type
        .fields
        .iter()
        .fold(Ok(vec![]), |result, graphql_field| {
            result.and_then(|mut avro_fields| {
                let avro_field = field(graphql_field, graphql_document)?;
                avro_fields.push(avro_field);
                Ok(avro_fields)
            })
        })?;

    Ok(avro::RecordDefinition::new(
        object_type.name.clone(),
        object_type.directive("namespace").map(|namespace| {
            // TODO: validate namespace, must have qualifier string field
            namespace
                .argument("qualifier")
                .and_then(|value| match value {
                    graphql::Value::String(value) => Some(value.clone()),
                    _ => None,
                })
                .unwrap_or_default()
        }),
        avro_fields,
    ))
}

/// Converts a GraphQL array type to an Avro array type
pub fn array<'a>(
    inner_type: &graphql::Type,
    graphql_document: &'a graphql::Document<'a>,
    item_name: Option<String>,
) -> Result<avro::ArrayType, Error> {
    let avro_schema = type_reference(inner_type, graphql_document, None, true)?;
    Ok(avro::ArrayType::new(Box::new(avro_schema), item_name))
}
