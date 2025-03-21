use bytes::{Buf, BufMut, BytesMut};

use super::{FromWireError, FromWireResult};
use crate::borrowed_bytesmut::BorrowedBytesMut;

/// Encoding Request in lightway-core
///
/// Frame size is fixed at 32 bytes, with 31 bytes reserved for future use.
#[derive(PartialEq, Debug)]
pub(crate) struct EncodingRequest {
    pub(crate) enable: bool,
}

impl EncodingRequest {
    /// Wire Size in bytes
    const WIRE_SIZE: usize = 32;

    pub(crate) fn try_from_wire(buf: &mut BorrowedBytesMut) -> FromWireResult<Self> {
        if buf.len() < Self::WIRE_SIZE {
            return Err(FromWireError::InsufficientData);
        };

        let enable = buf.get_u8();
        buf.advance(Self::WIRE_SIZE - 1); // Skip reserved bytes

        match enable {
            0 => Ok(Self { enable: false }),
            1 => Ok(Self { enable: true }),
            _ => Err(FromWireError::InvalidBool),
        }
    }

    pub(crate) fn append_to_wire(&self, buf: &mut BytesMut) {
        buf.reserve(Self::WIRE_SIZE);

        let encoding_enabled = if self.enable { 1 } else { 0 };

        buf.put_u8(encoding_enabled);
        buf.put_bytes(0, Self::WIRE_SIZE - 1); // Add reserved bytes
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::borrowed_bytesmut::ImmutableBytesMut;

    use test_case::test_case;

    #[test]
    fn from_wire_enabled() {
        let mut buf = ImmutableBytesMut::from(&b"\x01uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu"[..]);
        let mut buf = buf.as_borrowed_bytesmut();

        let wire = EncodingRequest::try_from_wire(&mut buf).expect("enabled");
        assert!(wire.enable);
    }

    #[test]
    fn from_wire_disabled() {
        let mut buf = ImmutableBytesMut::from(&b"\x00uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu"[..]);
        let mut buf = buf.as_borrowed_bytesmut();

        let wire = EncodingRequest::try_from_wire(&mut buf).expect("disabled");
        assert!(!wire.enable);
    }

    #[test]
    fn try_from_wire_too_short() {
        let mut buf = ImmutableBytesMut::from(&[0u8; EncodingRequest::WIRE_SIZE - 1][..]);
        let mut buf = buf.as_borrowed_bytesmut();
        assert!(matches!(
            EncodingRequest::try_from_wire(&mut buf).err().unwrap(),
            FromWireError::InsufficientData
        ));
    }

    #[test_case(EncodingRequest{enable: true} => b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00".to_vec(); "enable")]
    #[test_case(EncodingRequest{enable: false} => b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00".to_vec(); "disable")]
    fn test_append_to_wire(te: EncodingRequest) -> Vec<u8> {
        let mut buf = BytesMut::new();
        te.append_to_wire(&mut buf);
        buf.to_vec()
    }
}
