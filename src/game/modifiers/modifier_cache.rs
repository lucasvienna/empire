use std::collections::HashMap;

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::resource::ResourceType;

pub struct ModifierCache {
    cached_values: HashMap<(Uuid, ResourceType), CachedModifier>,
}

pub struct CachedModifier {
    total_multiplier: BigDecimal,
    expires_at: Option<DateTime<Utc>>,
    version: u64,
}
