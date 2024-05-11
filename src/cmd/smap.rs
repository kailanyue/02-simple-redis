use crate::{Backend, RespArray, RespFrame};

use super::{
    extract_args, validate_command, CommandError, CommandExecutor, SAdd, SisMember,
    TryIntoBulkString,
};

impl CommandExecutor for SAdd {
    fn execute(self, backend: &Backend) -> RespFrame {
        backend.sadd(self.key, self.values)
    }
}

impl CommandExecutor for SisMember {
    fn execute(self, backend: &Backend) -> RespFrame {
        backend.sismember(&self.key, self.value)
    }
}

// SAdd命令的TryFrom实现
impl TryFrom<RespArray> for SAdd {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        match value.len() > 2 {
            true => validate_command(&value, &["sadd"], value.len() - 1)?,
            false => {
                return Err(CommandError::InvalidArgument(
                    "wrong number of arguments for 'sadd' command".to_string(),
                ))
            }
        }

        let mut args = extract_args(value, 1)?.into_iter();

        let key = args
            .next()
            .ok_or_else(|| CommandError::InvalidArgument("Missing key".to_string()))?
            .try_into_bulk_string()?;

        let values = args
            .map(RespFrame::try_into_bulk_string)
            .collect::<Result<Vec<String>, Self::Error>>()?;

        Ok(SAdd { key, values })
    }
}

// SisMember命令的TryFrom实现
impl TryFrom<RespArray> for SisMember {
    type Error = CommandError;
    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["sismember"], 2)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match (args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(RespFrame::BulkString(field))) => {
                Ok(SisMember {
                    key: String::from_utf8(key.0)?,
                    value: String::from_utf8(field.0)?,
                })
            }
            _ => Err(CommandError::InvalidArgument(
                "Invalid key or field".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::{RESP_INT_0, RESP_INT_1, RESP_INT_2};
    use anyhow::Result;

    use super::*;

    #[test]
    fn test_sadd_one_value_command() -> Result<()> {
        let backend = Backend::new();
        let cmd = SAdd {
            key: "k1".to_string(),
            values: vec!["v1".to_string()],
        };
        let result = cmd.execute(&backend);
        assert_eq!(result, RESP_INT_1.clone());

        let cmd = SAdd {
            key: "k1".to_string(),
            values: vec!["v1".to_string()],
        };
        let result = cmd.execute(&backend);
        assert_eq!(result, RESP_INT_0.clone());

        let cmd = SAdd {
            key: "k1".to_string(),
            values: vec!["v2".to_string()],
        };
        let result = cmd.execute(&backend);
        assert_eq!(result, RESP_INT_1.clone());
        Ok(())
    }
    #[test]
    fn test_sadd_more_value_command() -> Result<()> {
        let backend = Backend::new();
        let cmd = SAdd {
            key: "k1".to_string(),
            values: vec!["v1".to_string(), "v2".to_string()],
        };
        let result = cmd.execute(&backend);
        assert_eq!(result, RESP_INT_2.clone());
        Ok(())
    }

    #[test]
    fn test_sismember_command() -> Result<()> {
        let backend = Backend::new();
        let cmd = SisMember {
            key: "k1".to_string(),
            value: "v1".to_string(),
        };
        let result = cmd.execute(&backend);
        assert_eq!(result, RESP_INT_0.clone());

        // sadd 添加数据
        let cmd = SAdd {
            key: "k1".to_string(),
            values: vec!["v1".to_string()],
        };
        cmd.execute(&backend);

        let cmd = SisMember {
            key: "k1".to_string(),
            value: "v1".to_string(),
        };
        let result = cmd.execute(&backend);
        assert_eq!(result, RESP_INT_1.clone());
        Ok(())
    }
}
