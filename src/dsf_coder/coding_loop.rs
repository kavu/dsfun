use std::io::{Read, Write};

use failure::{bail, Error};

use super::key::UnwindableKey;
use crate::errors::IOError;

pub fn run_coding_loop<R, W, F, S>(
    reader: &mut R,
    writer: &mut W,
    key_storage: &mut S,
    buffer: &mut Vec<u8>,
    coder: F,
) -> Result<(), Error>
where
    R: Read,
    W: Write,
    F: Fn(usize, &mut S, &mut Vec<u8>),
    S: UnwindableKey,
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
