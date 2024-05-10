use crate::cmd::{RESP_INT_0, RESP_INT_1};
use crate::RespFrame;
use dashmap::mapref::entry::Entry;
use dashmap::{DashMap, DashSet};
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Backend(Arc<BackendInner>);

#[derive(Debug)]
pub struct BackendInner {
    map: DashMap<String, RespFrame>,
    hmap: DashMap<String, DashMap<String, RespFrame>>,
}

impl Deref for Backend {
    type Target = BackendInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Backend {
    fn default() -> Self {
        Self(Arc::new(BackendInner::default()))
    }
}

impl Default for BackendInner {
    fn default() -> Self {
        Self {
            map: DashMap::new(),
            hmap: DashMap::new(),
        }
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

    pub fn sadd(&self, key: String, value: RespFrame) -> RespFrame {
        match self.map.entry(key) {
            Entry::Occupied(mut entry) => {
                if *entry.get() == value {
                    RESP_INT_0.clone() // 值已存在，不做改变
                } else {
                    entry.insert(value); // 更新为新值
                    RESP_INT_1.clone() // 表示添加了新元素
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(value); // 插入新值
                RESP_INT_1.clone() // 表示添加了新元素
            }
        }
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

    pub fn hmget(&self, key: &str, fields: Vec<String>) -> Option<DashMap<String, RespFrame>> {
        let field_set: DashSet<String> = fields.into_iter().collect();

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

    pub fn hmget1<I, T>(&self, key: &str, fields: I) -> Option<DashMap<String, RespFrame>>
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
