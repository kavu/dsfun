use clap::{crate_authors, crate_version};
use failure::Error;

mod decoder;
mod encoder;
mod errors;
mod key_storage;
mod reader_writer;

use encoder::encode_file;
use decoder::decode_file;

fn build_cli() -> clap::ArgMatches<'static> {
    let input_argument = clap::Arg::with_name("input")
        .help("input file")
        .value_name("INPUT_PATH")
        .takes_value(true)
        .required(true);

    let output_argument = clap::Arg::with_name("output")
        .help("output file")
        .value_name("OUTPUT_PATH")
        .takes_value(true)
        .required(true);

    let decode_subcommand = clap::SubCommand::with_name("decode")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .about("decode a DSF file")
        .arg(&input_argument)
        .arg(&output_argument);

    let encode_subcommand = clap::SubCommand::with_name("encode")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .about("encode a DSF file")
        .arg(&input_argument)
        .arg(&output_argument);

    clap::App::new("dsfun")
        .about("dsfun is a small utility programm for decoding and encode Dune media players' Dune Service File (DSF) executables")
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .subcommand(decode_subcommand)
        .subcommand(encode_subcommand)
        .get_matches()
}

fn run() -> Result<(), Error> {
    let cli = build_cli();

    if let (subcommand, Some(args)) = cli.subcommand() {
        let input_file_path = args.value_of("input").unwrap();
        let output_file_path = args.value_of("output").unwrap();

        match subcommand {
            "decode" => decode_file(input_file_path, output_file_path)?,
            "encode" => encode_file(input_file_path, output_file_path)?,
            _ => unreachable!(),
        }
    };

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
