use std::io::{Read, Seek, Write};

use failure::{bail, Error};

use super::decoder;
use super::encoder;
use super::key::UnwindableKey;
use crate::errors::IOError;

#[derive(Clone, Copy)]
pub struct CoderOptions {
    force: bool,
}

impl CoderOptions {
    pub fn new() -> Self {
        CoderOptions { force: false }
    }

    pub fn get_force(self) -> bool {
        self.force
    }

    pub fn force(mut self, force: bool) -> Self {
        self.force = force;

        self
    }
}

pub struct Coder<'a, R, W, K>
where
    R: Seek + Read,
    W: Write,
    K: UnwindableKey,
{
    input: &'a mut R,
    output: &'a mut W,
    key_storage: &'a mut K,
    buffer: &'a mut Vec<u8>,
}

impl<'a, R, W, K> Coder<'a, R, W, K>
where
    R: Seek + Read,
    W: Write,
    K: UnwindableKey,
{
    pub fn new(
        input: &'a mut R,
        output: &'a mut W,
        key_storage: &'a mut K,
        buffer: &'a mut Vec<u8>,
    ) -> Self {
        Coder {
            input,
            output,
            key_storage,
            buffer,
        }
    }

    pub fn run(self, method: &str) -> Result<(), Error> {
        let processing_fn = match method {
            "decode" => {
                decoder::seek_data_location_in(self.input)?;
                decoder::decode_chunk
            }
            "encode" => {
                encoder::write_header_to(self.output)?;
                encoder::encode_chunk
            }
            _ => unreachable!(),
        };

        self.run_coding_loop(processing_fn)
    }

    fn run_coding_loop<F>(self, coder: F) -> Result<(), Error>
    where
        F: Fn(usize, &mut K, &mut Vec<u8>),
    {
        loop {
            match self.input.by_ref().take(0x1000).read_to_end(self.buffer) {
                Ok(size) => {
                    if size == 0 {
                        break;
                    } else {
                        coder(size, self.key_storage, self.buffer);
                    }
                }
                Err(io_err) => bail!(IOError::InputFileRead { context: io_err }),
            }

            if let Err(io_err) = self.output.write_all(self.buffer) {
                bail!(IOError::OutputFileWrite { context: io_err });
            }

            self.buffer.clear();
        }

        Ok(())
    }
}
