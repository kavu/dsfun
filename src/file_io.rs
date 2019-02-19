use std::fs::File;
use std::io::{BufReader, BufWriter};

use failure::{bail, Error};

use crate::errors::IOError;

pub fn new_file_reader(input_path: &str) -> Result<BufReader<File>, Error> {
    match File::open(input_path) {
        Ok(file) => Ok(BufReader::new(file)),
        Err(io_err) => bail!(IOError::InputFileOpen {
            context: io_err,
            path: input_path.into()
        }),
    }
}

pub fn new_file_writer(output_path: &str) -> Result<BufWriter<File>, Error> {
    match File::create(output_path) {
        Ok(file) => Ok(BufWriter::new(file)),
        Err(io_err) => bail!(IOError::OutputFileOpen {
            context: io_err,
            path: output_path.into()
        }),
    }
}
