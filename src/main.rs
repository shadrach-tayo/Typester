use std::{
    fs::{self, File},
    io::Write,
};

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Typester")
        .version("0.1.0")
        .author("Shadrach")
        .about("Convert Rust types to Typescript types")
        .arg(
            Arg::new("input")
                .short('i')
                .required(true)
                .long("input")
                .help("The Rust file to process (including extension)"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .required(true)
                .long("output")
                .help("The name of the Typescript file to output (including extension)"),
        )
        .get_matches();

    let input_filename = matches.get_one::<String>("input").expect("Input required");
    let output_filename = matches
        .get_one::<String>("output")
        .expect("output required");

    dbg!(input_filename);
    dbg!(output_filename);

    let input_file_text = fs::read_to_string(input_filename)
        .expect(&format!("Unable to open file {}", input_filename));

    let input_syntax: syn::File = syn::parse_file(&input_file_text).expect("Unable to parse file");
    // dbg!(&input_syntax);

    let mut output_text = String::new();

    for item in input_syntax.items.iter() {
        match item {
            syn::Item::Type(item_type) => {
                let type_text = parse_item_type(item_type);
                output_text.push_str(&type_text);
                output_text.push_str("\n");
            }
            syn::Item::Enum(item_enum) => {
                let enum_text = parse_item_enum(item_enum);
                output_text.push_str(&enum_text);
            }
            syn::Item::Struct(item_struct) => {
                let struct_text = parse_item_struct(item_struct);
                output_text.push_str(&struct_text);
            }
            _ => {
                dbg!("Encountered an unimplemented type {}", item);
            }
        }
        output_text.push_str("\n");
    }
    // dbg!(&output_text);
    let mut output_file = File::create(output_filename).unwrap();
    write!(output_file, "{}", output_text).expect("Failed to write to output file");
}

/// Converts a Rust item type to a Typescript type
///
/// ## Examples
///
/// **Input:** type NumberAlias = i32;
///
/// **Output:** export type NumberAlias = number;
fn parse_item_type(item_type: &syn::ItemType) -> String {
    let mut output = String::new();

    output.push_str("export type ");

    // ``ident`` is the name of the type alias, e.g ``NumberAlias`` from the example
    output.push_str(&item_type.ident.to_string());
    output.push_str(" = ");

    let type_string = parse_type(&item_type.ty);
    output.push_str(&type_string);
    output.push_str(";");
    output
}

/// Converts a Rust type into a Typescript type
///
/// ## Examples
///
/// **Input:** (i32, i32) / Option<String>
///
/// **Output:** \[number, number\] / Option<string>;
fn parse_type(syn_type: &syn::Type) -> String {
    let mut output = String::new();

    match syn_type {
        // Primitive types like i32 will match Path
        // We currently do not do anything with full paths
        // so we take only the last() segment (the type name)
        syn::Type::Path(type_path) => {
            let segment = type_path.path.segments.last().unwrap();

            let field_type = segment.ident.to_string();
            let ts_field_type = parse_type_ident(&field_type);
            output.push_str(&ts_field_type.to_owned());

            match &segment.arguments {
                syn::PathArguments::None => {}
                
                _ => {
                    dbg!("Field type arguments token not implemented");
                }
            }
        }
        syn::Type::Tuple(tuple_type) => {
                    output.push_str("[");
                    for elem in tuple_type.elems.iter() {
                        output.push_str(&parse_type(&elem));
                        output.push_str(",");
                    }
                    output.push_str("]");
                }
        _ => {
            dbg!("parse_type::=> Encountered an unimplemented token");
        }
    }
    output
}

fn parse_type_ident(ident: &str) -> &str {
    match ident {
        "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" | "u64" | "f32" | "f64"
        | "isize" | "usize" => "number",
        "str" | "&str" | "String" | "char" => "string",
        "bool" => "boolean",
        _ => ident,
    }
}

/// Converts a Rust enum to a Typescript type
///
/// ## Examples
///
/// **Input:**
/// enum Colour {
///     Red(i32, i32),
///     Green(i32),
///     Blue(i32),
/// }
///
/// **Output:**
/// export type Colour =
///   | { t: "Red"; c: number }
///   | { t: "Green"; c: number }
///   | { t: "Blue"; c: number };
fn parse_item_enum(item_enum: &syn::ItemEnum) -> String {
    let mut output = String::new();
    output.push_str("export type ");
    output.push_str(&item_enum.ident.to_string());
    output.push_str(" = ");

    for variant in item_enum.variants.iter() {
        // For simplicity this implementation we are using assumes that enums will be
        // using serde's "Adjacently Tagged" attribute
        // #[serde(tag = "t", content = "c")]
        // https://serde.rs/enum-representations.html#adjacently-tagged
        // As an improvement on this implementation you could parse the attribute
        // and handle the enum differently depending on which attribute the user chose

        let variant_name = &variant.ident.to_string();
        output.push_str("| { t: \"");
        output.push_str(&variant_name);
        output.push_str("\" , content: ");

        // let item_type = &variant.fields.iter().last().unwrap().ty;
        // match item_type {
        //     syn::Type::Path(type_path) => {
        //         let segment = type_path.path.segments.last().unwrap();

        //         let type_ident = segment.ident.to_string();
        //         let type_ident = parse_type_ident(&type_ident);
        //         output.push_str(type_ident);
        //     }
        //     _ => {
        //         output.push_str("any");
        //         dbg!("Encountered invalid enum type path");
        //     }
        // };

        match &variant.fields {
            syn::Fields::Named(named_fields) => {
                output.push_str("{");
                for field in named_fields.named.iter() {
                    if let Some(ident) = &field.ident {
                        output.push_str(&ident.to_string());
                        output.push_str(":");

                        let field_type = parse_type(&field.ty);
                        output.push_str(&field_type);
                        output.push_str(";");
                    }
                }
                output.push_str("}");
            }
            syn::Fields::Unnamed(unnamed_fields) => {
                // Currently only support a single unnamed field: e.g the i32 in Blue(i32)
                let unnamed_field = unnamed_fields.unnamed.last().unwrap();
                let field_type = parse_type(&unnamed_field.ty);
                output.push_str(&field_type);
            }
            syn::Fields::Unit => {
                output.push_str("undefined");
            }
        }

        output.push_str(" } ")
    }

    output
}

/// Converts a Rust struct to a Typescript interface
///
/// ## Examples
///
/// **Input:**
/// struct Person {
///     name: String,
///     age: u32,
///     enjoys_coffee: bool,
/// }
///
/// **Output:**
/// export interface Person {
///     name: string;
///     age: number;
///     enjoys_coffee: boolean;
/// }
fn parse_item_struct(item_struct: &syn::ItemStruct) -> String {
    let mut output = String::new();
    let struct_name = item_struct.ident.to_string();

    output.push_str("export interface ");
    output.push_str(&struct_name);
    output.push_str(" { \n");

    match &item_struct.fields {
        syn::Fields::Named(named_fields) => {
            for named_field in named_fields.named.iter() {
                match &named_field.ident {
                    Some(ident) => {
                        let field_name = ident.to_string();
                        output.push_str(&field_name);
                        output.push_str(": ");
                    }
                    None => todo!(),
                }
                let field_type = parse_type(&named_field.ty);
                output.push_str(&field_type);
                output.push_str("; \n")
            }
        }

        // For tuple structs we will serialize them as interfaces with
        // fields named for the numerical index to align with serde's
        // default handling of this type
        syn::Fields::Unnamed(fields) => {
            for (index, field) in fields.unnamed.iter().enumerate() {
                output.push_str(&index.to_string());
                output.push_str(":");
                output.push_str(&parse_type(&field.ty));
                output.push_str("; \n");
            }
        }
        syn::Fields::Unit => (),
    };

    output.push_str("}");

    output
}
