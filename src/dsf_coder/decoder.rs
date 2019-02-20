use std::io::{Read, Seek, SeekFrom, Write};

use failure::{bail, Error};

use super::coding_loop::run_coding_loop;
use super::key::{AbstractKey, UnwindableKey};
use crate::errors::IOError;

const DATA_LOCATION: SeekFrom = SeekFrom::Start(0x141);

fn decode_chunk(size: usize, key_storage: &mut impl AbstractKey, buffer: &mut Vec<u8>) {
    let half_size = size >> 1;

    for idx in 0..half_size {
        buffer[idx] ^= buffer[idx + half_size] ^ key_storage.next_key();
    }

    for byte in buffer.iter_mut().take(size).skip(half_size) {
        *byte ^= key_storage.next_key();
    }
}

pub fn decode<R, W, K>(
    input: &mut R,
    output: &mut W,
    key_storage: &mut K,
    buffer: &mut Vec<u8>,
) -> Result<(), Error>
where
    R: Seek + Read,
    W: Write,
    K: UnwindableKey,
{
    if let Err(io_err) = input.seek(DATA_LOCATION) {
        bail!(IOError::InputFileRead { context: io_err });
    }

    run_coding_loop(input, output, key_storage, buffer, decode_chunk)?;

    Ok(())
}
