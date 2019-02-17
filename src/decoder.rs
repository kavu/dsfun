use std::io::{Read, Seek, SeekFrom, Write};

use failure::{bail, Error};

use crate::errors::IOError;
use crate::key_storage::{AbstractKeyStorage, SimpleKeyStorage};
use crate::reader_writer::{new_file_reader, new_file_writer};

fn decode_chunk(size: usize, key_storage: &mut impl AbstractKeyStorage, buffer: &mut Vec<u8>) {
    let half_size = size >> 1;

    for idx in 0..half_size {
        buffer[idx] ^= buffer[idx + half_size] ^ key_storage.next_key();
    }

    for byte in buffer.iter_mut().take(size).skip(half_size) {
        *byte ^= key_storage.next_key();
    }
}

pub fn decode_file(input_path: &str, output_path: &str) -> Result<(), Error> {
    let mut reader = new_file_reader(input_path)?;
    let mut writer = new_file_writer(output_path)?;

    let mut buffer: Vec<u8> = Vec::with_capacity(0x1000);

    let mut key_storage = SimpleKeyStorage::default();

    if let Err(io_err) = reader.seek(SeekFrom::Start(0x141)) {
        bail!(IOError::InputFileRead { context: io_err });
    }

    loop {
        match reader.by_ref().take(0x1000).read_to_end(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    break;
                } else {
                    decode_chunk(size, &mut key_storage, &mut buffer);
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
