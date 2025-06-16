use super::BencodeError;
use super::BencodeResult;
use super::BencodeValue;
use std::collections::HashMap;
use std::io::Write;
use tracing::instrument;

/// Encodes a byte slice as a bencode string.
///
/// This function writes the length of the byte slice followed by a colon and then the bytes themselves.
///
/// # Example Output
/// For input: b"hello"
/// Output: "5:hello"
#[instrument(skip(writer, s), level = "trace")]
fn encode_string<W: Write>(writer: &mut W, s: &[u8]) -> BencodeResult<()> {
    write!(writer, "{}:", s.len())?;
    writer.write_all(s)?;
    Ok(())
}

/// Encodes an integer as a bencode integer.
///
/// This function writes the integer prefixed with 'i' and suffixed with 'e'.
///
/// # Example Output
/// For input: 42
/// Output: "i42e"
#[instrument(skip(writer), level = "trace")]
fn encode_integer<W: Write>(writer: &mut W, i: i64) -> BencodeResult<()> {
    write!(writer, "i{}e", i)?;
    Ok(())
}

/// Encodes a slice of BencodeValue as a bencode list.
///
/// This function writes 'l', then encodes each item in the slice, and finally writes 'e'.
///
/// # Example Output
/// For input: [1, "hello"]
/// Output: "li1e5:helloe"
#[instrument(skip(writer, list), level = "trace")]
fn encode_list<W: Write>(writer: &mut W, list: &[BencodeValue]) -> BencodeResult<()> {
    writer.write_all(b"l")?;
    for item in list {
        encode_value(writer, item)?;
    }
    writer.write_all(b"e")?;
    Ok(())
}

/// Encodes a HashMap of byte slices to BencodeValue as a bencode dictionary.
///
/// This function writes 'd', then for each key-value pair (in sorted key order), encodes the key as a string,
/// then encodes the value, and finally writes 'e'.
///
/// # Example Output
/// For input: {"key": 42}
/// Output: "d3:keyi42ee"
#[instrument(skip(writer, dict), level = "trace")]
fn encode_dict<W: Write>(
    writer: &mut W,
    dict: &HashMap<Vec<u8>, BencodeValue>,
) -> BencodeResult<()> {
    writer.write_all(b"d")?;
    let mut keys: Vec<&Vec<u8>> = dict.keys().collect();
    keys.sort_unstable();
    for key in keys {
        encode_string(writer, key)?;
        let value = dict
            .get(key)
            .ok_or_else(|| BencodeError::InvalidFormat("Missing dict value for key".to_string()))?;
        encode_value(writer, value)?;
    }
    writer.write_all(b"e")?;
    Ok(())
}

/// Encodes a BencodeValue into its bencode representation.
///
/// This function matches on the variant of BencodeValue and calls the appropriate encoding function.
///
/// # Example Output
/// For input: BencodeValue::String(b"hello")
/// Output: "5:hello"
///
/// For input: BencodeValue::Integer(42)
/// Output: "i42e"
///
/// For input: BencodeValue::List([1, "hello"])
/// Output: "li1e5:helloe"
///
/// For input: BencodeValue::Dicts({"key": 42})
/// Output: "d3:keyi42ee"
#[instrument(skip(writer), level = "trace")]
fn encode_value<W: Write>(writer: &mut W, value: &BencodeValue) -> BencodeResult<()> {
    match value {
        BencodeValue::String(s) => encode_string(writer, s),
        BencodeValue::Integer(i) => encode_integer(writer, *i),
        BencodeValue::List(list) => encode_list(writer, list),
        BencodeValue::Dicts(dict) => encode_dict(writer, dict),
    }
}

/// Public function to encode a BencodeValue into its bencode representation.
///
/// This function is a wrapper around encode_value.
///
/// # Example Output
/// For input: BencodeValue::Integer(42)
/// Output: "i42e"
#[instrument(skip(writer), level = "debug")]
pub fn encode<W: Write>(writer: &mut W, value: &BencodeValue) -> BencodeResult<()> {
    encode_value(writer, value)
}
