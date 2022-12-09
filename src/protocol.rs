// Jackson Coxson

#![warn(clippy::derive_partial_eq_without_eq)]

use prost::Message;

include!(concat!(env!("OUT_DIR"), "/_.rs"));

impl Request {
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf).unwrap();
        buf
    }
    pub fn b64(&self) -> String {
        base64::encode(self.serialize())
    }
}

impl Response {
    fn deserialize(bytes: &[u8]) -> Result<Response, prost::DecodeError> {
        Response::decode(bytes)
    }
}

impl TryFrom<String> for Response {
    type Error = prost::DecodeError;
    fn try_from(s: String) -> Result<Response, prost::DecodeError> {
        let bytes = match base64::decode(s) {
            Ok(bytes) => bytes,
            Err(e) => return Err(prost::DecodeError::new(e.to_string())),
        };
        Response::deserialize(&bytes)
    }
}
