use crate::types::BoltValue;
use bytes::BytesMut;

pub struct BoltDecoder {}

impl tokio::codec::Decoder for BoltDecoder {
    type Item = BoltValue;
    type Error = tokio::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        Ok(None)
    }
}
