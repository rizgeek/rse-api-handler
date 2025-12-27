use redis::{Client, Commands};

use crate::application::ports::Cache;

pub struct RedisCache {
    client: redis::Client,
}

impl RedisCache {
    pub fn new(connection_str: &str) -> Self {
        let client = Client::open(connection_str).unwrap();
        Self { client }
    }

    fn get_connection(&self) -> redis::Connection {
        self.client.get_connection().unwrap()
    }
}

impl Cache for RedisCache  {
    fn get(&self, key: &str) -> Option<String> {
        let mut conn = self.get_connection();
        conn.get(key).ok()
    }

    fn set(&self, key: &str, value: &str) {
        let mut conn = self.get_connection();
        let _: () = conn.set(key, value).unwrap();
    }

    fn delete(&self, key: &str) {
        let mut conn = self.get_connection();
        let _: () = conn.del(key).unwrap();
        
    }
}