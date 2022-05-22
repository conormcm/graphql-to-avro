use clap::Parser;
use graphql_parser::schema::parse_schema;
use std::fs;

use graphql_to_avro::convert;
use graphql_to_avro::graphql;
use graphql_to_avro::graphql::DocumentExt;

#[derive(Debug)]
enum Error {
    Conversion(convert::Error),
    File(std::io::Error),
    GraphQLParser(graphql::ParseError),
    NoTypesFound,
}

/// Converts GraphQL type definitions to Avro JSON schemas
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// The input .graphql file
    input_file: String,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input_file).map_err(Error::File)?;
    let document = parse_schema::<String>(&contents).map_err(Error::GraphQLParser)?;
    let first_object_type = document.first_object_type().ok_or(Error::NoTypesFound)?;
    let output = convert::record(first_object_type, &document).map_err(Error::Conversion)?;

    println!("{}", serde_json::to_string_pretty(&output).unwrap());

    Ok(())
}
