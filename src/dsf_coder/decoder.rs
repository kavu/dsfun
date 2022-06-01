use std::io::{Read, Seek, SeekFrom};

use super::key;
use crate::errors::IOError;

const DATA_LOCATION: SeekFrom = SeekFrom::Start(0x141);

pub fn seek_data_location_in<R: Read + Seek>(input: &mut R) -> Result<(), IOError> {
    if let Err(io_err) = input.seek(DATA_LOCATION) {
        return Err(IOError::InputFileRead { source: io_err });
    }

    Ok(())
}

pub fn decode_chunk(size: usize, key_storage: &impl key::Abstract, buffer: &mut Vec<u8>) {
    let half_size = size >> 1;

    for idx in 0..half_size {
        buffer[idx] ^= buffer[idx + half_size] ^ key_storage.get_key(idx);
    }

    for (idx, byte) in buffer.iter_mut().enumerate().take(size).skip(half_size) {
        *byte ^= key_storage.get_key(idx);
    }
}
