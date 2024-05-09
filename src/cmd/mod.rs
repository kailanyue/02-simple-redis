use enum_dispatch::enum_dispatch;
use lazy_static::lazy_static;
use thiserror::Error;

use crate::{backend::Backend, RespArray, RespError, RespFrame, SimpleString};

mod conn;
mod hmap;
mod map;

lazy_static! {
    static ref RESP_OK: RespFrame = SimpleString::new("OK").into();
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("{0}")]
    RespError(#[from] RespError),
    #[error("Utf8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

#[enum_dispatch]
pub trait CommandExecutor {
    fn execute(self, backend: &Backend) -> RespFrame;
}

#[enum_dispatch(CommandExecutor)]
#[derive(Debug)]
pub enum Command {
    Get(Get),
    Set(Set),
    HGet(HGet),
    HSet(HSet),
    HGetAll(HGetAll),
    Echo(Echo),
    Ping(Ping),
    // unrecognized command
    Unrecognized(Unrecognized),
}

#[derive(Debug)]
pub struct Get {
    pub key: String,
}

#[derive(Debug)]
pub struct Set {
    pub key: String,
    pub value: RespFrame,
}

#[derive(Debug)]
pub struct HGet {
    pub key: String,
    pub field: String,
}

#[derive(Debug)]
pub struct HSet {
    pub key: String,
    pub field: String,
    pub value: RespFrame,
}

#[derive(Debug)]
pub struct HGetAll {
    pub key: String,
    sort: bool,
}

#[derive(Debug)]
pub struct Echo {
    pub message: String,
}

#[derive(Debug)]
pub struct Ping {
    pub message: String,
}

#[derive(Debug)]
pub struct Unrecognized;

impl TryFrom<RespFrame> for Command {
    type Error = CommandError;

    fn try_from(value: RespFrame) -> Result<Self, Self::Error> {
        match value {
            RespFrame::Array(array) => Command::try_from(array),
            _ => Err(CommandError::InvalidCommand(
                "Command must be an Array".to_string(),
            )),
        }
    }
}

impl TryFrom<RespArray> for Command {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        match value.first() {
            Some(RespFrame::BulkString(ref cmd)) => {
                match cmd.as_ref().to_ascii_lowercase().as_slice() {
                    b"get" => Ok(Get::try_from(value)?.into()),
                    b"set" => Ok(Set::try_from(value)?.into()),
                    b"hget" => Ok(HGet::try_from(value)?.into()),
                    b"hset" => Ok(HSet::try_from(value)?.into()),
                    b"hgetall" => Ok(HGetAll::try_from(value)?.into()),
                    b"echo" => Ok(Echo::try_from(value)?.into()),
                    b"ping" => Ok(Ping::try_from(value)?.into()),
                    _ => Ok(Unrecognized.into()),
                }
            }
            _ => Err(CommandError::InvalidCommand(
                "Command must have a BulkString as the first argument".to_string(),
            )),
        }
    }
}

impl CommandExecutor for Unrecognized {
    fn execute(self, _: &Backend) -> RespFrame {
        RESP_OK.clone()
    }
}

fn validate_command(
    value: &RespArray,
    names: &[&'static str],
    n_args: usize,
) -> Result<(), CommandError> {
    if value.len() != n_args + names.len() {
        return Err(CommandError::InvalidArgument(format!(
            "{} command must have exactly {} argument",
            names.join(" "),
            n_args
        )));
    }

    for (i, name) in names.iter().enumerate() {
        match value[i] {
            RespFrame::BulkString(ref cmd) => {
                if cmd.as_ref().to_ascii_lowercase() != name.as_bytes() {
                    return Err(CommandError::InvalidCommand(format!(
                        "Invalid command: expected {}, got {}",
                        name,
                        String::from_utf8_lossy(cmd.as_ref())
                    )));
                }
            }
            _ => {
                return Err(CommandError::InvalidCommand(
                    "Command must have a BulkString as the first argument".to_string(),
                ))
            }
        }
    }
    Ok(())
}

fn extract_args(value: RespArray, start: usize) -> Result<Vec<RespFrame>, CommandError> {
    Ok(value.0.into_iter().skip(start).collect::<Vec<RespFrame>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_command_with_two_args() {
        let value = RespArray(vec![
            RespFrame::BulkString(b"set".into()),
            RespFrame::BulkString(b"key".into()),
            RespFrame::BulkString(b"value".into()),
        ]);
        assert!(validate_command(&value, &["set"], 2).is_ok());

        let value = RespArray(vec![
            RespFrame::BulkString(b"set".into()),
            RespFrame::BulkString(b"key".into()),
        ]);
        assert!(validate_command(&value, &["set"], 2).is_err());

        let value = RespArray(vec![
            RespFrame::BulkString(b"set".into()),
            RespFrame::BulkString(b"key".into()),
            RespFrame::BulkString(b"value".into()),
            RespFrame::BulkString(b"extra".into()),
        ]);
        assert!(validate_command(&value, &["set"], 2).is_err());
    }

    #[test]
    fn test_validate_command_with_one_arg() {
        let value = RespArray(vec![
            RespFrame::BulkString(b"ping".into()),
            RespFrame::BulkString(b"extra".into()),
        ]);
        assert!(validate_command(&value, &["ping"], 1).is_ok());

        let value = RespArray(vec![RespFrame::BulkString(b"ping".into())]);
        assert!(validate_command(&value, &["ping", "pong"], 1).is_err());
    }

    #[test]
    fn test_extract_args() {
        let value = RespArray(vec![
            RespFrame::BulkString(b"set".into()),
            RespFrame::BulkString(b"key".into()),
            RespFrame::BulkString(b"value".into()),
        ]);

        assert_eq!(
            extract_args(value, 1).unwrap(),
            vec![
                RespFrame::BulkString(b"key".into()),
                RespFrame::BulkString(b"value".into())
            ]
        );
    }
}
