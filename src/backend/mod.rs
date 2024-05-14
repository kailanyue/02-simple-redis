use crate::cmd::{RESP_INT_0, RESP_INT_1};
use crate::RespFrame;
use dashmap::{DashMap, DashSet};
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
pub struct Backend(Arc<BackendInner>);

#[derive(Debug, Default)]
pub struct BackendInner {
    map: DashMap<String, RespFrame>,
    hmap: DashMap<String, DashMap<String, RespFrame>>,
    smap: DashMap<String, DashSet<String>>,
}

impl Deref for Backend {
    type Target = BackendInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Backend {
    pub fn new() -> Self {
        Self(Arc::new(BackendInner::default()))
    }

    pub fn get(&self, key: &str) -> Option<RespFrame> {
        self.map.get(key).map(|v| v.value().clone())
    }

    pub fn set(&self, key: String, value: RespFrame) {
        self.map.insert(key, value);
    }

    pub fn sadd<I, T>(&self, key: T, values: I) -> RespFrame
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let mut count = 0;
        let set = self.smap.entry(key.into()).or_default();

        for value in values {
            if set.insert(value.into()) {
                count += 1;
            }
        }

        RespFrame::Integer(count.into())
    }
    pub fn sismember(&self, key: &str, value: &str) -> RespFrame {
        self.smap
            .get(key)
            .and_then(|v| v.get(value).map(|_| RESP_INT_1.clone()))
            .unwrap_or_else(|| RESP_INT_0.clone())
    }

    pub fn hget(&self, key: &str, field: &str) -> Option<RespFrame> {
        // and_then 如何 key 不存在时返回 None，否则就执行对应的方法
        self.hmap
            .get(key)
            .and_then(|v| v.get(field).map(|v| v.value().clone()))
    }

    pub fn hset(&self, key: String, field: String, value: RespFrame) {
        let hmap = self.hmap.entry(key).or_default();
        hmap.insert(field, value);
    }

    pub fn hgetall(&self, key: &str) -> Option<DashMap<String, RespFrame>> {
        self.hmap.get(key).map(|v| v.clone())
    }

    pub fn hmget<I, T>(&self, key: &str, fields: I) -> Option<DashMap<String, RespFrame>>
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let field_set: DashSet<String> = fields.into_iter().map(Into::into).collect();

        self.hmap.get(key).map(|value| {
            let result = DashMap::new();

            value.iter().for_each(|entry| {
                let key = entry.key();
                if field_set.contains(key) {
                    result.insert(key.clone(), entry.value().clone());
                }
            });
            result
        })
    }
}
