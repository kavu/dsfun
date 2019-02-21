use clap::{crate_authors, crate_version};
use failure::Error;

mod dsf_coder;
mod errors;
mod file_io;

use dsf_coder::{decode, encode, CoderOptions, SimpleKey};
use file_io::{new_file_reader, new_file_writer};

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

    let force = clap::Arg::from_usage("-f, --force 'force output file writing'");

    let decode_subcommand = clap::SubCommand::with_name("decode")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .about("decode a DSF file")
        .arg(&force)
        .arg(&input_argument)
        .arg(&output_argument);

    let encode_subcommand = clap::SubCommand::with_name("encode")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .about("encode a DSF file")
        .arg(&force)
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
        let force = args.is_present("force");

        let options = CoderOptions::new().force(force);

        let mut reader = new_file_reader(input_file_path)?;
        let mut writer = new_file_writer(output_file_path, options)?;

        let mut key_storage = SimpleKey::default();
        let mut buffer: Vec<u8> = Vec::with_capacity(0x1000);

        match subcommand {
            "decode" => decode(&mut reader, &mut writer, &mut key_storage, &mut buffer)?,
            "encode" => encode(&mut reader, &mut writer, &mut key_storage, &mut buffer)?,
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
