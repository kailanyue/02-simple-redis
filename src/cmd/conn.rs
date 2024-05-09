// 实现 echo 和 ping 等连接相关的命令
use crate::{Backend, BulkString, RespArray, RespFrame, SimpleString};

use super::{extract_args, validate_command, CommandError, CommandExecutor, Echo, Ping};

const PING: &str = "ping";
const PONG: &str = "PONG";

impl CommandExecutor for Echo {
    fn execute(self, _: &Backend) -> RespFrame {
        BulkString::new(self.message).into()
    }
}

impl TryFrom<RespArray> for Echo {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["echo"], 1)?;
        let mut args = extract_args(value, 1)?.into_iter();

        match args.next() {
            Some(RespFrame::BulkString(key)) => Ok(Echo {
                message: String::from_utf8(key.0)?,
            }),
            _ => Err(CommandError::InvalidArgument(
                "Invalid argument".to_string(),
            )),
        }
    }
}

/*
    与官方实现一致，
    - 对 ping 命令使用 SimpleString
    - 对 ping arg 命令 使用BulkString

    127.0.0.1:6379> ping
    ->PONG
    127.0.0.1:6379> ping hello
    ->"hello"
*/
impl CommandExecutor for Ping {
    fn execute(self, _: &Backend) -> RespFrame {
        if self.message == PONG {
            SimpleString::new(self.message).into()
        } else {
            BulkString::new(self.message).into()
        }
    }
}

impl TryFrom<RespArray> for Ping {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        let command_len = value.len();

        match command_len {
            1 => {
                validate_command(&value, &[PING], command_len - 1)?;
                Ok(Ping {
                    message: PONG.to_string(),
                })
            }
            2 => {
                validate_command(&value, &[PING], command_len - 1)?;

                let mut args = extract_args(value, 1)?.into_iter();
                match args.next() {
                    Some(RespFrame::BulkString(key)) => Ok(Ping {
                        message: String::from_utf8(key.0)?,
                    }),
                    _ => Err(CommandError::InvalidArgument(
                        "Invalid argument".to_string(),
                    )),
                }
            }
            _ => Err(CommandError::InvalidArgument(
                "wrong number of arguments for 'ping' command".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::RespDecode;

    use super::*;
    use anyhow::Result;
    use bytes::BytesMut;

    #[test]
    fn test_echo_try_from() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*2\r\n$4\r\necho\r\n$5\r\nhello\r\n");

        let frame = RespArray::decode(&mut buf)?;
        let result: Echo = frame.try_into()?;

        assert_eq!(result.message, "hello");
        Ok(())
    }

    #[test]
    fn test_echo_command() -> Result<()> {
        let backend = Backend::new();
        let cmd = Echo {
            message: "hello".to_string(),
        };

        let result = cmd.execute(&backend);
        assert_eq!(result, BulkString::new("hello").into());

        Ok(())
    }
}
