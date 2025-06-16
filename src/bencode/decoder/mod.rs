use super::BencodeError;
use super::BencodeResult;
use super::BencodeValue;

use std::collections::HashMap;
use std::io::{self, Read};
use tracing::instrument;

/// Decodes a bencode string from the input stream.
///
/// This function reads a bencode string in the format `<length>:<data>` where:
/// - `<length>` is a decimal number indicating the length of the string
/// - `<data>` is the actual string data of the specified length
///
/// # Arguments
/// * `reader` - A peekable iterator over the bytes of the input stream
///
/// # Returns
/// * `Result<Vec<u8>>` - The decoded string as a byte vector, or an error if:
///   - The length prefix is invalid or missing
///   - The input ends unexpectedly
///   - An I/O error occurs
///
/// # Example
/// For input "5:hello", this function will return a Vec<u8> containing [104, 101, 108, 108, 111]

#[instrument(skip(reader), level = "trace")]
pub fn decode_string<R: Read>(
    reader: &mut std::iter::Peekable<io::Bytes<R>>,
) -> BencodeResult<Vec<u8>> {
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

#[instrument(skip(reader), level = "trace")]
pub fn read_until<R: Read>(
    reader: &mut std::iter::Peekable<io::Bytes<R>>,
    delimiter: u8,
) -> BencodeResult<String> {
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

/// Decodes a bencode integer from the input stream.
///
/// This function reads a bencode integer in the format `i<number>e` where:
/// - `i` is the literal character 'i' marking the start of an integer
/// - `<number>` is the actual integer value
/// - `e` is the literal character 'e' marking the end of the integer
///
/// The function performs several validations:
/// - Ensures the integer starts with 'i'
/// - Rejects leading zeros (except for single '0')
/// - Rejects "-0" as invalid
/// - Rejects empty integers
///
/// # Arguments
/// * `reader` - A peekable iterator over the bytes of the input stream
///
/// # Returns
/// * `Result<i64>` - The decoded integer value, or an error if:
///   - The format is invalid
///   - The integer value is invalid
///   - The input ends unexpectedly
///   - An I/O error occurs
///
/// # Example
/// For input "i42e", this function will return Ok(42)

#[instrument(skip(reader), level = "trace")]
pub fn decode_integer<R: Read>(
    reader: &mut std::iter::Peekable<io::Bytes<R>>,
) -> BencodeResult<i64> {
    let first_byte = reader
        .next()
        .ok_or(BencodeError::UnexpectedEOI)?
        .map_err(|e| BencodeError::Io(e.kind().into()))?;

    if first_byte != b'i' {
        return Err(BencodeError::InvalidFormat(
            "Integer must start with 'i'".to_string(),
        ));
    }

    let num_str = read_until(reader, b'e')?;

    if num_str.len() > 1 && num_str.starts_with('0') {
        return Err(BencodeError::InvalidInteger);
    }

    if num_str == "-0" {
        return Err(BencodeError::InvalidInteger);
    }

    if num_str.is_empty() {
        return Err(BencodeError::InvalidInteger);
    }

    num_str
        .parse::<i64>()
        .map_err(|_| BencodeError::InvalidInteger)
}

/// Decodes a bencode list from the input stream.
///
/// This function reads a bencode list in the format `l<items>e` where:
/// - `l` is the literal character 'l' marking the start of a list
/// - `<items>` is a sequence of bencode values (integers, strings, lists, or dictionaries)
/// - `e` is the literal character 'e' marking the end of the list
///
/// The function recursively decodes each item in the list using `decode_next()`.
///
/// # Arguments
/// * `reader` - A peekable iterator over the bytes of the input stream
///
/// # Returns
/// * `Result<Vec<BencodeValue>>` - A vector of decoded bencode values, or an error if:
///   - The format is invalid
///   - Any item in the list fails to decode
///   - The input ends unexpectedly
///   - An I/O error occurs
///
/// # Example
/// For input "li42ei-1ee", this function will return Ok(vec![Integer(42), Integer(-1)])

#[instrument(skip(reader), level = "trace")]
fn decode_list<R: Read>(
    reader: &mut std::iter::Peekable<io::Bytes<R>>,
) -> BencodeResult<Vec<BencodeValue>> {
    let first_byte = reader
        .next()
        .ok_or(BencodeError::UnexpectedEOI)?
        .map_err(|e| BencodeError::Io(e.kind().into()))?;
    if first_byte != b'l' {
        return Err(BencodeError::InvalidFormat(
            "List must start with 'l'".to_string(),
        ));
    }

    let mut list = Vec::new();

    loop {
        let &current_byte = reader
            .peek()
            .ok_or(BencodeError::UnexpectedEOI)?
            .as_ref()
            .map_err(|e| BencodeError::Io(e.kind().into()))?;

        if current_byte == b'e' {
            reader
                .next()
                .ok_or(BencodeError::UnexpectedEOI)?
                .map_err(|e| BencodeError::Io(e.kind().into()))?;
            break;
        }

        let item = decode_next(reader)?;
        list.push(item);
    }

    Ok(list)
}

/// Decodes a bencode dictionary from the input stream.
///
/// # Arguments
/// * `reader` - A peekable iterator over the bytes of the input stream
///
/// # Returns
/// * `Result<HashMap<Vec<u8>, BencodeValue>>` - A hashmap containing the decoded key-value pairs, or an error if:
///   - The format is invalid (doesn't start with 'd')
///   - Any key or value fails to decode
///   - The input ends unexpectedly
///   - An I/O error occurs
///
/// # Example
/// For input "d3:keyi42ee", this function will return Ok({ "key" => Integer(42) })
///
/// # Format
/// Dictionaries in bencode format start with 'd' and end with 'e'. Keys must be strings,
/// and values can be any valid bencode value. Keys must be sorted in lexicographical order.
#[instrument(skip(reader), level = "trace")]
fn decode_dict<R: Read>(
    reader: &mut std::iter::Peekable<io::Bytes<R>>,
) -> BencodeResult<HashMap<Vec<u8>, BencodeValue>> {
    let first_byte = reader
        .next()
        .ok_or(BencodeError::UnexpectedEOI)?
        .map_err(|e| BencodeError::Io(e.kind().into()))?;

    if first_byte != b'd' {
        return Err(BencodeError::InvalidFormat(
            "Dictionary must start with 'd'".to_string(),
        ));
    }

    let mut dict = HashMap::new();

    loop {
        let &current_byte = reader
            .peek()
            .ok_or(BencodeError::UnexpectedEOI)?
            .as_ref()
            .map_err(|e| BencodeError::Io(e.kind().into()))?;

        if current_byte == b'e' {
            reader
                .next()
                .ok_or(BencodeError::UnexpectedEOI)?
                .map_err(|e| BencodeError::Io(e.kind().into()))?;
            break;
        }

        let key = decode_string(reader)?;

        let value = decode_next(reader)?;

        dict.insert(key, value);
    }

    Ok(dict)
}

#[instrument(skip(reader), level = "trace")]
fn decode_next<R: Read>(
    reader: &mut std::iter::Peekable<io::Bytes<R>>,
) -> BencodeResult<BencodeValue> {
    let &first_byte = reader
        .peek()
        .ok_or(BencodeError::UnexpectedEOI)?
        .as_ref()
        .map_err(|e| BencodeError::Io(e.kind().into()))?;

    match first_byte {
        b'0'..=b'9' => decode_string(reader).map(BencodeValue::String),
        b'i' => decode_integer(reader).map(BencodeValue::Integer),
        b'l' => decode_list(reader).map(BencodeValue::List),
        b'd' => decode_dict(reader).map(BencodeValue::Dict),
        _ => Err(BencodeError::InvalidFormat(format!(
            "Unexpected character: {}",
            first_byte as char
        ))),
    }
}
