use std::collections::HashMap;
use std::sync::Arc;

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use tokio::sync::RwLock;
use tracing::trace;
use uuid::Uuid;

use crate::configuration::CacheSettings;
use crate::domain::modifier::ModifierTarget;
use crate::domain::player;
use crate::domain::player::resource::ResourceType;
use crate::{Error, ErrorKind};

#[derive(Debug, Clone)]
pub struct CacheEntry {
    /// The calculated total modifier value
    pub total_multiplier: BigDecimal,
    /// When this cache entry expires (None for permanent modifiers)
    pub expires_at: Option<DateTime<Utc>>,
    /// Version for optimistic locking
    pub version: u64,
    /// Timestamp of last update
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CacheKey {
    pub player_id: player::PlayerKey,
    pub target_type: ModifierTarget,
    pub target_resource: Option<ResourceType>,
}

pub struct ModifierCache {
    /// Main cache storage using RwLock for concurrent access
    cache: Arc<RwLock<HashMap<CacheKey, CacheEntry>>>,
    /// Default TTL for cache entries
    default_ttl: chrono::Duration,
    /// Maximum entries per player to prevent memory issues
    max_entries_per_user: usize,
}

impl ModifierCache {
    pub fn new(default_ttl: chrono::Duration, max_entries_per_user: usize) -> Self {
        trace!(
            "Initializing modifier cache with TTL: {} and max entries per user: {}",
            default_ttl,
            max_entries_per_user
        );
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            default_ttl,
            max_entries_per_user,
        }
    }

    pub fn from_settings(settings: &CacheSettings) -> Self {
        trace!("Initializing modifier cache from settings");
        let default_ttl = chrono::Duration::seconds(settings.default_ttl as i64);
        ModifierCache::new(default_ttl, settings.max_user_entries)
    }

    /// Get a cached modifier value if it exists and is valid
    pub async fn get(&self, key: &CacheKey) -> Option<CacheEntry> {
        let cache = self.cache.read().await;
        let entry = cache.get(key)?;

        // Check if entry has expired
        if let Some(expires_at) = entry.expires_at {
            if expires_at <= Utc::now() {
                return None;
            }
        }

        Some(entry.clone())
    }

    /// Set a new cache entry with optional expiration
    pub async fn set(
        &self,
        key: CacheKey,
        total_multiplier: BigDecimal,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<(), Error> {
        let mut cache = self.cache.write().await;

        // Check player entry limit
        let user_entries = cache
            .keys()
            .filter(|k| k.player_id == key.player_id)
            .count();

        if user_entries >= self.max_entries_per_user {
            return Err(Error::new(
                ErrorKind::CacheError,
                "Cache limit exceeded for player",
            ));
        }

        let entry = CacheEntry {
            total_multiplier,
            expires_at,
            version: 0,
            last_updated: Utc::now(),
        };

        cache.insert(key, entry);
        Ok(())
    }

    /// Update an existing cache entry with optimistic locking
    pub async fn update(
        &self,
        key: &CacheKey,
        total_multiplier: BigDecimal,
        expires_at: Option<DateTime<Utc>>,
        expected_version: u64,
    ) -> Result<(), Error> {
        let mut cache = self.cache.write().await;

        if let Some(entry) = cache.get(key) {
            if entry.version != expected_version {
                return Err(Error::new(
                    ErrorKind::CacheError,
                    "Cache entry version mismatch",
                ));
            }

            let new_entry = CacheEntry {
                total_multiplier,
                expires_at,
                version: entry.version + 1,
                last_updated: Utc::now(),
            };

            cache.insert(key.clone(), new_entry);
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::CacheMissError,
                "Cache entry not found",
            ))
        }
    }

    /// Invalidate a specific cache entry
    pub async fn invalidate(&self, key: &CacheKey) {
        let mut cache = self.cache.write().await;
        cache.remove(key);
    }

    /// Invalidate all entries for a player
    pub async fn invalidate_user(&self, player_id: Uuid) {
        let mut cache = self.cache.write().await;
        cache.retain(|k, _| k.player_id != player_id);
    }

    /// Get the next expiration time for a player's modifiers
    pub async fn next_expiration(&self, player_id: Uuid) -> Option<DateTime<Utc>> {
        let cache = self.cache.read().await;

        cache
            .iter()
            .filter(|(k, _)| k.player_id == player_id)
            .filter_map(|(_, v)| v.expires_at)
            .min()
    }

    /// Clean up expired entries
    pub async fn cleanup(&self) {
        let mut cache = self.cache.write().await;
        let now = Utc::now();
        cache.retain(|_, entry| entry.expires_at.map(|exp| exp > now).unwrap_or(true));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_basic_operations() {
        let cache = ModifierCache::new(chrono::Duration::hours(1), 100);

        let key = CacheKey {
            player_id: Uuid::new_v4(),
            target_type: ModifierTarget::Resource,
            target_resource: Some(ResourceType::Gold),
        };

        // Test set and get
        cache
            .set(
                key.clone(),
                BigDecimal::from(1),
                Some(Utc::now() + chrono::Duration::hours(1)),
            )
            .await
            .unwrap();

        let entry = cache.get(&key).await.unwrap();
        assert_eq!(entry.total_multiplier, BigDecimal::from(1));
    }

    #[tokio::test]
    async fn test_version_conflict() {
        let cache = ModifierCache::new(chrono::Duration::hours(1), 100);
        let key = CacheKey {
            player_id: Uuid::new_v4(),
            target_type: ModifierTarget::Resource,
            target_resource: Some(ResourceType::Gold),
        };

        // Initial set
        cache
            .set(key.clone(), BigDecimal::from(1), None)
            .await
            .unwrap();

        // Try to update with wrong version
        let result = cache.update(&key, BigDecimal::from(2), None, 1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_expiration() {
        let cache = ModifierCache::new(chrono::Duration::hours(1), 100);
        let key = CacheKey {
            player_id: Uuid::new_v4(),
            target_type: ModifierTarget::Resource,
            target_resource: Some(ResourceType::Gold),
        };

        // Set with short expiration
        cache
            .set(
                key.clone(),
                BigDecimal::from(1),
                Some(Utc::now() - chrono::Duration::seconds(1)),
            )
            .await
            .unwrap();

        // Should return None for expired entry
        assert!(cache.get(&key).await.is_none());
    }

    #[tokio::test]
    async fn test_cleanup() {
        let cache = ModifierCache::new(chrono::Duration::hours(1), 100);
        let player_id = Uuid::new_v4();

        let key1 = CacheKey {
            player_id,
            target_type: ModifierTarget::Resource,
            target_resource: Some(ResourceType::Gold),
        };

        let key2 = CacheKey {
            player_id,
            target_type: ModifierTarget::Resource,
            target_resource: Some(ResourceType::Wood),
        };

        // Set one expired and one valid entry
        cache
            .set(
                key1.clone(),
                BigDecimal::from(1),
                Some(Utc::now() - chrono::Duration::seconds(1)),
            )
            .await
            .unwrap();

        cache
            .set(
                key2.clone(),
                BigDecimal::from(1),
                Some(Utc::now() + chrono::Duration::hours(1)),
            )
            .await
            .unwrap();

        // Run cleanup
        cache.cleanup().await;

        assert!(cache.get(&key1).await.is_none());
        assert!(cache.get(&key2).await.is_some());
    }

    #[tokio::test]
    async fn test_user_limit() {
        let cache = ModifierCache::new(chrono::Duration::hours(1), 2);
        let player_id = Uuid::new_v4();

        // Add up to limit
        for i in 0..2 {
            let key = CacheKey {
                player_id,
                target_type: ModifierTarget::Resource,
                target_resource: if i == 0 {
                    Some(ResourceType::Gold)
                } else {
                    Some(ResourceType::Stone)
                },
            };
            cache.set(key, BigDecimal::from(i), None).await.unwrap();
        }

        // Try to exceed limit
        let key = CacheKey {
            player_id,
            target_type: ModifierTarget::Resource,
            target_resource: Some(ResourceType::Wood),
        };

        let result = cache.set(key, BigDecimal::from(3), None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_next_expiration() {
        let cache = ModifierCache::new(chrono::Duration::hours(1), 100);
        let player_id = Uuid::new_v4();

        let now = Utc::now();
        let exp1 = now + chrono::Duration::hours(1);
        let exp2 = now + chrono::Duration::hours(2);

        let key1 = CacheKey {
            player_id,
            target_type: ModifierTarget::Resource,
            target_resource: Some(ResourceType::Gold),
        };

        let key2 = CacheKey {
            player_id,
            target_type: ModifierTarget::Resource,
            target_resource: Some(ResourceType::Wood),
        };

        cache
            .set(key1, BigDecimal::from(1), Some(exp1))
            .await
            .unwrap();
        cache
            .set(key2, BigDecimal::from(1), Some(exp2))
            .await
            .unwrap();

        let next_exp = cache.next_expiration(player_id).await.unwrap();
        assert_eq!(next_exp, exp1);
    }
}
