//! Player repository module that handles database operations for players.
//!
//! This module provides functionality to perform CRUD operations on player entities
//! in the database through the PlayerRepository struct.

use std::fmt;

use diesel::prelude::*;

use crate::db::{DbConn, DbPool, Repository};
use crate::domain::error::Result;
use crate::domain::player::{NewPlayer, Player, PlayerKey, UpdatePlayer};
use crate::schema::player::dsl::*;

/// Repository for managing player entities in the database.
///
/// Provides methods for creating, reading, updating and deleting player records,
/// as well as specialised queries for player-specific operations.
///
/// # Fields
/// * `connection` - Database connection pool
pub struct PlayerRepository {
    connection: DbConn,
}

impl fmt::Debug for PlayerRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PlayerRepository")
    }
}

/// Implementation of the Repository trait for Player entities.
///
/// Provides standard CRUD operations for Player entities using diesel.
impl Repository<Player, NewPlayer, &UpdatePlayer, PlayerKey> for PlayerRepository {
    fn try_from_pool(pool: &DbPool) -> Result<Self> {
        Ok(Self {
            connection: pool.get()?,
        })
    }

    fn from_connection(connection: DbConn) -> Self {
        Self { connection }
    }

    fn get_all(&mut self) -> Result<Vec<Player>> {
        let player_list = player
            .select(Player::as_select())
            .load(&mut self.connection)?;
        Ok(player_list)
    }

    fn get_by_id(&mut self, player_id: &PlayerKey) -> Result<Player> {
        let player_ = player.find(player_id).first(&mut self.connection)?;
        Ok(player_)
    }

    fn create(&mut self, entity: NewPlayer) -> Result<Player> {
        let player_ = diesel::insert_into(player)
            .values(entity)
            .returning(Player::as_returning())
            .get_result(&mut self.connection)?;
        Ok(player_)
    }

    fn update(&mut self, changeset: &UpdatePlayer) -> Result<Player> {
        let player_ = diesel::update(player)
            .set(changeset)
            .get_result(&mut self.connection)?;
        Ok(player_)
    }

    fn delete(&mut self, player_id: &PlayerKey) -> Result<usize> {
        let deleted_count = diesel::delete(player.find(player_id)).execute(&mut self.connection)?;
        Ok(deleted_count)
    }
}

impl PlayerRepository {
    /// Attempts to find a player by their ID.
    ///
    /// # Arguments
    /// * `player_id` - The unique identifier of the player
    ///
    /// # Returns
    /// * `Ok(Some(Player))` if the player is found
    /// * `Ok(None)` if no player exists with the given ID
    /// * `Err` if a database error occurs
    pub fn find_by_id(&mut self, player_id: &PlayerKey) -> Result<Option<Player>> {
        let player_: Option<Player> = player
            .find(player_id)
            .first(&mut self.connection)
            .optional()?;
        Ok(player_)
    }

    /// Retrieves a player by their name.
    ///
    /// # Arguments
    /// * `player_name` - The name of the player to find
    ///
    /// # Returns
    /// * `Ok(Player)` if the player is found
    /// * `Err` if no player exists with the given name or a database error occurs
    pub fn get_by_name(&mut self, player_name: impl AsRef<str>) -> Result<Player> {
        let player_ = player
            .filter(name.eq(player_name.as_ref()))
            .first(&mut self.connection)?;
        Ok(player_)
    }

    /// Attempts to find a player by their name.
    ///
    /// # Arguments
    /// * `player_name` - The name of the player to find
    ///
    /// # Returns
    /// * `Ok(Some(Player))` if the player is found
    /// * `Ok(None)` if no player exists with the given name
    /// * `Err` if a database error occurs
    pub fn find_by_name(&mut self, player_name: impl AsRef<str>) -> Result<Option<Player>> {
        let player_: Option<Player> = player
            .filter(name.eq(player_name.as_ref()))
            .first(&mut self.connection)
            .optional()?;
        Ok(player_)
    }

    /// Checks if a player with the given name exists.
    ///
    /// # Arguments
    /// * `player_name` - The name to check for existence
    ///
    /// # Returns
    /// * `Ok(true)` if a player with the given name exists
    /// * `Ok(false)` if no player exists with the given name
    /// * `Err` if a database error occurs
    pub fn exists_by_name(&mut self, player_name: impl AsRef<str>) -> Result<bool> {
        let player_ = self.find_by_name(player_name)?;
        Ok(player_.is_some())
    }
}
