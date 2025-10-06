use std::collections::HashMap;

use bigdecimal::BigDecimal;

use crate::domain::player::resource::ResourceType;

pub mod resource_operations;
pub mod resource_processor;
pub mod resource_scheduler;
pub mod resource_service;

/// A decimal multiplier for a resource type.
pub type ResourceMultiplier = BigDecimal;
/// A decimal production rate for a resource type.
pub type ResourceProductionRate = BigDecimal;

/// A mapping of resource types to their corresponding decimal modifier values.
pub type ResourceMultipliers = HashMap<ResourceType, ResourceMultiplier>;
/// A mapping of resource types to their corresponding decimal production rates.
pub type ResourceProductionRates = HashMap<ResourceType, ResourceProductionRate>;
