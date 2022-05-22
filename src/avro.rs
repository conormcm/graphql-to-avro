use serde::Serialize;
use serde_json::Value as JsonValue;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Schema {
    Primitive(PrimitiveType),
    Record(RecordDefinition),
    Array(ArrayType),
    Union(UnionType),
    Logical(LogicalType),
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PrimitiveType {
    Null,
    Boolean,
    Int,
    Long,
    Float,
    Double,
    Bytes,
    String,
}

impl PrimitiveType {
    pub fn schema(self) -> Schema {
        Schema::Primitive(self)
    }
}

#[derive(Serialize, Debug)]
pub struct RecordDefinition {
    pub r#type: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    pub fields: Vec<FieldDefinition>,
}

impl RecordDefinition {
    pub fn new(
        name: String,
        namespace: Option<String>,
        fields: Vec<FieldDefinition>,
    ) -> RecordDefinition {
        RecordDefinition {
            r#type: "record".to_string(),
            name,
            namespace,
            fields,
        }
    }

    pub fn schema(self) -> Schema {
        Schema::Record(self)
    }
}

#[derive(Serialize, Debug)]
pub struct FieldDefinition {
    pub name: String,
    pub r#type: Schema,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<JsonValue>,
}

impl FieldDefinition {
    pub fn new(name: String, r#type: Schema, default: Option<JsonValue>) -> FieldDefinition {
        FieldDefinition {
            name,
            r#type,
            default,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ArrayType {
    pub r#type: String,
    pub items: Box<Schema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>, // can't find this field in the Avro spec?
}

impl ArrayType {
    pub fn new(items: Box<Schema>, name: Option<String>) -> ArrayType {
        ArrayType {
            r#type: "array".to_string(),
            items,
            name,
        }
    }

    pub fn schema(self) -> Schema {
        Schema::Array(self)
    }
}

#[derive(Serialize, Debug)]
#[serde(transparent)]
pub struct UnionType {
    pub variants: Vec<Schema>,
}

impl UnionType {
    pub fn new(variants: Vec<Schema>) -> UnionType {
        UnionType { variants }
    }

    pub fn schema(self) -> Schema {
        Schema::Union(self)
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LogicalType {
    pub r#type: String,
    pub logical_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<u8>,
}

impl LogicalType {
    pub fn new<S>(
        r#type: S,
        logical_type: S,
        scale: Option<u8>,
        precision: Option<u8>,
    ) -> LogicalType
    where
        S: Into<String>,
    {
        LogicalType {
            r#type: r#type.into(),
            logical_type: logical_type.into(),
            scale,
            precision,
        }
    }

    pub fn uuid() -> LogicalType {
        LogicalType::new("string", "uuid", None, None)
    }

    pub fn schema(self) -> Schema {
        Schema::Logical(self)
    }
}
