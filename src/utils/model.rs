use axum::{async_trait};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::time::Duration;
use crate::utils::redis::RedisService;
use crate::utils::error::{Error, Result};

//缓存服务接口
const RDS_URI: &str = "redis://:lXei9FTDYm3SXmvfrbbYCMWmHAAzpaU5@redis-14362.c60.us-west-1-2.ec2.cloud.redislabs.com:14362";


#[async_trait]
pub trait ICacheService: Sync + Send {
    async fn set_string(&self, k: &str, v: &str) -> Result<String>;

    async fn get_string(&self, k: &str) -> Result<String>;

    async fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> Result<String>;

    async fn ttl(&self, k: &str) -> Result<i64>;

    async fn del(&self, k: &str) -> Result<i64>;
}

pub struct CacheService {
    pub inner: Box<dyn ICacheService>,
}

impl CacheService {
    pub fn new() -> Result<Self> {
        Ok(Self {
            inner: Box::new(RedisService::new(RDS_URI)),
        }) 
    }

    pub async fn set_string(&self, k: &str, v: &str) -> Result<String> {
        self.inner.set_string(k, v).await
    }

    pub async fn get_string(&self, k: &str) -> Result<String> {
        self.inner.get_string(k).await
    }

    pub async fn set_json<T>(&self, k: &str, v: &T) -> Result<String>
    where
        T: Serialize + Sync,
    {
        let data = serde_json::to_string(v);
        if data.is_err() {
            return Err(Error::from(format!("MemCacheService set_json fail:{}", data.err().unwrap())));
        }
        let data = self.set_string(k, data.unwrap().as_str()).await?;
        Ok(data)
    }

    pub async fn get_json<T>(&self, k: &str) -> Result<T>
    where
        T: DeserializeOwned + Sync,
    {
        let mut r = self.get_string(k).await?;
        if r.is_empty() {
            r = "null".to_string();
        }
        let data: serde_json::Result<T> = serde_json::from_str(r.as_str());
        if data.is_err() {
            return Err(Error::from(format!("MemCacheService GET fail:{}", data.err().unwrap())));
        }
        Ok(data.unwrap())
    }

    pub async fn set_string_ex(&self, k: &str, v: &str, ex: Option<Duration>) -> Result<String> {
        self.inner.set_string_ex(k, v, ex).await
    }

    pub async fn ttl(&self, k: &str) -> Result<i64> {
        self.inner.ttl(k).await
    }

    pub async fn del(&self, k: &str) -> Result<i64> {
        self.inner.del(k).await
    }
}
