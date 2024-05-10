use std::ops::Deref;

use bytes::{Buf, BytesMut};

use crate::{RespDecode, RespEncode, RespError, RespFrame};

use super::{calc_total_length, parse_length, BUF_CAP, CRLF_LEN};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RespSet(pub(crate) Vec<RespFrame>);

impl RespEncode for RespSet {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);
        buf.extend_from_slice(&format!("~{}\r\n", self.len()).into_bytes());

        for item in self.0 {
            buf.extend_from_slice(&item.encode());
        }
        buf
    }
}

impl RespDecode for RespSet {
    const PREFIX: &'static str = "~";
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;

        let total_len = calc_total_length(buf, end, len, Self::PREFIX)?;

        if buf.len() < total_len {
            return Err(RespError::NotComplete);
        }

        buf.advance(end + CRLF_LEN);

        let mut frames = Vec::new();
        for _ in 0..len {
            frames.push(RespFrame::decode(buf)?);
        }

        Ok(RespSet::new(frames))
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        calc_total_length(buf, end, len, Self::PREFIX)
    }
}

impl RespSet {
    pub fn new(s: impl Into<Vec<RespFrame>>) -> Self {
        RespSet(s.into())
    }
}

impl Deref for RespSet {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{resp::array::RespArray, BulkString};
    use anyhow::Result;

    #[test]
    fn test_set_encode() {
        let s: RespFrame = RespSet::new(vec![
            BulkString::new("hello".to_string()).into(),
            RespArray::new([1234.into(), true.into()]).into(),
        ])
        .into();

        assert_eq!(&s.encode(), b"~2\r\n$5\r\nhello\r\n*2\r\n:1234\r\n#t\r\n")
    }

    #[test]
    fn test_set_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"~2\r\n$3\r\nset\r\n$5\r\nhello\r\n");

        let frame = RespSet::decode(&mut buf)?;
        assert_eq!(
            frame,
            RespSet::new(vec![
                BulkString::new(b"set".to_vec()).into(),
                BulkString::new(b"hello".to_vec()).into()
            ])
        );

        Ok(())
    }
}
