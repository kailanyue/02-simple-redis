use crate::{backend::Backend, RespArray, RespFrame, RespNull};

use super::{
    extract_args, validate_command, CommandError, CommandExecutor, Get, SAdd, Set, SisMember,
    RESP_INT_0, RESP_INT_1, RESP_OK,
};

impl CommandExecutor for Get {
    fn execute(self, backend: &Backend) -> RespFrame {
        match backend.get(&self.key) {
            Some(value) => value,
            None => RespFrame::Null(RespNull),
        }
    }
}

impl CommandExecutor for Set {
    fn execute(self, backend: &Backend) -> RespFrame {
        backend.set(self.key, self.value);
        RESP_OK.clone()
    }
}

impl CommandExecutor for SAdd {
    fn execute(self, backend: &Backend) -> RespFrame {
        backend.sadd(self.key, self.value)
    }
}

impl CommandExecutor for SisMember {
    fn execute(self, backend: &Backend) -> RespFrame {
        backend.get(&self.key).map_or(RESP_INT_0.clone(), |value| {
            if value == self.value {
                RESP_INT_1.clone()
            } else {
                RESP_INT_0.clone()
            }
        })
    }
}

impl TryFrom<RespArray> for Get {
    type Error = CommandError;
    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["get"], 1)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match args.next() {
            Some(RespFrame::BulkString(key)) => Ok(Get {
                key: String::from_utf8(key.0)?,
            }),
            _ => Err(CommandError::InvalidArgument("Invalid key".to_string())),
        }
    }
}

impl TryFrom<RespArray> for Set {
    type Error = CommandError;
    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["set"], 2)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match (args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(value)) => Ok(Set {
                key: String::from_utf8(key.0)?,
                value,
            }),
            _ => Err(CommandError::InvalidArgument(
                "Invalid key or value".to_string(),
            )),
        }
    }
}

impl TryFrom<RespArray> for SAdd {
    type Error = CommandError;
    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["sadd"], 2)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match (args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(value)) => Ok(SAdd {
                key: String::from_utf8(key.0)?,
                value,
            }),
            _ => Err(CommandError::InvalidArgument(
                "Invalid key or value".to_string(),
            )),
        }
    }
}

impl TryFrom<RespArray> for SisMember {
    type Error = CommandError;
    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["sismember"], 2)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match (args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(value)) => Ok(SisMember {
                key: String::from_utf8(key.0)?,
                value,
            }),
            _ => Err(CommandError::InvalidArgument(
                "Invalid key or value".to_string(),
            )),
        }
    }
}

// // 通用函数，用于验证命令并提取参数
// fn extract_and_validate_args(
//     value: RespArray,
//     command: &'static str,
//     expected_args: usize,
// ) -> Result<(String, Option<RespFrame>), CommandError> {
//     validate_command(&value, &[command], expected_args)?;

//     let mut args = extract_args(value, expected_args)?.into_iter();
//     let key = match args.next() {
//         Some(RespFrame::BulkString(key)) => String::from_utf8(key.0)?,
//         _ => return Err(CommandError::InvalidArgument("Invalid key".to_string())),
//     };

//     let value = args.next();
//     Ok((key, value))
// }

// // Get命令的TryFrom实现
// impl TryFrom<RespArray> for Get {
//     type Error = CommandError;
//     fn try_from(value: RespArray) -> Result<Self, Self::Error> {
//         let (key, _) = extract_and_validate_args(value, "get", 1)?;
//         Ok(Get { key })
//     }
// }

// // Set命令的TryFrom实现
// impl TryFrom<RespArray> for Set {
//     type Error = CommandError;
//     fn try_from(value: RespArray) -> Result<Self, Self::Error> {
//         let (key, value) = extract_and_validate_args(value, "set", 2)?;
//         match value {
//             Some(value) => Ok(Set { key, value }),
//             _ => Err(CommandError::InvalidArgument("Invalid value".to_string())),
//         }
//     }
// }

// // SAdd命令的TryFrom实现
// impl TryFrom<RespArray> for SAdd {
//     type Error = CommandError;
//     fn try_from(value: RespArray) -> Result<Self, Self::Error> {
//         let (key, value) = extract_and_validate_args(value, "sadd", 2)?;
//         match value {
//             Some(value) => Ok(SAdd { key, value }),
//             _ => Err(CommandError::InvalidArgument("Invalid value".to_string())),
//         }
//     }
// }

// // SisMember命令的TryFrom实现
// impl TryFrom<RespArray> for SisMember {
//     type Error = CommandError;
//     fn try_from(value: RespArray) -> Result<Self, Self::Error> {
//         let (key, value) = extract_and_validate_args(value, "sismember", 2)?;
//         match value {
//             Some(value) => Ok(SisMember { key, value }),
//             _ => Err(CommandError::InvalidArgument("Invalid value".to_string())),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use crate::RespDecode;
    use anyhow::Result;
    use bytes::BytesMut;

    use super::*;

    #[test]
    fn test_get_from_resp_array() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*2\r\n$3\r\nget\r\n$5\r\nhello\r\n");

        let frame = RespArray::decode(&mut buf)?;
        let result: Get = frame.try_into()?;

        assert_eq!(result.key, "hello");

        Ok(())
    }

    #[test]
    fn test_set_from_resp_array() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*3\r\n$3\r\nset\r\n$5\r\nhello\r\n$5\r\nworld\r\n");

        let frame = RespArray::decode(&mut buf)?;
        let result: Set = frame.try_into()?;

        assert_eq!(result.key, "hello");
        assert_eq!(result.value, RespFrame::BulkString(b"world".into()));

        Ok(())
    }

    #[test]
    fn test_set_get_command() -> Result<()> {
        let backend = Backend::new();
        let cmd = Set {
            key: "hello".to_string(),
            value: RespFrame::BulkString(b"world".into()),
        };
        let result = cmd.execute(&backend);
        assert_eq!(result, RESP_OK.clone());

        let cmd = Get {
            key: "hello".to_string(),
        };
        let result = cmd.execute(&backend);
        assert_eq!(result, RespFrame::BulkString(b"world".into()));

        Ok(())
    }
}
