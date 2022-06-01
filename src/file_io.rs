use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};

use crate::dsf_coder::Options;
use crate::errors::IOError;

pub fn new_file_reader(input_path: &str) -> Result<BufReader<File>, IOError> {
    match File::open(input_path) {
        Ok(file) => Ok(BufReader::new(file)),
        Err(io_err) => Err(IOError::InputFileOpen {
            path: input_path.into(),
            source: io_err,
        }),
    }
}

pub fn new_file_writer(output_path: &str, options: Options) -> Result<BufWriter<File>, IOError> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .create_new(!options.get_force())
        .open(output_path);

    match file {
        Ok(file) => Ok(BufWriter::new(file)),
        Err(io_err) => Err(IOError::OutputFileOpen {
            path: output_path.into(),
            source: io_err,
        }),
    }
}
