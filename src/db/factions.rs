use std::fmt;
use std::sync::Arc;

use axum::extract::FromRef;
use diesel::prelude::*;

use crate::db::Repository;
use crate::domain::app_state::{AppPool, AppState};
use crate::domain::error::Result;
use crate::domain::factions::{Faction, FactionKey, NewFaction, UpdateFaction};
use crate::domain::modifier::Modifier;
use crate::schema::faction::dsl::*;

/// Repository for managing faction entities in the database.
///
/// This struct provides CRUD operations for [`Faction`] entities using a database connection.
///
/// # Fields
/// * `pool` - Thread-safe connection pool of type [`AppPool`] for database access
pub struct FactionRepository {
    pool: AppPool,
}

impl fmt::Debug for FactionRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FactionRepository")
    }
}

impl FromRef<AppState> for FactionRepository {
    fn from_ref(state: &AppState) -> Self {
        Self::new(&state.db_pool)
    }
}

impl Repository<Faction, NewFaction, &UpdateFaction, FactionKey> for FactionRepository {
    fn new(pool: &AppPool) -> Self {
        Self {
            pool: Arc::clone(pool),
        }
    }

    /// Retrieves all [`Faction`] entities from the database.
    ///
    /// # Returns
    /// A Result containing a vector of all faction entities
    fn get_all(&self) -> Result<Vec<Faction>> {
        let mut conn = self.pool.get()?;
        let fac_list = faction.select(Faction::as_select()).load(&mut conn)?;
        Ok(fac_list)
    }

    /// Retrieves a single [`Faction`] by its ID.
    ///
    /// # Arguments
    /// * `faction_id` - The unique identifier of the faction to retrieve
    ///
    /// # Returns
    /// A Result containing the requested faction
    fn get_by_id(&self, faction_id: &FactionKey) -> Result<Faction> {
        let mut conn = self.pool.get()?;
        let fac = faction.find(faction_id).first(&mut conn)?;
        Ok(fac)
    }

    /// Creates a new [`Faction`] in the database.
    ///
    /// # Arguments
    /// * `entity` - The new faction entity to create
    ///
    /// # Returns
    /// A Result containing the created faction
    fn create(&self, entity: NewFaction) -> Result<Faction> {
        let mut conn = self.pool.get()?;
        let created_faction = diesel::insert_into(faction)
            .values(entity)
            .returning(Faction::as_returning())
            .get_result(&mut conn)?;
        Ok(created_faction)
    }

    /// Updates an existing [`Faction`] in the database.
    ///
    /// # Arguments
    /// * `changeset` - The changes to apply to the faction
    ///
    /// # Returns
    /// A Result containing the updated faction
    fn update(&self, changeset: &UpdateFaction) -> Result<Faction> {
        let mut conn = self.pool.get()?;
        let updated_faction = diesel::update(faction)
            .set(changeset)
            .get_result(&mut conn)?;
        Ok(updated_faction)
    }

    /// Deletes a [`Faction`] from the database by its ID.
    ///
    /// # Arguments
    /// * `faction_id` - The unique identifier of the faction to delete
    ///
    /// # Returns
    /// A Result containing the number of rows deleted
    fn delete(&self, faction_id: &FactionKey) -> Result<usize> {
        let mut conn = self.pool.get()?;
        let rows_deleted = diesel::delete(faction.find(faction_id)).execute(&mut conn)?;
        Ok(rows_deleted)
    }
}

impl FactionRepository {
    /// Retrieves faction-specific bonuses (modifiers) from the database.
    ///
    /// # Arguments
    /// * `faction_key` - Optional faction identifier to filter bonuses for a specific faction
    pub fn get_bonuses(&self, faction_key: Option<&FactionKey>) -> Result<Vec<Modifier>> {
        let mut conn = self.pool.get()?;
        let bonuses = {
            use crate::schema::modifiers as md;
            let mut query = md::table
                .select(Modifier::as_select())
                .filter(md::stacking_group.similar_to("faction_%"))
                .into_boxed();

            if let Some(faction_key) = faction_key {
                let ilike = format!("{}_%", faction_key);
                query = query.filter(md::name.ilike(ilike));
            }

            query.get_results(&mut conn)?
        };

        Ok(bonuses)
    }
}
