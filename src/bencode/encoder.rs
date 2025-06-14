use super::BencodeError;
use super::BencodeValue;
use super::Result;
use std::collections::HashMap;
use std::io::Write;
use tracing::instrument;


#[instrument(skip(writer, s), level = "trace")]
fn encode_string<W: Write>(writer: &mut W, s: &[u8]) -> Result<()> {
    write!(writer, "{}:", s.len())?;
    writer.write_all(s)?;
    Ok(())
}

#[instrument(skip(writer), level = "trace")]
fn encode_integer<W: Write>(writer: &mut W, i: i64) -> Result<()> {
    write!(writer, "i{}e", i)?;
    Ok(())
}

#[instrument(skip(writer, list), level = "trace")]
fn encode_list<W: Write>(writer: &mut W, list: &[BencodeValue]) -> Result<()> {
    writer.write_all(b"l")?;
    for item in list {
        encode_value(writer, item)?;
    }
    writer.write_all(b"e")?;
    Ok(())
}

#[instrument(skip(writer, dict), level = "trace")]
fn encode_dict<W: Write>(writer: &mut W, dict: &HashMap<Vec<u8>, BencodeValue>) -> Result<()> {
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

#[instrument(skip(writer), level = "trace")]
fn encode_value<W: Write>(writer: &mut W, value: &BencodeValue) -> Result<()> {
    match value {
        BencodeValue::String(s) => encode_string(writer, s),
        BencodeValue::Integer(i) => encode_integer(writer, *i),
        BencodeValue::List(list) => encode_list(writer, list),
        BencodeValue::Dicts(dict) => encode_dict(writer, dict),
    }
}

#[instrument(skip(writer), level = "debug")]
pub fn encode<W: Write>(writer: &mut W, value: &BencodeValue) -> Result<()> {
    encode_value(writer, value)
}
