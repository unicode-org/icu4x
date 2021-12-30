// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Utilities for extracting `ResourceKey` objects from a byte stream. Requires the "std" feature.

use crate::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::io;

pub fn extract_keys_from_byte_stream(stream: impl io::Read) -> io::Result<Vec<ResourceKey>> {
    let mut reader = BufReader::with_capacity(1024, stream);
    let mut working_buffer = [0u8; 1024 + 39];
    let mut output = Vec::new();
    loop {
        let reader_buffer = reader.fill_buf()?;
        if reader_buffer.len() == 0 {
            break;
        }
        let len = reader_buffer.len();
        // Save 39 bytes from iteration to iteration: one less than a 40-byte window
        working_buffer[39..(39+len)].copy_from_slice(reader_buffer);
        for window in working_buffer[..(39+len)].windows(40) {
            if &window[0..8] == b"ICURES[[" && &window[36..40] == b"]]**" {
                let mut bytes: [u8; 40] = [0; 40];
                bytes.copy_from_slice(window);
                let resc_key = match ResourceKey::from_repr_c(bytes) {
                    Some(k) => k,
                    None => continue
                };
                output.push(resc_key);
            }
        }
        reader.consume(len);
        working_buffer.copy_within(len.., 0);
    }
    output.sort();
    output.dedup();
    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::resource_key;

    const GOLDEN_BYTES: &[u8] = b"\x00\x00ICURES[[\x02\x00\x00\x00\x00\x00\x00\x00skeletons\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00]]**ICURES[[\x02\x00\x00\x00\x00\x00\x00\x00symbols\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00]]**\x00\x00";

    #[test]
    fn test_extract_golden() {
        let keys = extract_keys_from_byte_stream(&*GOLDEN_BYTES).unwrap();
        assert_eq!(keys, vec![
            resource_key!(DateTime, "skeletons", 1),
            resource_key!(DateTime, "symbols", 1),
        ]);
    }

    #[test]
    fn test_extract_large() {
        let keys: Vec<ResourceKey> = (0u8..=255u8).map(|i| resource_key!(Core, "demo", i)).collect();
        let mut buffer: Vec<u8> = Vec::new();
        for key in keys.iter() {
            // Insert some garbage
            buffer.extend(b"\x00ICURES[[\x00\x00]]**\x00\x00");
            // This is safe because we are transmuting to a POD type
            let key_bytes: [u8; 40] = unsafe { core::mem::transmute(*key) };
            buffer.extend(&key_bytes);
        }
        let extracted_keys = extract_keys_from_byte_stream(&*buffer).unwrap();
        assert_eq!(keys, extracted_keys);
    }
}
