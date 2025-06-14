use super::BencodeError;
use super::BencodeValue;
use super::Result;
use std::collections::HashMap;
use std::f64::consts::E;
use std::io::{self, Read};
use std::str::FromStr;
use tracing::instrument;

pub fn decode_string<R: Read>(reader: &mut std::iter::Peekable<io::Bytes<R>>) -> Result<Vec<u8>> {
    // Not implemented yet
    let length_str = read_until(reader, b':')?;
    let length = length_str
        .parse::<usize>()
        .map_err(|_| BencodeError::InvalidStringLength)?;

    let mut string_bytes = vec![0; length];
    for i in 0..length {
        string_bytes[i] = reader
            .next()
            .ok_or(BencodeError::UnexpectedEOI)?
            .map_err(|e| BencodeError::Io(e.kind().into()))?;
    }
    Ok(string_bytes)
}

/// Reads bytes from the reader until a specified delimiter is encountered.
///
/// This function reads bytes one at a time from the input stream until it finds
/// the specified delimiter byte. It collects all bytes read (excluding the delimiter)
/// into a buffer and returns them as a UTF-8 string.
///
/// # Arguments
/// * `reader` - A peekable iterator over the bytes of the input stream
/// * `delimiter` - The byte value that marks the end of the reading
///
/// # Returns
/// * `Result<String>` - The collected bytes as a UTF-8 string, or an error if:
///   - The input ends unexpectedly
///   - An I/O error occurs
///   - The collected bytes are not valid UTF-8
pub fn read_until<R: Read>(
    reader: &mut std::iter::Peekable<io::Bytes<R>>,
    delimiter: u8,
) -> Result<String> {
    let mut buffer = Vec::new();

    loop {
        let &current_byte = reader
            .peek()
            .ok_or(BencodeError::UnexpectedEOI)?
            .as_ref()
            .map_err(|e| BencodeError::Io(e.kind().into()))?;

        if current_byte == delimiter {
            reader
                .next()
                .ok_or(BencodeError::UnexpectedEOI)?
                .map_err(|e| BencodeError::Io(e.kind().into()))?;
            break;
        } else {
            buffer.push(
                reader
                    .next()
                    .ok_or(BencodeError::UnexpectedEOI)?
                    .map_err(|e| BencodeError::Io(e.kind().into()))?,
            );
        }
    }

    String::from_utf8(buffer).map_err(|e| {
        BencodeError::InvalidFormat(format!("Non-UTF8 characters in length/integer: {}", e))
    })
}
