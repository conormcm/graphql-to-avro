# graphql-to-avro

`graphql-to-avro` is a small CLI tool for converting GraphQL type definitions to Avro JSON schemas.

# Getting Started

## Prerequisites

- [Rust](https://www.rust-lang.org/) (quickstart with [rustup](https://rustup.rs/))

## Building/Installation

`cargo build --release` generates a `graphql-to-avro` binary in `./target/release`.

# Usage

```
CLI tool for converting GraphQL type definitions to Avro JSON schemas

USAGE:
    graphql-to-avro <INPUT_FILE>

ARGS:
    <INPUT_FILE>    The input .graphql file

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

## Example Input File

```graphql
type Product @namespace(qualifier: "com.acme.avro") {
  id: UUID!,
  name: String!,
  vendor: Vendor!,
  description: String @default,
}

type Vendor {
  id: UUID!,
  name: String!,
  address: [String!]! @item(name: "line"),
}
```

## Example Output

<details>
  <summary>Click to expand</summary>

  ```json
  {
    "type": "record",
    "name": "Product",
    "namespace": "com.acme.avro",
    "fields": [
      {
        "name": "id",
        "type": {
          "type": "string",
          "logicalType": "uuid"
        }
      },
      {
        "name": "name",
        "type": "string"
      },
      {
        "name": "vendor",
        "type": {
          "type": "record",
          "name": "Vendor",
          "fields": [
            {
              "name": "id",
              "type": {
                "type": "string",
                "logicalType": "uuid"
              }
            },
            {
              "name": "name",
              "type": "string"
            },
            {
              "name": "address",
              "type": {
                "type": "array",
                "items": "string",
                "name": "line"
              }
            }
          ]
        }
      },
      {
        "name": "description",
        "type": [
          "null",
          "string"
        ],
        "default": null
      }
    ]
  }
  ```
</details>

# GraphQL Format Notes

- The first `type` declaration is used for the output.
- Any `type` that is referenced by another `type` is embedded in the Avro schema. Keeping a named reference without embedding is not supported yet.
- Only supports `type` declarations (for now), no `union` or `enum` yet. Everything other than `type` is ignored.
- Supports basic GraphQL scalar types `Boolean`/`Int`/`Float`/`String` (maps to Avro primitives). `ID` is omitted.
- Adds `Long`/`Double`/`Bytes` types (maps to the remaining Avro primitives).
- Adds explicit `UUID` type (maps to native logical type in Avro).
- The `@namespace(qualifier: String)` directive adds a `namespace` field to records with a value of `qualifier`.
- The `@default` directive adds a `default` field to record fields with a value of `null`.
- The `@item(name: String)` directive adds a `name` field to array types with a value of `name`. (This field is not in the Avro spec)
