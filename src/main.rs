extern crate failure_derive;

use clap::{crate_authors, crate_version};
use failure::Error;

mod reader_writer;
mod decoder;
mod errors;
mod key_storage;

use decoder::decode_file;

fn run() -> Result<(), Error> {
    let decode_subcommand = clap::SubCommand::with_name("decode")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .about("decode a DSF file")
        .arg(
            clap::Arg::with_name("input")
                .help("input file to decode")
                .value_name("INPUT_PATH")
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("output")
                .help("output file")
                .value_name("OUTPUT_PATH")
                .takes_value(true)
                .default_value("decoded.gz")
                .required(false),
        );

    let app = clap::App::new("dsfun")
        .about("dsfun is a small utility programm for decoding and encode Dune media players' Dune Service File (DSF) executables")
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .subcommand(decode_subcommand)
        .get_matches();

    if let ("decode", Some(arg_matches)) = app.subcommand() {
        // Safe unwrap, because argument is required
        let input_file_path = arg_matches.value_of("input").unwrap();
        let output_file_path = arg_matches.value_of("output").unwrap();

        decode_file(input_file_path, output_file_path)?;
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
