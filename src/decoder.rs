use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};

use crate::key_storage::{AbstractKeyStorage, SimpleKeyStorage};

fn decode_chunk(size: usize, key_storage: &mut impl AbstractKeyStorage, buffer: &mut Vec<u8>) {
    println!("Read {:X}", size);
    let half_size = size >> 1;
    println!("Half {:X}", half_size);

    for idx in 0..half_size {
        let key = key_storage.next_key();
        let byte = buffer[idx + half_size];
        let a1 = buffer[idx];

        let mut decoded_byte = byte;
        decoded_byte ^= key;
        decoded_byte ^= a1;

        println!(
            "  [{:02X}]={:02X}, xor {:02X}, [{:02X}]={:02X}, storing {:02X}",
            idx + half_size,
            byte,
            key,
            idx,
            a1,
            decoded_byte
        );

        buffer[idx] = decoded_byte;
    }

    for (idx, byte) in buffer.iter_mut().enumerate().take(size).skip(half_size) {
        let key = key_storage.next_key();
        let mut decoded_byte = key;

        decoded_byte ^= *byte;

        println!(
            "  xor {:02X}, [{:02x}]={:02X}, storing {:02X}",
            key, idx, byte, decoded_byte
        );

        *byte = decoded_byte;
    }
}

pub fn decode_file(input_path: &str, output_path: &str) -> Result<(), Box<Error>> {
    let f = File::open(input_path)?;
    let mut reader = BufReader::new(f);

    let out = File::create(output_path)?;
    let mut writer = BufWriter::new(out);

    let mut buffer: Vec<u8> = Vec::with_capacity(0x1000);

    let mut key_storage = SimpleKeyStorage::default();

    reader.seek(SeekFrom::Start(0x141))?;

    let loop_reader = reader.by_ref();
    loop {
        match loop_reader.take(0x1000).read_to_end(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    break;
                } else {
                    decode_chunk(size, &mut key_storage, &mut buffer);
                }
            }
            Err(err) => panic!(err),
        }

        writer.write_all(&buffer)?;
        buffer.clear();
    }

    Ok(())
}
