use prost::DecodeError;
use prost::bytes::{Buf, BufMut};
use prost::encoding::{DecodeContext, WireType, bytes, skip_field};
use serde::Serialize;
use serde::de::DeserializeOwned;

pub(crate) fn encode_as_json_message<T: Serialize>(value: &T, buf: &mut impl BufMut) {
    let payload = serde_json::to_vec(value).expect("schema JSON message encode should succeed");
    bytes::encode(1, &payload, buf);
}

pub(crate) fn merge_from_json_message<T: DeserializeOwned>(
    value: &mut T,
    tag: u32,
    wire_type: WireType,
    buf: &mut impl Buf,
    ctx: DecodeContext,
) -> Result<(), DecodeError> {
    match tag {
        1 => {
            let mut payload = Vec::new();
            bytes::merge(wire_type, &mut payload, buf, ctx)?;
            #[allow(deprecated)]
            let decode_error =
                |error| DecodeError::new(format!("schema JSON message decode failed: {error}"));
            *value = serde_json::from_slice(&payload).map_err(decode_error)?;
            Ok(())
        }
        _ => skip_field(wire_type, tag, buf, ctx),
    }
}

pub(crate) fn json_message_encoded_len<T: Serialize>(value: &T) -> usize {
    let payload = serde_json::to_vec(value).expect("schema JSON message length should succeed");
    bytes::encoded_len(1, &payload)
}
