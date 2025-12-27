#[cfg(test)]
mod tests {
    use crate::application::ports::Cache;
    use crate::infrastructure::cache::redis::RedisCache;

    #[test]
    fn test_redis_crud() {
        let redis = RedisCache::new("redis://127.0.0.1:1111/");

        // SET
        redis.set("foo", "Bar");

        // GET
        let get = redis.get("foo");
        assert_eq!(get, Some("Bar".to_string()));

        // DELETE
        redis.delete("foo");
        let get_after_delete = redis.get("foo");
        assert!(get_after_delete.is_none());
    }

    #[test]
    fn test_get_nonexistent_key() {
        let redis = RedisCache::new("redis://127.0.0.1:1111/");
        let result = redis.get("nonexistent_key");
        assert!(result.is_none());
    }
}
