use std::ops::Deref;

use bytes::{Buf, BytesMut};

use crate::{RespDecode, RespEncode, RespError};

use super::{
    calc_total_length, extract_fixed_data, frame::RespFrame, parse_length, BUF_CAP, CRLF_LEN,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RespArray(pub(crate) Vec<RespFrame>);

const NULL_RESP_ARRAY: &[u8] = b"*-1\r\n";

impl RespEncode for RespArray {
    fn encode(self) -> Vec<u8> {
        if self.is_null() {
            // 如果是空数组，返回对应的编码
            NULL_RESP_ARRAY.to_vec()
        } else {
            let mut buf = Vec::with_capacity(BUF_CAP);
            buf.extend_from_slice(&format!("*{}\r\n", self.0.len()).into_bytes());

            for item in self.0 {
                buf.extend_from_slice(&item.encode());
            }
            buf
        }
    }
}

impl RespDecode for RespArray {
    const PREFIX: &'static str = "*";
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        if buf.starts_with(NULL_RESP_ARRAY) {
            // 如果是空数组的编码，直接返回空数组
            extract_fixed_data(buf, std::str::from_utf8(NULL_RESP_ARRAY)?, "NullArray")?;
            Ok(RespArray::null())
        } else {
            let (end, len) = parse_length(buf, Self::PREFIX)?;
            let total_len = calc_total_length(buf, end, len, Self::PREFIX)?;

            if buf.len() < total_len {
                return Err(RespError::NotComplete);
            }

            buf.advance(end + CRLF_LEN);

            let mut frames = Vec::with_capacity(len);
            for _ in 0..len {
                frames.push(RespFrame::decode(buf)?);
            }

            Ok(RespArray::new(frames))
        }
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        if buf.starts_with(NULL_RESP_ARRAY) {
            // 如果是空数组的编码，返回对应的长度
            Ok(NULL_RESP_ARRAY.len())
        } else {
            let (end, len) = parse_length(buf, Self::PREFIX)?;
            calc_total_length(buf, end, len, Self::PREFIX)
        }
    }
}

impl RespArray {
    pub fn new(s: impl Into<Vec<RespFrame>>) -> Self {
        RespArray(s.into())
    }

    pub fn null() -> Self {
        RespArray(Vec::new())
    }

    pub fn is_null(&self) -> bool {
        self.0.is_empty()
    }
}

impl Deref for RespArray {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BulkString;
    use anyhow::Result;

    #[test]
    fn test_array_encode() {
        let s: RespFrame = RespArray::new(vec![
            BulkString::new("set".to_string()).into(),
            BulkString::new("hello".to_string()).into(),
            BulkString::new("world".to_string()).into(),
        ])
        .into();

        assert_eq!(
            &s.encode(),
            b"*3\r\n$3\r\nset\r\n$5\r\nhello\r\n$5\r\nworld\r\n"
        );
    }

    #[test]
    fn test_null_array_encode() {
        let s: RespFrame = RespArray::null().into();
        assert_eq!(s.encode(), b"*-1\r\n");
    }

    #[test]
    fn test_null_array_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*-1\r\n");

        let frame = RespArray::decode(&mut buf)?;
        assert_eq!(frame, RespArray::null());

        Ok(())
    }

    #[test]
    fn test_array_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*2\r\n$3\r\nset\r\n$5\r\nhello\r\n");

        let frame = RespArray::decode(&mut buf)?;
        assert_eq!(frame, RespArray::new([b"set".into(), b"hello".into()]));

        buf.extend_from_slice(b"*2\r\n$3\r\nset\r\n");
        let ret = RespArray::decode(&mut buf);
        assert_eq!(ret.unwrap_err(), RespError::NotComplete);

        buf.extend_from_slice(b"$5\r\nhello\r\n");
        let frame = RespArray::decode(&mut buf)?;
        assert_eq!(frame, RespArray::new([b"set".into(), b"hello".into()]));

        Ok(())
    }
}
