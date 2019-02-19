use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::io::{Read, Write};

use failure::{bail, Error};

use crate::errors::IOError;
use crate::key_storage::UnwindableKeyStorage;

pub fn run_coding_loop<F, S>(
    reader: &mut BufReader<File>,
    writer: &mut BufWriter<File>,
    key_storage: &mut S,
    buffer: &mut Vec<u8>,
    coder: F,
) -> Result<(), Error>
where
    F: Fn(usize, &mut S, &mut Vec<u8>),
    S: UnwindableKeyStorage,
{
    loop {
        match reader.by_ref().take(0x1000).read_to_end(buffer) {
            Ok(size) => {
                if size == 0 {
                    break;
                } else {
                    coder(size, key_storage, buffer);
                }
            }
            Err(io_err) => bail!(IOError::InputFileRead { context: io_err }),
        }

        if let Err(io_err) = writer.write_all(&buffer) {
            bail!(IOError::OutputFileWrite { context: io_err });
        }

        buffer.clear();
    }

    Ok(())
}
