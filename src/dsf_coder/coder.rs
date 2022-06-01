use std::io::{Read, Seek, Write};

use super::decoder;
use super::encoder;
use super::key::Abstract;
use crate::errors::IOError;

pub struct Options {
    force: bool,
}

impl Options {
    pub const fn new() -> Self {
        Self { force: false }
    }

    pub const fn get_force(self) -> bool {
        self.force
    }

    pub const fn force(mut self, force: bool) -> Self {
        self.force = force;

        self
    }
}

pub struct Coder<R, W, K>
where
    R: Seek + Read,
    W: Write,
    K: Abstract,
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
    K: Abstract,
{
    pub fn new(input: R, output: W, key_storage: K) -> Self {
        Self {
            input,
            output,
            key_storage,
            buffer: Vec::with_capacity(0x1000),
        }
    }

    pub fn run(&mut self, method: &str) -> Result<(), IOError> {
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

    fn run_coding_loop<F>(&mut self, coder: F) -> Result<(), IOError>
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
                    }

                    coder(size, &self.key_storage, &mut self.buffer);
                }
                Err(io_err) => return Err(IOError::InputFileRead { source: io_err }),
            }

            if let Err(io_err) = self.output.write_all(&self.buffer) {
                return Err(IOError::OutputFileWrite { source: io_err });
            }

            self.buffer.clear();
        }

        Ok(())
    }
}
