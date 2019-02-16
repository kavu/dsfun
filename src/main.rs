mod decoder;
mod key_storage;

use decoder::decode_file;

fn main() -> std::io::Result<()> {
    match decode_file("dune_service_af_make_whole_ext2_on_hdd.dsf", "outfile.test") {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("{}", err);
            std::process::exit(1)
        }
    }
}
