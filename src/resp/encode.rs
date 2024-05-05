use crate::{
    BulkString, RespArray, RespEncode, RespMap, RespNull, RespNullArray, RespNullBulkString,
    RespSet, SimpleError, SimpleString,
};

const BUF_CAP: usize = 4096;

// - simple string: "+OK\r\n"
impl RespEncode for SimpleString {
    fn encode(self) -> Vec<u8> {
        format!("+{}\r\n", self.0).into_bytes()
    }
}

// - error: "-Error message\r\n"
impl RespEncode for SimpleError {
    fn encode(self) -> Vec<u8> {
        format!("-{}\r\n", self.0).into_bytes()
    }
}

// - integer: ":[<+|->]<value>\r\n"
impl RespEncode for i64 {
    fn encode(self) -> Vec<u8> {
        let sign = if self < 0 { "" } else { "+" };
        format!(":{}{}\r\n", sign, self).into_bytes()
    }
}

// - bulk string: "$<length>\r\n<data>\r\n"
impl RespEncode for BulkString {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.len() + 16);
        buf.extend_from_slice(&format!("${}\r\n", self.len()).into_bytes());
        buf.extend_from_slice(&self);
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

// - null bulk string: "$-1\r\n"
impl RespEncode for RespNullBulkString {
    fn encode(self) -> Vec<u8> {
        b"$-1\r\n".to_vec()
    }
}

// - array: "*<number-of-elements>\r\n<element-1>...<element-n>"
impl RespEncode for RespArray {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);
        buf.extend_from_slice(&format!("*{}\r\n", self.0.len()).into_bytes());

        for item in self.0 {
            buf.extend_from_slice(&item.encode());
        }
        buf
    }
}

// - null array: "*-1\r\n"
impl RespEncode for RespNullArray {
    fn encode(self) -> Vec<u8> {
        b"*-1\r\n".to_vec()
    }
}

// - null: "_\r\n"
impl RespEncode for RespNull {
    fn encode(self) -> Vec<u8> {
        b"_\r\n".to_vec()
    }
}

// - boolean: "#<t|f>\r\n"
impl RespEncode for bool {
    fn encode(self) -> Vec<u8> {
        if self {
            b"#t\r\n".to_vec()
        } else {
            b"#f\r\n".to_vec()
        }
    }
}

// - double: ",[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n"
impl RespEncode for f64 {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(32);
        let ret = if self.abs() > 1e+8 || self.abs() < 1e-8 {
            format!(",{:+e}\r\n", self)
        } else {
            let sign = if self < 0.0 { "" } else { "+" };
            format!(",{}{}\r\n", sign, self)
        };

        buf.extend_from_slice(&ret.into_bytes());
        buf
    }
}

// - map: "%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>"
// we only support string key which encode to SimpleString
impl RespEncode for RespMap {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);
        buf.extend_from_slice(&format!("%{}\r\n", self.0.len()).into_bytes());

        for (key, value) in self.0 {
            buf.extend_from_slice(&SimpleString::new(key).encode());
            buf.extend_from_slice(&value.encode());
        }
        buf
    }
}

// - set: "~<number-of-elements>\r\n<element-1>...<element-n>"
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

#[cfg(test)]
mod tests {
    use crate::RespFrame;

    use super::*;

    #[test]
    fn test_simple_string_encode() {
        let s: RespFrame = SimpleString::new("hello").into();
        assert_eq!(s.encode(), b"+hello\r\n");
    }

    #[test]
    fn test_simple_error_encode() {
        let s: RespFrame = SimpleError::new("Error message").into();
        assert_eq!(s.encode(), b"-Error message\r\n");
    }

    #[test]
    fn test_integer_encode() {
        let s: RespFrame = 123.into();
        assert_eq!(s.encode(), b":+123\r\n");

        let s: RespFrame = (-123).into();
        assert_eq!(s.encode(), b":-123\r\n");
    }

    #[test]
    fn test_bulk_string_encode() {
        let s: RespFrame = BulkString::new("hello").into();
        assert_eq!(s.encode(), b"$5\r\nhello\r\n");
    }

    #[test]
    fn test_null_bulk_string_encode() {
        let s: RespFrame = RespNullBulkString.into();
        assert_eq!(s.encode(), b"$-1\r\n");
    }

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
        let s: RespFrame = RespNullArray.into();
        assert_eq!(s.encode(), b"*-1\r\n");
    }

    #[test]
    fn test_null_encode() {
        let s: RespFrame = RespNull.into();
        assert_eq!(s.encode(), b"_\r\n");
    }

    #[test]
    fn test_boolean_encode() {
        let s: RespFrame = true.into();
        assert_eq!(s.encode(), b"#t\r\n");

        let s: RespFrame = false.into();
        assert_eq!(s.encode(), b"#f\r\n");
    }

    #[test]
    fn test_double_encode() {
        let s: RespFrame = 123.456.into();
        assert_eq!(s.encode(), b",+123.456\r\n");

        let s: RespFrame = (-123.456).into();
        assert_eq!(s.encode(), b",-123.456\r\n");

        let s: RespFrame = 1.23456e+8.into();
        assert_eq!(s.encode(), b",+1.23456e8\r\n");

        let s: RespFrame = (-1.23456e-9).into();
        assert_eq!(s.encode(), b",-1.23456e-9\r\n");
    }

    #[test]
    fn test_map_encode() {
        let mut map = RespMap::new();
        map.insert(
            "key".to_string(),
            BulkString::new("value".to_string()).into(),
        );
        map.insert("test".to_string(), (123.456).into());

        let frame: RespFrame = map.into();
        assert_eq!(
            &frame.encode(),
            b"%2\r\n+key\r\n$5\r\nvalue\r\n+test\r\n,+123.456\r\n"
        );

        // 因为 RespMap 底层使用的是 TreeMap 因此会对key进行排序，
        // 故 map1  encode 之后的顺序与插入的顺序不一致

        let mut map1 = RespMap::new();
        map1.insert(
            "key".to_string(),
            BulkString::new("value".to_string()).into(),
        );
        map1.insert("a".to_string(), (123.456).into());

        let frame1: RespFrame = map1.into();
        assert_eq!(
            &frame1.encode(),
            b"%2\r\n+a\r\n,+123.456\r\n+key\r\n$5\r\nvalue\r\n"
        );
    }

    #[test]
    fn test_set_encode() {
        let s: RespFrame = RespSet::new(vec![
            BulkString::new("hello".to_string()).into(),
            RespArray::new([1234.into(), true.into()]).into(),
        ])
        .into();

        assert_eq!(&s.encode(), b"~2\r\n$5\r\nhello\r\n*2\r\n:+1234\r\n#t\r\n")
    }
}
