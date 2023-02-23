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
            _ => {
                dbg!("Encountered an unimplemented type");
            }
        }
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
            println!("Ts type ident {ts_field_type}");
            output.push_str(&ts_field_type.to_owned());

            match &segment.arguments {
                syn::PathArguments::None => {}
                _ => {
                    dbg!("Field type arguments token not implemented");
                }
            }
        }
        // syn::Type::Reference(type_referece) => {
        //     // let ref_el = type_referece.elem;
        //     match *type_referece.elem {
        //         syn::Type::Path(type_path) => {
        //             let segment = type_path.path.segments.last().unwrap();

        //             let field_type = segment.ident.to_string();
        //             let ts_field_type = parse_type_ident(&field_type);
        //             println!("Ts type ident {ts_field_type}");
        //             output.push_str(&ts_field_type.to_owned());

        //             match &segment.arguments {
        //                 syn::PathArguments::None => {}
        //                 _ => {
        //                     dbg!("Reference Type Field type arguments token not implemented");
        //                 }
        //             }
        //         }
        //         _ => {
        //             dbg!("Unknown Reference path ");
        //         }
        //     }
        // }
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
