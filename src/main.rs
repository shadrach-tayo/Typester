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
    let output_filename = matches.get_one::<String>("output").expect("output required");

    dbg!(input_filename);
    dbg!(output_filename);
}
