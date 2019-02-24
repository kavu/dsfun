use std::io::{Read, Seek, Write};

use failure::{bail, Error};

use super::decoder;
use super::encoder;
use super::key::AbstractKey;
use crate::errors::IOError;

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

pub struct Coder<R, W, K>
where
    R: Seek + Read,
    W: Write,
    K: AbstractKey,
{
    input: R,
    output: W,
    key_storage: K,
    buffer: Vec<u8>,
}

impl<R, W, K> Coder<R, W, K>
where
    R: Seek + Read,
    W: Write,
    K: AbstractKey,
{
    pub fn new(input: R, output: W, key_storage: K, buffer: Vec<u8>) -> Self {
        Coder {
            input,
            output,
            key_storage,
            buffer,
        }
    }

    pub fn run(&mut self, method: &str) -> Result<(), Error> {
        let processing_fn = match method {
            "decode" => {
                decoder::seek_data_location_in(&mut self.input)?;
                decoder::decode_chunk
            }
            "encode" => {
                encoder::write_header_to(&mut self.output)?;
                encoder::encode_chunk
            }
            _ => unreachable!(),
        };

        self.run_coding_loop(processing_fn)
    }

    fn run_coding_loop<F>(&mut self, coder: F) -> Result<(), Error>
    where
        F: Fn(usize, &K, &mut Vec<u8>),
    {
        loop {
            match self
                .input
                .by_ref()
                .take(0x1000)
                .read_to_end(&mut self.buffer)
            {
                Ok(size) => {
                    if size == 0 {
                        break;
                    } else {
                        coder(size, &self.key_storage, &mut self.buffer);
                    }
                }
                Err(io_err) => bail!(IOError::InputFileRead { context: io_err }),
            }

            if let Err(io_err) = self.output.write_all(&self.buffer) {
                bail!(IOError::OutputFileWrite { context: io_err });
            }

            self.buffer.clear();
        }

        Ok(())
    }
}
